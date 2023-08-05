use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use avid_rustacean_model::blog::{
    BlogPost, PagerQuery, PostSummary, PublishedPost, SummaryPager, UnpublishedPost,
};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, spec::BinarySubtype, Binary, Document},
    options::UpdateOptions,
    Collection, Database,
};
use serde::Serialize;
use uuid::Uuid;

use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot::{channel as oneshot_channel, Receiver as OneshotReceiver, Sender as OneshotSender},
};

const UNPUBLISHED_COLL: &str = "unpublished";
const PUBLISHED_COLL: &str = "published";

#[derive(Debug, Clone)]
pub struct AppState {
    handle: UnboundedSender<PostCommand>,
}

impl AppState {
    pub async fn new(db: Database) -> Self {
        let (send, recv) = unbounded_channel();
        let cache = CacheHandle::new(db, recv).await;
        // Dropping the handle because the cache's future will never return
        drop(tokio::spawn(cache.run()));
        Self { handle: send }
    }

    pub fn get_pager(&self, query: PagerQuery) -> Tracker<SummaryPager> {
        let (send, recv) = oneshot_channel();
        self.handle.send(PostCommand::Pager(query, send)).unwrap();
        Tracker { recv }
    }

    pub fn create_post(&self, post: (String, String)) -> Tracker<Uuid> {
        let (send, recv) = oneshot_channel();
        self.handle.send(PostCommand::Create(post, send)).unwrap();
        Tracker { recv }
    }

    pub fn update_post(&self, p_id: Uuid, post: (String, String)) -> Tracker<bool> {
        let (send, recv) = oneshot_channel();
        self.handle
            .send(PostCommand::Update(p_id, post, send))
            .unwrap();
        Tracker { recv }
    }

    pub fn publish_post(&self, p_id: Uuid) -> Tracker<bool> {
        let (send, recv) = oneshot_channel();
        self.handle.send(PostCommand::Publish(p_id, send)).unwrap();
        Tracker { recv }
    }

    pub fn get_post(&self, p_id: Uuid) -> Tracker<Option<BlogPost>> {
        let (send, recv) = oneshot_channel();
        self.handle.send(PostCommand::Get(p_id, send)).unwrap();
        Tracker { recv }
    }
}

pub struct Tracker<T> {
    recv: OneshotReceiver<T>,
}

impl<T> Future for Tracker<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.as_mut().recv)
            .poll(cx)
            // We unwrap the option here because the actor won't drop the sender half
            .map(Result::unwrap)
    }
}

enum PostCommand {
    Get(Uuid, OneshotSender<Option<BlogPost>>),
    Create((String, String), OneshotSender<Uuid>),
    Update(Uuid, (String, String), OneshotSender<bool>),
    Publish(Uuid, OneshotSender<bool>),
    Pager(PagerQuery, OneshotSender<SummaryPager>),
}

struct CacheHandle {
    inbound: UnboundedReceiver<PostCommand>,
    db: Database,
    cache: Cache,
}

struct Cache {
    summaries: Vec<PostSummary>,
    published: HashMap<Uuid, PublishedPost>,
    unpublished: HashMap<Uuid, UnpublishedPost>,
}

impl CacheHandle {
    async fn new(db: Database, inbound: UnboundedReceiver<PostCommand>) -> Self {
        let cache = Cache::new_and_init(&db).await;
        Self { inbound, db, cache }
    }

    async fn run(mut self) -> ! {
        loop {
            tokio::select! {
                msg = self.inbound.recv() => {
                    self.process_command(msg.unwrap());
                }
            }
        }
    }

    fn process_command(&mut self, msg: PostCommand) {
        match msg {
            PostCommand::Get(p_id, send) => {
                let _ = send.send(self.get_post(p_id));
            }
            PostCommand::Create(post, send) => {
                let _ = send.send(self.create_post(post));
            }
            PostCommand::Update(p_id, post, send) => {
                let _ = send.send(self.update_post(p_id, post));
            }
            PostCommand::Publish(p_id, send) => {
                let _ = send.send(self.publish_post(p_id));
            }
            PostCommand::Pager(query, send) => {
                let _ = send.send(self.get_pager(query));
            }
        }
    }

    fn get_post(&self, p_id: Uuid) -> Option<BlogPost> {
        self.cache.get_post(p_id)
    }

    fn create_post(&mut self, (title, body): (String, String)) -> Uuid {
        let post = self.cache.create_post(title, body);
        let digest = post.id;
        // Dropping handle because it returns nothing and is unneeded
        drop(tokio::spawn(persist_blog_post(
            self.db.clone(),
            BlogPost::InProgress(post),
        )));
        digest
    }

