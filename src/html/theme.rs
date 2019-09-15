use self::Theme::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Default,
    Github,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Default => "default/style.css",
            Github => "github.css",
        }
    }

    pub fn names() -> Vec<&'static str> {
        vec!["Github", "Default"]
    }
}

impl From<&str> for Theme {
    fn from(input: &str) -> Self {
        match input {
            "Github" => Github,
            _ => Default,
        }
    }
}
