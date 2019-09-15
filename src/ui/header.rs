use crate::html::theme::Theme;
use crate::settings;
use gtk::*;

pub struct Header {
    pub container: HeaderBar,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        let combo = gtk::ComboBoxText::new();

        Theme::names()
            .iter()
            .for_each(|theme| combo.append_text(theme));
        combo.set_active(Some(0));

        combo.connect_changed(move |combo| {
            let selection = combo.get_active_text().unwrap();
            let selection = selection.as_str();
            info!("changing theme to : {}", selection);
            settings::set_theme(Theme::from(selection));
        });

        container.add(&combo);
        container.set_title(Some("Illumination"));
        container.set_show_close_button(true);
        Header { container }
    }
}