    fn update_post(&mut self, p_id: Uuid, (title, body): (String, String)) -> bool {
        let Some(post) = self.cache.update_post(p_id, title, body) else {
            return false;
        };
        // Dropping handle because it returns nothing and is unneeded
        drop(tokio::spawn(persist_blog_post(self.db.clone(), post)));
        true
    }

    fn publish_post(&mut self, p_id: Uuid) -> bool {
        let digest = self.cache.publish_post(p_id);
        if digest {
            // Dropping the handle since nothing is returned
            drop(tokio::spawn(persist_publication(self.db.clone(), p_id)));
        }
        digest
    }

    fn get_pager(&self, query: PagerQuery) -> SummaryPager {
        self.cache.get_pager(query)
    }
}

impl Cache {
    async fn new_and_init(db: &Database) -> Self {
        // Fetch all published and unpublished posts
        // Construct summaries
        let coll = db.collection::<PublishedPost>(PUBLISHED_COLL);
        let mut published = HashMap::new();
        let mut summaries = Vec::new();
        if let Ok(mut cursor) = coll.find(None, None).await {
            while let Some(Ok(post)) = cursor.next().await {
                summaries.push(PostSummary::from(&post));
                published.insert(post.id, post);
            }
        }
        let coll = db.collection::<UnpublishedPost>(UNPUBLISHED_COLL);
        let mut unpublished = HashMap::new();
        if let Ok(mut cursor) = coll.find(None, None).await {
            while let Some(Ok(post)) = cursor.next().await {
                unpublished.insert(post.id, post);
            }
        }
        Self {
            summaries,
            published,
            unpublished,
        }
    }

    fn get_post(&self, p_id: Uuid) -> Option<BlogPost> {
        self.published
            .get(&p_id)
            .cloned()
            .map(BlogPost::Finished)
            .or_else(|| {
                self.unpublished
                    .get(&p_id)
                    .cloned()
                    .map(BlogPost::InProgress)
            })
    }

    fn create_post(&mut self, title: String, body: String) -> UnpublishedPost {
        let id = Uuid::new_v4();
        let post = UnpublishedPost { id, title, body };
        self.unpublished.insert(id, post.clone());
        post
    }

    fn update_post(&mut self, p_id: Uuid, title: String, body: String) -> Option<BlogPost> {
        if let Some(post) = self.unpublished.get_mut(&p_id) {
            post.update(title, body);
            return Some(BlogPost::InProgress(post.clone()));
        }
        if let Some(post) = self.published.get_mut(&p_id) {
            post.update(title, body);
            return Some(BlogPost::Finished(post.clone()));
        }
        None
    }

    fn publish_post(&mut self, p_id: Uuid) -> bool {
        let Some(post) = self.unpublished.remove(&p_id) else {
            return false;
        };
        self.published.insert(p_id, post.publish());
        true
    }

    fn get_pager(&self, query: PagerQuery) -> SummaryPager {
        let summaries = self
            .summaries
            .iter()
            .rev()
            .skip(query.page_no * query.page_size as usize)
            .take(query.page_size as usize)
            .cloned()
            .collect();
        SummaryPager { query, summaries }
    }
}

async fn persist_blog_post(db: Database, post: BlogPost) {
    // Find post in database and update
    match post {
        BlogPost::InProgress(post) => {
            let coll = db.collection::<UnpublishedPost>(UNPUBLISHED_COLL);
            update_or_insert(coll, post.id, post).await;
        }
        BlogPost::Finished(post) => {
            let coll = db.collection::<PublishedPost>(PUBLISHED_COLL);
            update_or_insert(coll, post.id, post).await;
        }
    }
}

async fn persist_publication(db: Database, p_id: Uuid) {
    if let Ok(Some(post)) = db
        .collection::<UnpublishedPost>(UNPUBLISHED_COLL)
        .find_one_and_delete(make_query(p_id), None)
        .await
    {
        let post = post.publish();
        let _ = db
            .collection::<PublishedPost>(PUBLISHED_COLL)
            .insert_one(&post, None)
            .await;
    }
}

async fn update_or_insert<T>(coll: Collection<T>, id: Uuid, data: T)
where
    T: Serialize,
{
    let doc: Document = mongodb::bson::to_raw_document_buf(&data)
        .unwrap()
        .try_into()
        .unwrap();
    let _ = coll
        .update_one(
            make_query(id),
            doc! {"$set": doc},
            UpdateOptions::builder().upsert(true).build(),
        )
        .await;
}

fn make_query(p_id: Uuid) -> Document {
    doc! { "id": Binary {
        bytes: p_id.as_bytes().to_vec(),
        subtype: BinarySubtype::Generic,
    }}
}
