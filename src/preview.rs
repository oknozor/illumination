use horrorshow::Raw;
use horrorshow::helper::doctype;
use pulldown_cmark::{html, Parser};

/// In goes markdown text; out comes HTML text.
fn mark_to_html(markdown: &str) -> String {
    let parser = Parser::new(&markdown);
    let mut buffer = String::new();
    html::push_html(&mut buffer, parser);
    buffer
}

/// In goes markdown text; out comes stylish HTML text.
pub fn render(markdown: &str) -> String {
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