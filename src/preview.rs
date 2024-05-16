use crate::settings::HLJS_CSS;
use crate::settings::JS;
use crate::settings::THEME;
use horrorshow::helper::doctype;
use horrorshow::Raw;
use pulldown_cmark::{html, Options, Parser};

/// In goes markdown text; out comes HTML text.
fn mark_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_MATH);
    let parser = Parser::new_ext(&markdown, options);
    let mut buffer = String::new();
    html::push_html(&mut buffer, parser);
    buffer
}

/// In goes markdown text; out comes stylish HTML text.
pub fn render(markdown: &str, scroll: f64) -> String {
    let scroll = format!(
        r#"
        let target = document.documentElement.scrollHeight / 100 * {};
        function scrollDown() {{ window.scrollTo(0, target); }};
        window.onload = scrollDown;
        "#,
        scroll
    );

    format!(
        "{}",
        html!(
            : doctype::HTML;
            html {
                head {
                    style {
                        : "body { width: 80%; margin: 0 auto }";
                        : "img { max-width: 80% }";
                        : Raw(HLJS_CSS.as_str());
                        : Raw(THEME.lock().unwrap().contents.as_str());
                    }
                    script {
                        : Raw(JS.as_str());
                    }
                    script {
                        : (scroll.clone());
                        : Raw("hljs.initHighlightingOnLoad();")
                    }

                }
                body {
                    : Raw(&mark_to_html(markdown));
                }
            }
        )
    )
}
