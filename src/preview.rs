use horrorshow::Raw;
use horrorshow::helper::doctype;
use pulldown_cmark::{html, Parser, Options};

/// In goes markdown text; out comes HTML text.
fn mark_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&markdown, options);
    let mut buffer = String::new();
    html::push_html(&mut buffer, parser);
    buffer
}

/// In goes markdown text; out comes stylish HTML text.
pub fn render(markdown: &str, scroll: i64) -> String {
    let scroll = format!("function scrollDown() {{ window.scrollTo(0, {}); }}; window.onload = scrollDown;", scroll);
    format!(
        "{}",
        html!(
            : doctype::HTML;
            html {
                head {
                    link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/github.min.css") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js") {}
                    script {
                        : Raw("hljs.initHighlightingOnLoad()")
                    }
                    script {
                        : (scroll.clone())
                    }
                    style {
                        : "body { width: 80%; margin: 0 auto }";
                        : "img { max-width: 80% }"
                    }
                }
                body {
                    : Raw(&mark_to_html(markdown));
                }
            }
        )
    )
}