use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag, TagEnd};
use two_face::{
    re_exports::syntect::html,
    re_exports::syntect::parsing::SyntaxSet,
    theme::{EmbeddedLazyThemeSet, EmbeddedThemeName},
};

pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme_set: EmbeddedLazyThemeSet,
    theme: EmbeddedThemeName,
}

impl Highlighter {
    pub fn new(theme: EmbeddedThemeName) -> Self {
        let syntax_set = two_face::syntax::extra_newlines();
        let theme_set = two_face::theme::extra();

        Self {
            syntax_set,
            theme_set,
            theme,
        }
    }

    pub fn highlight<'e, IE: Iterator<Item = Event<'e>>>(&self, events: IE) -> Vec<Event<'e>> {
        let theme = self.theme_set.get(self.theme);
        let mut syntax = self.syntax_set.find_syntax_plain_text();
        let mut inside_code = false;

        let mut ret = vec![];

        let mut buffer = String::new();
        for event in events {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    if let CodeBlockKind::Fenced(lang) = kind {
                        syntax = self
                            .syntax_set
                            .find_syntax_by_token(&lang)
                            .unwrap_or(syntax);
                    }
                    inside_code = true
                }
                Event::End(TagEnd::CodeBlock) => {
                    if !inside_code {
                        unreachable!("Run into CodeBlock end without a CodeBlock start")
                    }

                    let html =
                        html::highlighted_html_for_string(&buffer, &self.syntax_set, syntax, theme)
                            .unwrap();

                    buffer.clear();
                    inside_code = false;
                    ret.push(Event::Html(CowStr::from(html)));
                }
                Event::Text(text) => {
                    if inside_code {
                        buffer.push_str(&text);
                    } else {
                        ret.push(Event::Text(text));
                    }
                }
                ev => {
                    ret.push(ev);
                }
            }
        }

        ret
    }
}
