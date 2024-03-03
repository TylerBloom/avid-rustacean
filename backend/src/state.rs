use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock},
};

use avid_rustacean_model::*;
use chrono::DateTime;
use futures::StreamExt;
use mongodb::{bson::Document, Collection, Database};
use serde::Deserialize;
use tracing::{error, warn};

use crate::rss::RssManager;

/// A struct used to store post data in files. This is used to store a post's data in a TOML file
/// so that it can be diff by git. The AppState uses this model as its source of truth on start up.
/// Doing so allows corrections by readers to be made via a PR and incorporated without needing to
/// go through the post creation API.
#[derive(Debug, Deserialize)]
struct FileData {
    title: String,
    summary: String,
    body: String,
    created: String,
}

impl FileData {
    fn into_post(self) -> Post {
        let body = self.body.parse().unwrap();
        let summary = self.summary.parse().unwrap();
        let summary = PostSummary {
            title: self.title,
            summary,
            create_on: DateTime::parse_from_rfc3339(&self.created).unwrap().into(),
            last_edit: None,
        };
        Post { summary, body }
    }

    fn into_project(self) -> Project {
        let body = self.body.parse().unwrap();
        let summary = self.summary.parse().unwrap();
        let summary = ProjectSummary {
            name: self.title,
            summary,
        };
        Project { summary, body }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub rss: Arc<RwLock<RssManager>>,
    home: Arc<RwLock<Arc<HomePage>>>,
    posts: Arc<RwLock<HashMap<String, Arc<Post>>>>,
    post_sums: Arc<RwLock<Arc<Vec<PostSummary>>>>,
    projects: Arc<RwLock<HashMap<String, Arc<Project>>>>,
    proj_sums: Arc<RwLock<Arc<Vec<ProjectSummary>>>>,
    db: Database,
}

impl AppState {
    const HOME_TABLE_NAME: &'static str = "Home";
    const PROJECT_TABLE_NAME: &'static str = "Projects";
    const BLOG_TABLE_NAME: &'static str = "Blog";

    pub fn new(db: Database) -> Self {
        Self {
            home: Arc::new(RwLock::new(Arc::new(HomePage::default()))),
            posts: Arc::new(RwLock::new(HashMap::new())),
            projects: Arc::new(RwLock::new(HashMap::new())),
            post_sums: Arc::new(RwLock::new(Arc::new(Vec::new()))),
            proj_sums: Arc::new(RwLock::new(Arc::new(Vec::new()))),
            rss: Arc::new(RwLock::new(RssManager::new())),
            db,
        }
    }

    /* --------- Database methods --------- */
    /// Loads data from the database on startup.
    pub async fn load(&self) {
        // Home page
        let table = self.get_home_table();
        if let Some(home) = table.find_one(None, None).await.unwrap() {
            *self.home.write().unwrap() = Arc::new(home);
        } else {
            warn!("Failed to find/deserialze home page!!");
            *self.home.write().unwrap() = Arc::new(HomePage::default());
        }

        // Blog
        let file_posts: HashMap<String, FileData> =
            toml::from_str(include_str!("../../content/posts.toml")).unwrap();

        let mut posts = HashMap::with_capacity(file_posts.len());
        for (name, post) in file_posts {
            posts.insert(name, Arc::new(post.into_post()));
        }

        let table = self.get_blog_table();
        let mut cursor = table.find(None, None).await.unwrap();
        while let Some(post) = cursor.next().await {
            match post {
                Ok(post) => match posts.get(&post.summary.title) {
                    Some(p) if p.as_ref() != &post => {
                        let doc: Document = mongodb::bson::to_raw_document_buf(&post)
                            .unwrap()
                            .try_into()
                            .unwrap();
                        let _ = table.delete_one(doc, None).await;
                        let _ = table.insert_one(&post, None).await;
                    }
                    Some(_) => {}
                    None => {
                        posts.insert(post.summary.title.clone(), Arc::new(post));
                    }
                },
                Err(e) => {
                    error!("Failed to deserialize post!! Got error: {e}");
                }
            }
        }

        let mut sums: Vec<_> = posts.values().map(|p| p.summary.clone()).collect();
        sums.sort_by(|a, b| a.create_on.cmp(&b.create_on));
        self.rss.write().unwrap().load(
            sums.iter()
                .rev()
                .filter_map(|sum| posts.get(&sum.title).map(Deref::deref)),
        );
        *self.post_sums.write().unwrap() = Arc::new(sums);
        *self.posts.write().unwrap() = posts;

        // Projects
        let file_projects: HashMap<String, FileData> =
            toml::from_str(include_str!("../../content/projects.toml")).unwrap();

        let mut projects = HashMap::with_capacity(file_projects.len());
        for (name, post) in file_projects {
            projects.insert(name, Arc::new(post.into_project()));
        }

        let table = self.get_projects_table();
        let mut cursor = table.find(None, None).await.unwrap();
        while let Some(project) = cursor.next().await {
            match project {
                Ok(project) => match projects.get(&project.summary.name) {
                    Some(p) if p.as_ref() != &project => {
                        let doc: Document = mongodb::bson::to_raw_document_buf(&project)
                            .unwrap()
                            .try_into()
                            .unwrap();
                        let _ = table.delete_one(doc, None).await;
                        let _ = table.insert_one(&project, None).await;
                    }
                    Some(_) => {}
                    None => {
                        projects.insert(project.summary.name.clone(), Arc::new(project));
                    }
                },
                Err(e) => {
                    error!("Failed to deserialize post!! Got error: {e}");
                }
            }
        }
        *self.proj_sums.write().unwrap() =
            Arc::new(projects.values().map(|p| p.summary.clone()).collect());
        *self.projects.write().unwrap() = projects;
    }

