use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    prelude::*,
    sql_types::Text,
};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(FromSqlRow, Serialize, Deserialize)]
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

#[derive(Queryable, Selectable, Serialize, Deserialize)]
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

impl From<Content> for aftershock_bridge::Post {
    fn from(value: Content) -> Self {
        Self {
            uid: value.uid,
            created_at: value.created_at,
            updated_at: value.updated_at,
            title: value.title,
            body: value.body,
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
