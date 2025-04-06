use std::io;
use ratatui::style::Color;

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

// TODO put all functions below inside impl
pub fn resource_to_verb(actions: &TfPlanResourceChange) -> Result<Verb, io::Error> {
    let mut sorted = actions.change.actions.clone();
    sorted.sort();
    if sorted == vec!["no-op"] {
        Ok(Verb::Ignore)
    } else if sorted == vec!["create"] {
        Ok(Verb::Create)
    } else if sorted == vec!["update"] {
        Ok(Verb::Update)
    } else if sorted == vec!["delete"] {
        Ok(Verb::Destroy)
    } else if sorted == vec!["create", "delete"] {
        Ok(Verb::Replace)
    } else if sorted == vec!["read"] {
        Ok(Verb::Read)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Verb not found for actions: {:?}", sorted)
        ))
    }
}

pub fn verb_to_past_tense(verb: &Verb) -> String {
    match verb {
        Verb::Create => "created",
        Verb::Update => "updated",
        Verb::Replace => "replaced",
        Verb::Destroy => "destroyed",
        Verb::Read => "read",
        Verb::Ignore => "ignored",
    }.to_string()
}

pub fn verb_to_color(verb: &Verb) -> Color {
    match verb {
        Verb::Create => Color::Green,
        Verb::Update => Color::Yellow,
        Verb::Replace => Color::Magenta,
        Verb::Destroy => Color::Red,
        Verb::Read => Color::Cyan,
        Verb::Ignore => Color::Gray,
    }
}