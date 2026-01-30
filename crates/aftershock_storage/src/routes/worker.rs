use diesel::{prelude::*, r2d2::ConnectionManager, sql_types::Bool, sqlite::Sqlite};
use r2d2::PooledConnection;

use crate::{
    Result,
    models::{Content, ContentTag, IntoPost, Tag, UpdateContent},
    schema::{self},
};

pub enum TargetKind {
    Post,
    Page,
}

#[derive(PartialEq, Eq)]
pub enum ContentKind {
    Meta,
    Content,
}

pub enum PublishState {
    Published,
    All,
}

pub enum Filter {
    All,
    Id(String),
    Name(String),
    Tag(String),
}

pub enum Action {
    Create(aftershock_bridge::NewPost),
    Update(UpdateContent),
    Delete,
    Query,
}

type BorrowedConnection<'c> = &'c mut PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct Worker<'c> {
    conn: BorrowedConnection<'c>,
    action: Box<dyn FnOnce(BorrowedConnection<'c>) -> Result<Vec<aftershock_bridge::Post>>>,
}

impl Worker<'_> {
    pub fn builder() -> WorkerBuilder {
        WorkerBuilder::new()
    }

    pub fn load<T>(self) -> Result<Vec<T>>
    where
        T: From<aftershock_bridge::Post>,
    {
        let ret = (self.action)(self.conn)?;

        Ok(ret.into_iter().map(|x| x.into()).collect())
    }
}

pub struct WorkerBuilder {
    target_kind: Option<TargetKind>,
    publish_state: Option<PublishState>,
    filter: Option<Filter>,
    action: Option<Action>,
}

impl Default for WorkerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkerBuilder {
    pub fn new() -> WorkerBuilder {
        Self {
            target_kind: None,
            publish_state: Some(PublishState::All),
            filter: Some(Filter::All),
            action: None,
        }
    }

    pub fn build<'c>(self, conn: BorrowedConnection<'c>) -> Option<Worker<'c>> {
        use crate::schema;

        let action: Box<
            dyn FnOnce(BorrowedConnection<'c>) -> Result<Vec<aftershock_bridge::Post>>,
        > = match self.action? {
            Action::Create(post) => Box::new(move |c| {
                use crate::schema::{contents, contents_tags, tags};

                let new_content: crate::models::NewContent = (&post).into();

                let content = diesel::insert_into(contents::table)
                    .values(&new_content)
                    .returning(Content::as_returning())
                    .get_result(&mut *c)?;

                let new_tags: Vec<crate::models::NewTag<'_>> =
                    post.tags.iter().map(|x| x.into()).collect();

                let tags = c.transaction(|conn| {
                    for new_tag in new_tags {
                        let _ = diesel::insert_into(tags::table)
                            .values(&new_tag)
                            .on_conflict_do_nothing()
                            .execute(conn)?;
                    }
                    tags::table
                        .filter(tags::tag.eq_any(&post.tags))
                        .select(Tag::as_returning())
                        .get_results(conn)
                })?;

                let ct: Vec<ContentTag> =
                    tags.iter().map(|tag| (content.id, tag.id).into()).collect();
                diesel::insert_into(contents_tags::table)
                    .values(&ct)
                    .execute(&mut *c)?;

                Ok(vec![(content, tags).into_post()])
            }),
            action => {
                let query = schema::contents::table
                    .filter(Self::filter_by_target_kind(self.target_kind?))
                    .filter(Self::filter_by_filter(self.filter?))
                    .filter(Self::filter_by_publish_state(self.publish_state?));
                match action {
                    Action::Query => Box::new(|c| {
                        let contents = query.select(Content::as_select()).load(&mut *c)?;

                        let tags = Self::get_tags_from_contents(&contents)(&mut *c)?;

                        let ret = Self::combine_content_tags(contents, tags);

                        Ok(ret)
                    }),
                    Action::Update(mut update_content) => Box::new(|c| {
                        let now = crate::utils::now();
                        update_content.updated_at = Some(now);
                        if update_content.published.is_some_and(|x| x) {
                            update_content.created_at = Some(now);
                        }

                        let query = diesel::update(query)
                            .set(update_content)
                            .returning(Content::as_returning())
                            .get_results(&mut *c)?;

                        let tags = Self::get_tags_from_contents(&query)(&mut *c)?;

                        //TODO: update tags

                        let ret = Self::combine_content_tags(query, tags);

                        Ok(ret)
                    }),
                    Action::Delete => Box::new(|c| {
                        use crate::schema::contents_tags;

                        let (content, tags) =
                            c.transaction::<_, crate::error::Error, _>(|conn| {
                                let content = diesel::delete(query)
                                    .returning(Content::as_returning())
                                    .get_results(conn)?;

                                let tags = Self::get_tags_from_contents(&content)(conn)?;

                                if !content.is_empty() {
                                    diesel::delete(
                                        contents_tags::table.filter(
                                            contents_tags::content_id
                                                .eq_any(content.iter().map(|x| x.id)),
                                        ),
                                    )
                                    .execute(conn)?;
                                }

                                Ok((content, tags))
                            })?;

                        let ret = Self::combine_content_tags(content, tags);

                        Ok(ret)
                    }),
                    Action::Create(_) => unreachable!(),
                }
            }
        };

        Some(Worker { conn, action })
    }

    pub fn post(mut self) -> Self {
        self.target_kind = Some(TargetKind::Post);
        self
    }

    pub fn page(mut self) -> Self {
        self.target_kind = Some(TargetKind::Page);
        self
    }

    pub fn by_id(mut self, id: String) -> Self {
        self.filter = Some(Filter::Id(id));
        self
    }

    pub fn by_name(mut self, name: String) -> Self {
        self.filter = Some(Filter::Name(name));
        self
    }

    pub fn by_tag(mut self, tag: String) -> Self {
        self.filter = Some(Filter::Tag(tag));
        self
    }

    pub fn published_only(mut self) -> Self {
        self.publish_state = Some(PublishState::Published);
        self
    }

    pub fn query(mut self) -> Self {
        self.action = Some(Action::Query);
        self
    }

    pub fn create(mut self, content: aftershock_bridge::NewPost) -> Self {
        self.action = Some(Action::Create(content));
        self
    }

    pub fn delete(mut self) -> Self {
        self.action = Some(Action::Delete);
        self
    }

    pub fn update(mut self, content: UpdateContent) -> Self {
        self.action = Some(Action::Update(content));
        self
    }
}

