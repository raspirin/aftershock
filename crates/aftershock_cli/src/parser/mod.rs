use std::sync::LazyLock;

use highlighter::Highlighter;
use pulldown_cmark::{Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use two_face::theme::EmbeddedThemeName;

mod highlighter;

static OPTIONS: LazyLock<Options> = LazyLock::new(get_options);

#[derive(Debug)]
pub struct ParserOutput {
    pub metadata: ParserOutputMetadata,
    pub html: String,
}

impl ParserOutput {
    pub fn new(metadata: ParserOutputMetadata, html: String) -> Self {
        Self { metadata, html }
    }
}

impl From<ParserOutput> for aftershock_bridge::NewPost {
    fn from(value: ParserOutput) -> Self {
        Self {
            title: value.metadata.title,
            kind: value.metadata.kind,
            body: value.html,
            tags: value.metadata.tags,
            published: false,
            summary: value.metadata.summary,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParserOutputMetadata {
    pub title: String,
    pub kind: String,
    pub tags: Vec<String>,
    pub summary: Option<String>,
}

fn get_options() -> Options {
    let mut ret = Options::empty();
    ret.insert(Options::ENABLE_TABLES);
    ret.insert(Options::ENABLE_GFM);
    ret.insert(Options::ENABLE_FOOTNOTES);
    ret.insert(Options::ENABLE_STRIKETHROUGH);
    ret.insert(Options::ENABLE_TASKLISTS);
    ret.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    ret.insert(Options::ENABLE_MATH);
    ret
}

fn get_parser(text: &str) -> Parser {
    Parser::new_ext(text, *OPTIONS)
}

fn parse_metadata<'e, I>(events: I) -> ParserOutputMetadata
where
    I: Iterator<Item = &'e Event<'e>>,
{
    let mut inside_metadata = false;
    let mut metadata = None;
    for event in events {
        match event {
            Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                inside_metadata = true
            }
            Event::Text(m) if inside_metadata => metadata = Some(m.clone()),
            Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                inside_metadata = false
            }
            _ => {}
        }
    }

    toml::from_str::<ParserOutputMetadata>(&metadata.unwrap()).unwrap()
}

pub fn parse(text: &str) -> ParserOutput {
    let parser = get_parser(text);

    let events: Vec<_> = parser.into_iter().collect();
    let metadata = parse_metadata(events.iter());

    let highlighter = Highlighter::new(EmbeddedThemeName::Nord);
    let events = highlighter.highlight(events.into_iter());

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    ParserOutput::new(metadata, html)
}
