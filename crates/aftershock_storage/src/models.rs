use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    prelude::*,
    sql_types::Text,
};
use serde::{Deserialize, Serialize};

use crate::utils;

pub trait IntoPost {
    fn into_post(self) -> aftershock_bridge::Post;
}

#[derive(FromSqlRow, PartialEq, Serialize, Deserialize, Debug)]
pub enum ContentKind {
    Post,
    Page,
}

impl From<ContentKind> for String {
    fn from(value: ContentKind) -> Self {
        match value {
            ContentKind::Post => "post".into(),
            ContentKind::Page => "page".into(),
        }
    }
}

impl<DB> FromSql<Text, DB> for ContentKind
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let kind = String::from_sql(bytes)?;
        match ContentKind::try_from(kind.as_str()) {
            Ok(kind) => Ok(kind),
            Err(_) => Err(format!("Unrecognized content kind {kind}").into()),
        }
    }
}

impl TryFrom<&str> for ContentKind {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "post" => Ok(Self::Post),
            "page" => Ok(Self::Page),
            _ => Err(crate::error::Error::ContentKindError),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::contents, check_for_backend(diesel::sqlite::Sqlite))]
pub struct Content {
    pub id: i32,
    pub kind: ContentKind,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub uid: String,
    pub summary: Option<String>,
}

impl IntoPost for (Content, Vec<Tag>) {
    fn into_post(self) -> aftershock_bridge::Post {
        let (content, tags) = self;
        let tags = tags.into_iter().map(|tag| tag.into()).collect();
        aftershock_bridge::Post {
            uid: content.uid,
            kind: content.kind.into(),
            created_at: content.created_at,
            updated_at: content.updated_at,
            title: content.title,
            body: content.body,
            tags,
            summary: content.summary,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::contents, check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewContent<'a> {
    pub kind: String,
    created_at: i64,
    updated_at: i64,
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
    pub uid: String,
    pub summary: Option<String>,
}

impl<'a> NewContent<'a> {
    pub fn new(
        kind: ContentKind,
        title: &'a str,
        body: &'a str,
        published: bool,
        summary: Option<String>,
    ) -> Self {
        let created_at = utils::now();
        let uid = match kind {
            ContentKind::Page => title.to_lowercase(),
            _ => utils::Nid::new().to_string(),
        };
        let kind = kind.into();

        Self {
            kind,
            created_at,
            updated_at: created_at,
            title,
            body,
            published,
            uid,
            summary,
        }
    }
}

impl<'a> From<&'a aftershock_bridge::NewPost> for NewContent<'a> {
    fn from(value: &'a aftershock_bridge::NewPost) -> Self {
        Self::new(
            ContentKind::try_from(value.kind.as_str()).unwrap(),
            &value.title,
            &value.body,
            value.published,
            value.summary.clone(),
        )
    }
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::contents)]
pub struct UpdateContent {
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub published: Option<bool>,
}

impl From<aftershock_bridge::UpdatePost> for UpdateContent {
    fn from(value: aftershock_bridge::UpdatePost) -> Self {
        Self {
            created_at: None,
            updated_at: None,
            title: value.title,
            body: value.body,
            published: value.published,
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::tags, check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}

impl From<Tag> for String {
    fn from(value: Tag) -> Self {
        value.tag
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::tags, check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTag<'a> {
    pub tag: &'a str,
}

impl<'a> From<&'a String> for NewTag<'a> {
    fn from(value: &'a String) -> Self {
        Self {
            tag: value.as_str(),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, PartialEq, Debug)]
#[diesel(table_name = crate::schema::contents_tags, check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Content, foreign_key = content_id))]
#[diesel(belongs_to(Tag, foreign_key = tag_id))]
#[diesel(primary_key(content_id, tag_id))]
pub struct ContentTag {
    pub content_id: i32,
    pub tag_id: i32,
}

impl From<(i32, i32)> for ContentTag {
    fn from(value: (i32, i32)) -> Self {
        let (content_id, tag_id) = value;
        Self { content_id, tag_id }
    }
}
