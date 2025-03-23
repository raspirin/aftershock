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
}

impl From<ContentKind> for String {
    fn from(value: ContentKind) -> Self {
        match value {
            ContentKind::Post => "post".into(),
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
        match kind.as_str() {
            "post" => Ok(Self::Post),
            otherwise => Err(format!("Unrecognized content type: {otherwise}").into()),
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
}

impl IntoPost for (Content, Vec<Tag>) {
    fn into_post(self) -> aftershock_bridge::Post {
        let (content, tags) = self;
        let tags = tags.into_iter().map(|tag| tag.into()).collect();
        aftershock_bridge::Post {
            uid: content.uid,
            created_at: content.created_at,
            updated_at: content.updated_at,
            title: content.title,
            body: content.body,
            tags,
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
}

impl<'a> NewContent<'a> {
    pub fn new(kind: ContentKind, title: &'a str, body: &'a str, published: bool) -> Self {
        let created_at = utils::now();
        let kind = kind.into();
        let uid = utils::Nid::new().to_string();

        Self {
            kind,
            created_at,
            updated_at: created_at,
            title,
            body,
            published,
            uid,
        }
    }
}

impl<'a> From<&'a aftershock_bridge::NewPost> for NewContent<'a> {
    fn from(value: &'a aftershock_bridge::NewPost) -> Self {
        Self::new(
            ContentKind::Post,
            &value.title,
            &value.body,
            value.published,
        )
    }
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::contents)]
pub struct UpdateContent {
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub published: Option<bool>,
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
