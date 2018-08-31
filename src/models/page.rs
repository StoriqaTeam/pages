use schema::pages;

use std::time::SystemTime;
use stq_api::pages::*;
use stq_types::{PageId, PageSlug};

#[derive(From, Into, Queryable, Insertable, Identifiable)]
#[table_name = "pages"]
pub struct DbPage {
    pub id: PageId,
    pub slug: PageSlug,
    pub html: String,
    pub css: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl From<DbPage> for Page {
    fn from(v: DbPage) -> Self {
        Self {
            id: v.id,
            slug: v.slug,
            html: v.html,
            css: v.css,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

#[derive(Insertable)]
#[table_name = "pages"]
pub struct DbNewPage {
    pub id: PageId,
    pub slug: PageSlug,
    pub html: String,
    pub css: String,
}

impl From<NewPage> for DbNewPage {
    fn from(v: NewPage) -> Self {
        Self {
            id: v.id,
            slug: v.slug.0.to_lowercase().into(),
            html: v.html,
            css: v.css,
        }
    }
}
