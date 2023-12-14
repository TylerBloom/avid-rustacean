use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use avid_rustacean_model::*;
use futures::StreamExt;
use mongodb::{Collection, Database};
use tracing::error;

#[derive(Debug, Clone)]
pub struct AppState {
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
            error!("Failed to find/deserialze home page!!");
        }

        // Blog
        let table = self.get_blog_table();
        let mut posts = HashMap::new();
        let mut cursor = table.find(None, None).await.unwrap();
        while let Some(post) = cursor.next().await {
            match post {
                Ok(post) => {
                    posts.insert(post.summary.title.clone(), Arc::new(post));
                }
                Err(e) => {
                    error!("Failed to deserialize post!! Got error: {e}");
                }
            }
        }
        let mut sums: Vec<_> = posts.values().map(|p| p.summary.clone()).collect();
        sums.sort_by(|a, b| a.create_on.cmp(&b.create_on));
        *self.post_sums.write().unwrap() = Arc::new(sums);
        *self.posts.write().unwrap() = posts;

        // Projects
        let table = self.get_projects_table();
        let mut projects = HashMap::new();
        let mut cursor = table.find(None, None).await.unwrap();
        while let Some(project) = cursor.next().await {
            match project {
                Ok(project) => {
                    projects.insert(project.summary.name.clone(), Arc::new(project));
                }
                Err(e) => {
                    error!("Failed to deserialize project!! Got error: {e}");
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
