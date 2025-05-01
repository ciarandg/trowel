use std::collections::{HashMap, HashSet};
use std::io;

use ratatui::style::Color;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};
use serde_json::Value;
use tui_tree_widget::TreeItem;

use super::{
    tf_plan::{TfPlan, TfPlanResourceChangeChange},
    verb::Verb,
};

#[derive(Clone)]
pub struct TrowelDiff(Vec<TrowelDiffEntry>);

impl TrowelDiff {
    pub fn from_tf_plan(plan: &TfPlan) -> Result<TrowelDiff, io::Error> {
        let mut out = TrowelDiff(Vec::new());

        if let Some(changes) = plan.resource_changes.as_ref() {
            for rc in changes {
                let verb: Verb = Verb::from_resource(rc)?;

                if verb != Verb::Ignore {
                    let mut values = HashMap::new();
                    let resource_names = all_resource_names(&rc.change)?;

                    for n in resource_names {
                        values.insert(
                            n.clone(),
                            TrowelDiffEntryBeforeAfter {
                                before: get_before_value(&n, &rc.change)?,
                                after: get_after_value(&n, &rc.change)?,
                            },
                        );
                    }

                    out.0.push(TrowelDiffEntry {
                        verb,
                        resource_path: rc.address.clone(),
                        values,
                    });
                }
            }
        }

        Ok(out)
    }

