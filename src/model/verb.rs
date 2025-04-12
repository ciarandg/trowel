use ratatui::style::Color;
use std::io;

use super::tf_plan::TfPlanResourceChange;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verb {
    Create = 0,
    Update = 3,
    Replace = 2,
    Destroy = 1,
    Read = 4,
    Ignore = 99,
}

impl Verb {
    pub fn from_resource(actions: &TfPlanResourceChange) -> Result<Self, io::Error> {
        let mut sorted = actions.change.actions.clone();
        sorted.sort();
        if sorted == vec!["no-op"] {
            Ok(Self::Ignore)
        } else if sorted == vec!["create"] {
            Ok(Self::Create)
        } else if sorted == vec!["update"] {
            Ok(Self::Update)
        } else if sorted == vec!["delete"] {
            Ok(Self::Destroy)
        } else if sorted == vec!["create", "delete"] {
            Ok(Self::Replace)
        } else if sorted == vec!["read"] {
            Ok(Self::Read)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Verb not found for actions: {:?}", sorted),
            ))
        }
    }

    pub fn to_past_tense(&self) -> String {
        match self {
            Self::Create => "created",
            Self::Update => "updated",
            Self::Replace => "replaced",
            Self::Destroy => "destroyed",
            Self::Read => "read",
            Self::Ignore => "ignored",
        }
        .to_string()
    }

    pub fn to_color(&self) -> Color {
        match self {
            Self::Create => Color::Green,
            Self::Update => Color::Yellow,
            Self::Replace => Color::Magenta,
            Self::Destroy => Color::Red,
            Self::Read => Color::Cyan,
            Self::Ignore => Color::Gray,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Verb::Create => "Create",
            Verb::Update => "Update",
            Verb::Replace => "Replace",
            Verb::Destroy => "Destroy",
            Verb::Read => "Read",
            Verb::Ignore => "Ignore",
        }
    }

    pub fn name_lower(&self) -> String {
        self.name().to_lowercase()
    }
}
