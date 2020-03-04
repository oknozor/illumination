use crate::gtk::prelude::ComboBoxExtManual;
use crate::html::theme::Theme;
use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub theme_selector: ComboBoxText,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        let theme_selector = gtk::ComboBoxText::new();

        Theme::names()
            .iter()
            .for_each(|theme| theme_selector.append_text(theme));
        theme_selector.set_active(Some(1));

        container.add(&theme_selector);
        container.set_title(Some("Illumination"));
        container.set_show_close_button(true);
        Header {
            container,
            theme_selector,
        }
    }
}
