use std::io;
use ratatui::style::Color;

use super::tf_plan::TfPlanResourceChange;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verb {
    CREATE,
    UPDATE,
    REPLACE,
    DESTROY,
    READ,
    IGNORE,
}

pub fn resource_to_verb(actions: &TfPlanResourceChange) -> Result<Verb, io::Error> {
    let mut sorted = actions.change.actions.clone();
    sorted.sort();
    if sorted == vec!["no-op"] {
        Ok(Verb::IGNORE)
    } else if sorted == vec!["create"] {
        Ok(Verb::CREATE)
    } else if sorted == vec!["update"] {
        Ok(Verb::UPDATE)
    } else if sorted == vec!["delete"] {
        Ok(Verb::DESTROY)
    } else if sorted == vec!["create", "delete"] {
        Ok(Verb::REPLACE)
    } else if sorted == vec!["read"] {
        Ok(Verb::READ)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Verb not found for actions: {:?}", sorted)
        ))
    }
}

pub fn verb_to_past_tense(verb: &Verb) -> String {
    match verb {
        Verb::CREATE => "created",
        Verb::UPDATE => "updated",
        Verb::REPLACE => "replaced",
        Verb::DESTROY => "destroyed",
        Verb::READ => "read",
        Verb::IGNORE => "ignored",
    }.to_string()
}

pub fn verb_to_color(verb: &Verb) -> Color {
    match verb {
        Verb::CREATE => Color::Green,
        Verb::UPDATE => Color::Yellow,
        Verb::REPLACE => Color::Magenta,
        Verb::DESTROY => Color::Red,
        Verb::READ => Color::Cyan,
        Verb::IGNORE => Color::Gray,
    }
}