    pub fn to_tree_items(&self) -> Result<Vec<TreeItem<String>>, io::Error> {
        let mut out = vec![];

        for e in &self.0 {
            let mut values = Vec::new();
            let mut unchanged: usize = 0;

            // Assemble a vec of TreeItems containing all of the resource's attributes
            for (k, v) in e.values_sorted() {
                if v.changed() {
                    values.push(TreeItem::new_leaf(
                        format!("{} {}", e.resource_path, k),
                        Line::from(
                            std::iter::once(Span::from(k))
                                .chain(std::iter::once(Span::from(" ")))
                                .chain(v.fmt().into_iter())
                                .collect::<Vec<_>>(),
                        ),
                    ))
                } else {
                    unchanged += 1;
                }
            }

            // Create placeholder TreeItem for unchanged attributes
            if unchanged > 0 {
                values.push(TreeItem::new_leaf(
                    format!("{} unchanged", e.resource_path),
                    Line::from(vec![Span::styled(
                        format!("{} unchanged attributes", unchanged),
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    )]),
                ));
            }

            // Create TreeItem for resource
            let item = TreeItem::new(
                e.resource_path.clone(),
                Line::from(vec![
                    Span::styled(
                        e.resource_path.to_string(),
                        Style::default()
                            .fg(e.verb.to_color())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::from(format!(" will be {}", e.verb.to_past_tense())),
                ]),
                values,
            )?;

            out.push(item);
        }

        Ok(out)
    }

    pub fn verb_uses(&self) -> HashMap<Verb, u8> {
        let mut out = HashMap::new();
        for e in &self.0 {
            let current = *out.get(&e.verb).unwrap_or(&0);
            out.insert(e.verb.clone(), current + 1);
        }
        out
    }

    pub fn verb_uses_fmt(&self) -> Line {
        let mut lines = Vec::new();

        let uses = &self.verb_uses();
        let mut uses: Vec<_> = uses.iter().collect();
        uses.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (i, (verb, use_count)) in uses.iter().enumerate() {
            if i == 0 {
                lines.push(Span::from(" "));
            } else {
                lines.push(Span::from(" | "));
            }

            let plaintext = format!("{} {}", verb.name_lower(), use_count);
            lines.push(Span::styled(
                plaintext,
                Style::default()
                    .fg(verb.to_color())
                    .add_modifier(Modifier::BOLD),
            ));

            if i == uses.len() - 1 {
                lines.push(Span::from(" "));
            }
        }
        Line::from(lines)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrowelDiffEntry {
    pub verb: Verb,
    pub resource_path: String,
    pub values: HashMap<String, TrowelDiffEntryBeforeAfter>,
}

impl TrowelDiffEntry {
    pub fn values_sorted(&self) -> Vec<(&String, &TrowelDiffEntryBeforeAfter)> {
        let mut out: Vec<_> = self.values.iter().collect();
        out.sort_by_key(|(k, _)| *k);
        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrowelDiffEntryBeforeAfter {
    before: TrowelDiffEntryBefore,
    after: TrowelDiffEntryAfter,
}

impl TrowelDiffEntryBeforeAfter {
    pub fn fmt(&self) -> Vec<Span<'_>> {
        let before = Span::styled(Self::plaintext(&self.before), Self::style(&self.before));

        let after = Span::styled(Self::plaintext(&self.after), Self::style(&self.after));

        vec![before, Span::from(" -> "), after]
    }

    fn changed(&self) -> bool {
        match &self.before {
            TrowelDiffEntryBefore::Known(v1) => match &self.after {
                TrowelDiffEntryBefore::Known(v2) => v1 != v2,
                TrowelDiffEntryBefore::Sensitive(v2) => v1 != v2,
                TrowelDiffEntryBefore::Unknown => true,
                TrowelDiffEntryBefore::Absent => true,
            },
            TrowelDiffEntryBefore::Sensitive(v1) => match &self.after {
                TrowelDiffEntryBefore::Known(v2) => v1 != v2,
                TrowelDiffEntryBefore::Sensitive(v2) => v1 != v2,
                TrowelDiffEntryBefore::Unknown => true,
                TrowelDiffEntryBefore::Absent => true,
            },
            TrowelDiffEntryBefore::Unknown => match &self.after {
                TrowelDiffEntryBefore::Known(_) => true,
                TrowelDiffEntryBefore::Sensitive(_) => true,
                TrowelDiffEntryBefore::Unknown => true,
                TrowelDiffEntryBefore::Absent => true,
            },
            TrowelDiffEntryBefore::Absent => match &self.after {
                TrowelDiffEntryBefore::Known(_) => true,
                TrowelDiffEntryBefore::Sensitive(_) => true,
                TrowelDiffEntryBefore::Unknown => true,
                TrowelDiffEntryBefore::Absent => false,
            },
        }
    }

    fn plaintext(v: &TrowelDiffEntryBefore) -> String {
        match v {
            TrowelDiffEntryBefore::Known(value) => value.to_string(),
            TrowelDiffEntryBefore::Sensitive(_) => "(sensitive value)".to_string(),
            TrowelDiffEntryBefore::Unknown => "(unknown value)".to_string(),
            TrowelDiffEntryBefore::Absent => "(absent value)".to_string(),
        }
    }

    fn style(v: &TrowelDiffEntryBefore) -> Style {
        match v {
            TrowelDiffEntryBefore::Known(_) => Style::default(),
            TrowelDiffEntryBefore::Sensitive(_) => Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            TrowelDiffEntryBefore::Unknown => {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            }
            TrowelDiffEntryBefore::Absent => Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TrowelDiffEntryBefore {
    Known(Value),
    Sensitive(Value),
    Unknown,
    Absent,
}

type TrowelDiffEntryAfter = TrowelDiffEntryBefore;

fn get_before_value(
    attribute_name: &String,
    change: &TfPlanResourceChangeChange,
) -> Result<TrowelDiffEntryBefore, io::Error> {
    let before_sensitive: Option<Value> = change
        .before_sensitive
        .inner()
        .clone()
        .map(|m| m.get(attribute_name).cloned())
        .flatten()
        .map(|v| v.clone());
    let before: Option<Value> = change
        .before
        .as_ref()
        .and_then(|map| map.get(attribute_name).cloned())
        .map(|v| v.clone());

    match before_sensitive {
        Some(_) => match before {
            Some(b) => Ok(TrowelDiffEntryBefore::Sensitive(b)),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Before value for attribute {} is in before_sensitive but missing from before",
                    attribute_name
                ),
            )),
        },
        None => match before {
            Some(b) => Ok(TrowelDiffEntryBefore::Known(b)),
            None => Ok(TrowelDiffEntryBefore::Absent),
        },
    }
}

fn get_after_value(
    attribute_name: &String,
    change: &TfPlanResourceChangeChange,
) -> Result<TrowelDiffEntryAfter, io::Error> {
    let after_sensitive: Option<Value> = change
        .after_sensitive
        .inner()
        .clone()
        .map(|m| m.get(attribute_name).cloned())
        .flatten()
        .map(|v| v.clone());
    let after: Option<Value> = change
        .after
        .as_ref()
        .and_then(|map| map.get(attribute_name).cloned())
        .map(|v| v.clone());
    let after_unknown: Option<TrowelDiffEntryAfter> = change
        .after_unknown
        .get(attribute_name)
        .map(|_| TrowelDiffEntryAfter::Unknown);

    match after_unknown {
        Some(a) => Ok(a),
        None => match after_sensitive {
            Some(_) => match after {
                Some(c) => Ok(TrowelDiffEntryAfter::Sensitive(c)),
                None => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "After value for attribute {} is in after_sensitive but missing from after",
                        attribute_name
                    ),
                )),
            },
            None => match after {
                Some(c) => Ok(TrowelDiffEntryAfter::Known(c)),
                None => Ok(TrowelDiffEntryBefore::Absent),
            },
        },
    }
}

fn all_resource_names(change: &TfPlanResourceChangeChange) -> Result<Vec<String>, io::Error> {
    let mut names: HashSet<String> = HashSet::new();

    if let Some(map) = change.before.as_ref() {
        for k in map.keys() {
            names.insert(k.to_string());
        }
    }
    if let Some(map) = change.after.as_ref() {
        for k in map.keys() {
            names.insert(k.to_string());
        }
    }
    for k in change.after_unknown.keys() {
        names.insert(k.to_string());
    }
    if let Some(map) = change.before_sensitive.inner() {
        for k in map.keys() {
            names.insert(k.to_string());
        }
    }
    if let Some(map) = change.after_sensitive.inner() {
        for k in map.keys() {
            names.insert(k.to_string());
        }
    }

    let mut v: Vec<String> = names.into_iter().collect();
    v.sort();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use crate::model::tf_plan::{
        SensitiveValues, TfPlanConfiguration, TfPlanPlannedValues, TfPlanPlannedValuesRootModule,
        TfPlanPriorState, TfPlanResourceChange,
    };

    use super::*;

    #[test]
    fn test_from_tf_plan_empty() {
        let plan = TfPlan {
            format_version: "".to_string(),
            terraform_version: "".to_string(),
            planned_values: TfPlanPlannedValues {
                root_module: TfPlanPlannedValuesRootModule {
                    resources: Some(vec![]),
                    child_modules: Some(vec![]),
                },
            },
            resource_changes: Some(vec![]),
            prior_state: Some(TfPlanPriorState {
                format_version: "".to_string(),
                terraform_version: "".to_string(),
                values: TfPlanPlannedValues {
                    root_module: TfPlanPlannedValuesRootModule {
                        resources: Some(vec![]),
                        child_modules: Some(vec![]),
                    },
                },
            }),
            configuration: TfPlanConfiguration {
                provider_config: Some(HashMap::new()),
                root_module: Value::Null,
            },
            relevant_attributes: Some(vec![]),
            checks: Some(vec![]),
            timestamp: "".to_string(),
            errored: false,
            resource_drift: None,
            applyable: None,
            complete: None,
        };
        let diff = TrowelDiff::from_tf_plan(&plan);
        assert_eq!(diff.unwrap().0.len(), 0)
    }

    #[test]
    fn test_from_tf_plan_resource_change() {
        let plan = TfPlan {
            format_version: "".to_string(),
            terraform_version: "".to_string(),
            planned_values: TfPlanPlannedValues {
                root_module: TfPlanPlannedValuesRootModule {
                    resources: Some(vec![]),
                    child_modules: Some(vec![]),
                },
            },
            resource_changes: Some(vec![TfPlanResourceChange {
                address: "apple".to_string(),
                mode: "orange".to_string(),
                resource_type: "banana".to_string(),
                name: "mango".to_string(),
                provider_name: "guava".to_string(),
                change: TfPlanResourceChangeChange {
                    actions: vec!["create".to_string()],
                    before: None,
                    after: None,
                    after_unknown: HashMap::new(),
                    before_sensitive: SensitiveValues::new(None),
                    after_sensitive: SensitiveValues::new(None),
                    replace_paths: None,
                    importing: None,
                },
                action_reason: None,
                module_address: None,
                index: None,
            }]),
            prior_state: Some(TfPlanPriorState {
                format_version: "".to_string(),
                terraform_version: "".to_string(),
                values: TfPlanPlannedValues {
                    root_module: TfPlanPlannedValuesRootModule {
                        resources: Some(vec![]),
                        child_modules: Some(vec![]),
                    },
                },
            }),
            configuration: TfPlanConfiguration {
                provider_config: Some(HashMap::new()),
                root_module: Value::Null,
            },
            relevant_attributes: Some(vec![]),
            checks: Some(vec![]),
            timestamp: "".to_string(),
            errored: false,
            resource_drift: None,
            applyable: None,
            complete: None,
        };
        let diff = TrowelDiff::from_tf_plan(&plan);
        assert_eq!(diff.as_ref().unwrap().0.len(), 1);
        assert_eq!(
            diff.as_ref().unwrap().0[0],
            TrowelDiffEntry {
                verb: Verb::Create,
                resource_path: "apple".to_string(),
                values: HashMap::new()
            }
        )
    }

    #[test]
    fn test_to_tree_items_empty() {
        let diff = TrowelDiff(vec![]);
        let tree_items = diff.to_tree_items();
        assert_eq!(tree_items.unwrap().len(), 0);
    }

    #[test]
    fn test_to_tree_items_one_empty() {
        let diff = TrowelDiff(vec![TrowelDiffEntry {
            verb: Verb::Create,
            resource_path: "apple".to_string(),
            values: HashMap::new(),
        }]);
        let tree_items = diff.to_tree_items().unwrap();
        assert_eq!(tree_items.len(), 1);
        let item = &tree_items[0];
        assert_eq!(item.identifier(), "apple");
        assert_eq!(item.children().len(), 0);
    }

    #[test]
    fn test_to_tree_items_one_empty_nonempty() {
        let diff = TrowelDiff(vec![TrowelDiffEntry {
            verb: Verb::Create,
            resource_path: "apple".to_string(),
            values: HashMap::from([
                (
                    "c".to_string(),
                    TrowelDiffEntryBeforeAfter {
                        before: TrowelDiffEntryBefore::Known(Value::String("old".to_string())),
                        after: TrowelDiffEntryBefore::Known(Value::String("new".to_string())),
                    },
                ),
                (
                    "b".to_string(),
                    TrowelDiffEntryBeforeAfter {
                        before: TrowelDiffEntryBefore::Known(Value::String("old".to_string())),
                        after: TrowelDiffEntryBefore::Known(Value::String("old".to_string())),
                    },
                ),
                (
                    "d".to_string(),
                    TrowelDiffEntryBeforeAfter {
                        before: TrowelDiffEntryBefore::Sensitive(Value::String("old".to_string())),
                        after: TrowelDiffEntryBefore::Sensitive(Value::String("old".to_string())),
                    },
                ),
                (
                    "e".to_string(),
                    TrowelDiffEntryBeforeAfter {
                        before: TrowelDiffEntryBefore::Sensitive(Value::String("old".to_string())),
                        after: TrowelDiffEntryBefore::Sensitive(Value::String("new".to_string())),
                    },
                ),
                (
                    "a".to_string(),
                    TrowelDiffEntryBeforeAfter {
                        before: TrowelDiffEntryBefore::Unknown,
                        after: TrowelDiffEntryBefore::Unknown,
                    },
                ),
            ]),
        }]);
        let tree_items = diff.to_tree_items().unwrap();
        assert_eq!(tree_items.len(), 1);
        let item = &tree_items[0];
        assert_eq!(item.identifier(), "apple");
        assert_eq!(item.children().len(), 4);

        // Resource attributes are alphabetized, with unchanged at the end
        let value_identifiers: Vec<_> = item.children().iter().map(|i| i.identifier()).collect();
        assert_eq!(
            value_identifiers,
            vec!["apple a", "apple c", "apple e", "apple unchanged"]
        );
    }

    #[test]
    fn test_to_tree_items_multiple_empty() {
        let diff = TrowelDiff(vec![
            TrowelDiffEntry {
                verb: Verb::Create,
                resource_path: "orange".to_string(),
                values: HashMap::new(),
            },
            TrowelDiffEntry {
                verb: Verb::Update,
                resource_path: "banana".to_string(),
                values: HashMap::new(),
            },
            TrowelDiffEntry {
                verb: Verb::Destroy,
                resource_path: "apple".to_string(),
                values: HashMap::new(),
            },
        ]);
        let tree_items = diff.to_tree_items().unwrap();
        assert_eq!(tree_items.len(), 3);
        let identifiers: Vec<_> = tree_items.iter().map(|i| i.identifier()).collect();
        assert_eq!(identifiers, vec!["orange", "banana", "apple"]) // Does not alphabetize resources
    }

    #[test]
    fn test_verb_uses_empty() {
        let diff = TrowelDiff(vec![]);
        let uses = diff.verb_uses();
        assert_eq!(uses.len(), 0);
    }

    #[test]
    fn test_verb_uses_one() {
        let diff = TrowelDiff(vec![TrowelDiffEntry {
            verb: Verb::Create,
            resource_path: "foo".to_string(),
            values: HashMap::new(),
        }]);

        let uses = diff.verb_uses();
        assert_eq!(uses, [(Verb::Create, 1)].into_iter().collect())
    }

    #[test]
    fn test_verb_uses_multiple() {
        let diff = TrowelDiff(vec![
            TrowelDiffEntry {
                verb: Verb::Create,
                resource_path: "foo".to_string(),
                values: HashMap::new(),
            },
            TrowelDiffEntry {
                verb: Verb::Update,
                resource_path: "bar".to_string(),
                values: HashMap::new(),
            },
            TrowelDiffEntry {
                verb: Verb::Create,
                resource_path: "baz".to_string(),
                values: HashMap::new(),
            },
        ]);

        let uses = diff.verb_uses();
        assert_eq!(
            uses,
            [(Verb::Create, 2), (Verb::Update, 1)].into_iter().collect()
        )
    }

    #[test]
    fn test_verb_uses_fmt_empty() {
        let diff = TrowelDiff(vec![]);
        let uses = diff.verb_uses_fmt();
        assert_eq!(uses, Line::from(vec![]));
    }

    #[test]
    fn test_verb_uses_fmt_one() {
        let diff = TrowelDiff(vec![TrowelDiffEntry {
            verb: Verb::Create,
            resource_path: "foo".to_string(),
            values: HashMap::new(),
        }]);
        let uses = diff.verb_uses_fmt();
        assert_eq!(
            uses,
            Line::from(vec![
                Span::from(" "),
                Span::styled(
                    "create 1",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                ),
                Span::from(" "),
            ])
        );
    }

    #[test]
    fn test_verb_uses_fmt_multiple() {
        let diff = TrowelDiff(vec![
            TrowelDiffEntry {
                verb: Verb::Destroy,
                resource_path: "apple".to_string(),
                values: HashMap::new(),
            },
            TrowelDiffEntry {
                verb: Verb::Update,
                resource_path: "banana".to_string(),
                values: HashMap::new(),
            },
            TrowelDiffEntry {
                verb: Verb::Create,
                resource_path: "orange".to_string(),
                values: HashMap::new(),
            },
            TrowelDiffEntry {
                verb: Verb::Destroy,
                resource_path: "mango".to_string(),
                values: HashMap::new(),
            },
        ]);
        let uses = diff.verb_uses_fmt();
        // Entries will be sorted by the numeric value of each Verb
        assert_eq!(
            uses,
            Line::from(vec![
                Span::from(" "),
                Span::styled(
                    "create 1",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                ),
                Span::from(" | "),
                Span::styled(
                    "destroy 2",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                ),
                Span::from(" | "),
                Span::styled(
                    "update 1",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                ),
                Span::from(" "),
            ])
        );
    }

    #[test]
    fn test_get_before_value() {
        let change = TfPlanResourceChangeChange {
            actions: vec![],
            before: Some(HashMap::from([
                ("banana".to_string(), Value::String("mango".to_string())),
                ("apple".to_string(), Value::String("orange".to_string())),
                ("pineapple".to_string(), Value::String("papaya".to_string())),
            ])),
            after: None,
            after_unknown: HashMap::new(),
            before_sensitive: SensitiveValues::new(Some(HashMap::from([(
                "pineapple".to_string(),
                Value::Bool(true),
            )]))),
            after_sensitive: SensitiveValues::new(None),
            replace_paths: None,
            importing: None,
        };

        assert_eq!(
            get_before_value(&"apple".to_string(), &change).unwrap(),
            TrowelDiffEntryBefore::Known(Value::String("orange".to_string()))
        );
        assert_eq!(
            get_before_value(&"pineapple".to_string(), &change).unwrap(),
            TrowelDiffEntryBefore::Sensitive(Value::String("papaya".to_string()))
        );
        assert_eq!(
            get_before_value(&"guava".to_string(), &change).unwrap(),
            TrowelDiffEntryBefore::Absent
        );
    }

    #[test]
    fn test_get_before_value_err() {
        let change = TfPlanResourceChangeChange {
            actions: vec![],
            before: Some(HashMap::new()),
            after: None,
            after_unknown: HashMap::new(),
            before_sensitive: SensitiveValues::new(Some(HashMap::from([(
                "apple".to_string(),
                Value::String("banana".to_string()),
            )]))),
            after_sensitive: SensitiveValues::new(None),
            replace_paths: None,
            importing: None,
        };

        assert!(get_before_value(&"apple".to_string(), &change).is_err()); // present in before_sensitive but missing in before
    }

    #[test]
    fn test_get_after_value() {
        let change = TfPlanResourceChangeChange {
            actions: vec![],
            before: None,
            after: Some(HashMap::from([
                ("banana".to_string(), Value::String("mango".to_string())),
                ("apple".to_string(), Value::String("orange".to_string())),
                ("pineapple".to_string(), Value::String("papaya".to_string())),
            ])),
            after_unknown: HashMap::from([(
                "dragonfruit".to_string(),
                Value::String("pear".to_string()),
            )]),
            before_sensitive: SensitiveValues::new(None),
            after_sensitive: SensitiveValues::new(Some(HashMap::from([(
                "pineapple".to_string(),
                Value::Bool(true),
            )]))),
            replace_paths: None,
            importing: None,
        };

        assert_eq!(
            get_after_value(&"apple".to_string(), &change).unwrap(),
            TrowelDiffEntryBefore::Known(Value::String("orange".to_string()))
        );
        assert_eq!(
            get_after_value(&"pineapple".to_string(), &change).unwrap(),
            TrowelDiffEntryBefore::Sensitive(Value::String("papaya".to_string()))
        );
        assert_eq!(
            get_after_value(&"guava".to_string(), &change).unwrap(),
            TrowelDiffEntryBefore::Absent
        );
        assert_eq!(
            get_after_value(&"dragonfruit".to_string(), &change).unwrap(),
            TrowelDiffEntryBefore::Unknown
        );
    }

    #[test]
    fn test_get_after_value_err() {
        let change = TfPlanResourceChangeChange {
            actions: vec![],
            before: None,
            after: Some(HashMap::new()),
            after_unknown: HashMap::new(),
            before_sensitive: SensitiveValues::new(None),
            after_sensitive: SensitiveValues::new(Some(HashMap::from([(
                "apple".to_string(),
                Value::String("banana".to_string()),
            )]))),
            replace_paths: None,
            importing: None,
        };

        assert!(get_after_value(&"apple".to_string(), &change).is_err()); // present in after_sensitive but missing in after
    }
}
