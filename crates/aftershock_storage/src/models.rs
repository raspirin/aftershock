use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    prelude::*,
    sql_types::Text,
};

#[derive(FromSqlRow)]
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

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::contents, check_for_backend(diesel::sqlite::Sqlite))]
pub struct Content {
    pub id: i32,
    pub kind: ContentKind,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::contents, check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewContent<'a> {
    pub kind: String,
    created_at: i64,
    updated_at: i64,
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> NewContent<'a> {
    pub fn new(kind: ContentKind, title: &'a str, body: &'a str) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("System time error! Do we have a time machine?")
            .as_secs() as i64;
        let kind = kind.into();

        Self {
            kind,
            created_at,
            updated_at: created_at,
            title,
            body,
        }
    }
}