type WorkerBuilderInnerFilter =
    Box<dyn BoxableExpression<crate::schema::contents::table, Sqlite, SqlType = Bool>>;

// Inner implementation of the WorkerBuilder
impl WorkerBuilder {
    #[diesel::dsl::auto_type(no_type_alias)]
    fn filter_by_target_kind(target_kind: TargetKind) -> _ {
        use crate::schema;

        let keyword: &'static str = match target_kind {
            TargetKind::Post => "post",
            TargetKind::Page => "page",
        };

        schema::contents::kind.eq(keyword)
    }

    fn filter_by_filter(filter: Filter) -> WorkerBuilderInnerFilter {
        use crate::schema;

        match filter {
            Filter::All => Box::new(schema::contents::title.is_not_null()),
            Filter::Id(id) => Box::new(schema::contents::uid.eq(id)),
            Filter::Name(name) => Box::new(schema::contents::title.eq(name)),
            Filter::Tag(tag) => Box::new(
                schema::contents::id.eq_any(
                    schema::contents_tags::table
                        .inner_join(schema::tags::table)
                        .filter(schema::tags::tag.eq(tag))
                        .select(schema::contents_tags::content_id),
                ),
            ),
        }
    }

    fn filter_by_publish_state(publish_state: PublishState) -> WorkerBuilderInnerFilter {
        use crate::schema;

        match publish_state {
            PublishState::Published => Box::new(schema::contents::published.eq(true)),
            PublishState::All => Box::new(schema::contents::published.is_not_null()),
        }
    }

    fn get_tags_from_contents<'a>(
        contents: &'a [Content],
    ) -> impl FnOnce(BorrowedConnection<'a>) -> Result<Vec<Vec<Tag>>> {
        use crate::schema::tags;

        move |conn| {
            let tags: Vec<(ContentTag, Tag)> = ContentTag::belonging_to(contents)
                .inner_join(tags::table)
                .select((ContentTag::as_select(), Tag::as_select()))
                .load(conn)?;

            let ret = tags
                .grouped_by(contents)
                .into_iter()
                .map(|ts| ts.into_iter().map(|t| t.1).collect())
                .collect();

            Ok(ret)
        }
    }

    fn combine_content_tags(
        contents: Vec<Content>,
        tags: Vec<Vec<Tag>>,
    ) -> Vec<aftershock_bridge::Post> {
        contents
            .into_iter()
            .zip(tags)
            .map(|x| x.into_post())
            .collect()
    }
}
