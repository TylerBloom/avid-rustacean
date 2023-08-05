use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An enum that encapsulates (un)finished blog posts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlogPost {
    InProgress(UnpublishedPost),
    Finished(PublishedPost),
}

/// A struct that acts as the lowest common denominator between (un)published blog posts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyPost {
    pub id: Uuid,
    title: String,
    body: String,
}

/// A struct that contains all information for an unpublished blog post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpublishedPost {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

/// A struct that contains all information for a published blog post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedPost {
    pub id: Uuid,
    published_date: DateTime<Utc>,
    title: String,
    body: String,
}

/// A summary of a blog post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSummary {
    id: Uuid,
    title: String,
    published_date: DateTime<Utc>,
}

/// The query type accepted by the backend's summary pager
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PagerQuery {
    pub page_size: u8,
    pub page_no: usize,
}

/// A struct that encapulates and the data and logic for paginated lists of blog post summaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryPager {
    pub query: PagerQuery,
    pub summaries: Vec<PostSummary>,
}

impl SummaryPager {
    /// Updates the pager by fetching the next page of summaries
    pub async fn next(&mut self) {
        self.query.next();
        self.refresh().await;
    }

    /// Updates the pager by fetching the previous page of summaries
    pub async fn prev(&mut self) {
        self.query.prev();
        self.refresh().await;
    }

    /// Updates the pager by setting the page number and fetching those summaries
    pub async fn set_page(&mut self, page_no: usize) {
        self.query.set_page(page_no);
        self.refresh().await;
    }

    /// Updates the pager by fetching performing the same query
    pub async fn refresh(&mut self) {
        // Make request to backend
        // Update self with returned data
        todo!()
    }
}

impl PagerQuery {
    /// Updates the query to the next page
    fn next(&mut self) {
        self.page_no = self.page_no.saturating_add(1);
    }

    /// Updates the query to the previous page
    fn prev(&mut self) {
        self.page_no = self.page_no.saturating_sub(1);
    }

    /// Updates the query by setting the page number
    fn set_page(&mut self, page_no: usize) {
        self.page_no = page_no;
    }
}

impl BlogPost {
    pub fn update(&mut self, title: String, body: String) {
        match self {
            BlogPost::InProgress(post) => post.update(title, body),
            BlogPost::Finished(post) => post.update(title, body),
        }
    }
}

impl PublishedPost {
    pub fn update(&mut self, title: String, body: String) {
        self.title = title;
        self.body = body;
    }
}

impl UnpublishedPost {
    pub fn publish(self) -> PublishedPost {
        let UnpublishedPost { id, title, body } = self;
        PublishedPost {
            id,
            title,
            body,
            published_date: Utc::now(),
        }
    }

    pub fn update(&mut self, title: String, body: String) {
        self.title = title;
        self.body = body;
    }
}

impl From<&PublishedPost> for PostSummary {
    fn from(post: &PublishedPost) -> Self {
        Self {
            id: post.id,
            title: post.title.clone(),
            published_date: post.published_date,
        }
    }
}

impl Default for PagerQuery {
    fn default() -> Self {
        Self {
            page_size: 20,
            page_no: 0,
        }
    }
}
