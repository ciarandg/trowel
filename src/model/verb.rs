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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::model::tf_plan::{SensitiveValues, TfPlanResourceChangeChange};

    use super::*;

    #[test]
    fn test_from_resource_ignore() {
        let mut actions = TfPlanResourceChange {
            address: "".to_string(),
            mode: "".to_string(),
            resource_type: "".to_string(),
            name: "".to_string(),
            provider_name: "".to_string(),
            change: TfPlanResourceChangeChange {
                actions: vec![],
                before: None,
                after: None,
                after_unknown: HashMap::new(),
                before_sensitive: SensitiveValues::new(None),
                after_sensitive: SensitiveValues::new(None),
                replace_paths: None,
            },
            action_reason: None,
            module_address: None,
        };

        assert!(Verb::from_resource(&actions).is_err());
        actions.change.actions = vec!["foo".to_string()];
        assert!(Verb::from_resource(&actions).is_err());
        actions.change.actions = vec!["create".to_string(), "foo".to_string()];
        assert!(Verb::from_resource(&actions).is_err());

        actions.change.actions = vec!["no-op".to_string()];
        assert_eq!(Verb::from_resource(&actions).unwrap(), Verb::Ignore);

        actions.change.actions = vec!["create".to_string()];
        assert_eq!(Verb::from_resource(&actions).unwrap(), Verb::Create);

        actions.change.actions = vec!["update".to_string()];
        assert_eq!(Verb::from_resource(&actions).unwrap(), Verb::Update);

        actions.change.actions = vec!["delete".to_string()];
        assert_eq!(Verb::from_resource(&actions).unwrap(), Verb::Destroy);

        actions.change.actions = vec!["create".to_string(), "delete".to_string()];
        assert_eq!(Verb::from_resource(&actions).unwrap(), Verb::Replace);
        actions.change.actions = vec!["delete".to_string(), "create".to_string()];
        assert_eq!(Verb::from_resource(&actions).unwrap(), Verb::Replace);

        actions.change.actions = vec!["read".to_string()];
        assert_eq!(Verb::from_resource(&actions).unwrap(), Verb::Read);
    }
}
