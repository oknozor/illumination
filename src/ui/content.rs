use fragile::Fragile;
use gtk::*;
use std::sync::Arc;
use std::sync::Mutex;
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

    pub fn scroll_to(&self) {
        let scroll = self.scroll_value;
        let js_scroll = &format!("window.scrollTo(0, {})", scroll);

        self.preview
            .run_javascript(js_scroll, None::<&gio::Cancellable>, move |_msg| {
                info!("webkit window scrolling to : {} px", scroll);
            });
    }

    pub fn update_scroll_pos(&mut self) {
        let webview = Arc::new(Mutex::new(Fragile::new(self.preview.clone())));
        let webview_lock = webview.lock().unwrap();
        let context = Fragile::new(webview_lock.get().get_javascript_global_context().unwrap());
        let mut wk_height: i64 = 0;
        webview_lock.get().run_javascript(
            "document.documentElement.offsetHeight",
            None::<&'static gio::Cancellable>,
            move |msg| {
                let current_wk_height = msg.unwrap().get_value().unwrap().to_number(context.get());
                info!("webkit window scroll height : {:?}", current_wk_height);
                wk_height = current_wk_height.unwrap() as i64;
            },
        );
        self.scroll_value = wk_height
    }
}
