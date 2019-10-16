use gtk::*;
use webkit2gtk::WebView;
use webkit2gtk::*;

pub struct Content {
    pub container: Paned,
    pub preview: WebView,
    pub scroll_value: i64,
}

impl Content {
    pub fn new() -> Content {
        let container = Paned::new(Orientation::Horizontal);
        let context = WebContext::get_default().unwrap();
        let preview = WebView::new_with_context(&context);
        container.pack1(&preview, true, true);
        Content {
            container,
            preview,
            scroll_value: 0,
        }
    }
}

    pub fn scroll_to(webview: &WebView, to: i64) {
        let js_scroll = &format!(
            " let scroll_pos = document.documentElement.offsetHeight; \
             window.scrollTo(0, {})",
            to
        );
        webview
            .run_javascript(js_scroll, None::<&gio::Cancellable>, move |_msg| {
                info!("webkit window scrolling to : {} px", to);
            });
    }
