use gtk::*;
use webkit2gtk::*;

pub struct Content {
    pub container: Paned,
    pub preview: WebView,
}

impl Content {
    pub fn new() -> Content {
        let container = Paned::new(Orientation::Horizontal);
        let context = WebContext::get_default().unwrap();
        let preview = WebView::new_with_context(&context);
        container.pack1(&preview, true, true);
        Content { container, preview }
    }
}