    pub fn get_home_table(&self) -> Collection<HomePage> {
        self.db.collection::<HomePage>(Self::HOME_TABLE_NAME)
    }

    pub fn get_blog_table(&self) -> Collection<Post> {
        self.db.collection::<Post>(Self::BLOG_TABLE_NAME)
    }

    pub fn get_projects_table(&self) -> Collection<Project> {
        self.db.collection::<Project>(Self::PROJECT_TABLE_NAME)
    }

    /* --------- Homepage methods --------- */
    /// Returns the homepage data
    pub fn get_homepage(&self) -> Arc<HomePage> {
        Arc::clone(&self.home.read().unwrap())
    }

    /// Overwrites the homepage data
    pub async fn update_homepage(&self, body: Markdown) {
        let table = self.get_home_table();
        let _ = table.drop(None).await;
        let home = HomePage { body };
        let _ = table.insert_one(&home, None).await;
        *self.home.write().unwrap() = Arc::new(home);
    }

    /* --------- Blog methods --------- */
    /// Attempts to create a post and returns if it already exists.
    pub async fn create_post(&self, post: Post) {
        let table = self.get_blog_table();
        let _ = table.insert_one(&post, None).await;
        if self
            .posts
            .write()
            .unwrap()
            .insert(post.summary.title.clone(), Arc::new(post.clone()))
            .is_none()
        {
            self.rss.write().unwrap().add_post(&post);
            let mut lock = self.post_sums.write().unwrap();
            let mut sums = Vec::clone(&lock);
            sums.push(post.summary.clone());
            *lock = Arc::new(sums);
        }
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_post(&self, title: &str) -> Option<Arc<Post>> {
        self.posts.read().unwrap().get(title).cloned()
    }

    /// Attempts to delete a post
    pub async fn delete_post(&self, title: &str) -> bool {
        let Some(post) = self.posts.write().unwrap().remove(title) else {
            return false;
        };
        let doc: Document = mongodb::bson::to_raw_document_buf(&post)
            .unwrap()
            .try_into()
            .unwrap();
        let _ = self.get_blog_table().delete_one(doc, None).await;
        let mut lock = self.post_sums.write().unwrap();
        let mut sums = Vec::clone(&lock);
        sums.retain(|s| s.title != title);
        *lock = Arc::new(sums);
        true
    }

    pub fn get_post_summaries(&self) -> Arc<Vec<PostSummary>> {
        Arc::clone(&self.post_sums.read().unwrap())
    }

    /* --------- Project methods --------- */
    /// Attempts to create a post and returns if it already exists.
    pub async fn create_project(&self, project: Project) {
        let table = self.get_projects_table();
        let _ = table.insert_one(&project, None).await;
        if self
            .projects
            .write()
            .unwrap()
            .insert(project.summary.name.clone(), Arc::new(project.clone()))
            .is_none()
        {
            let mut lock = self.proj_sums.write().unwrap();
            let mut sums = Vec::clone(&lock);
            sums.push(project.summary.clone());
            *lock = Arc::new(sums);
        }
    }

    /// Attempts to retrieve a post from the app state.
    pub fn get_project(&self, name: &str) -> Option<Arc<Project>> {
        self.projects.read().unwrap().get(name).cloned()
    }

    pub fn get_project_summaries(&self) -> Arc<Vec<ProjectSummary>> {
        Arc::clone(&self.proj_sums.read().unwrap())
    }
}
