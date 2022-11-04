use colored::Colorize;

#[derive(Debug, Default)]
pub struct Suggestion(pub String);

impl From<&'static str> for Suggestion {
    fn from(s: &'static str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Suggestion {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl core::fmt::Display for Suggestion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}: {}",
            "Suggestion".underline().purple(),
            self.0.purple()
        )
    }
}
