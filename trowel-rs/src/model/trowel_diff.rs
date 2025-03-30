use std::collections::{HashMap, HashSet};

use ratatui::{
    style::{Modifier, Style}, text::{Line, Span}
};
use serde_json::{json, Value};
use tui_tree_widget::TreeItem;

use super::{tf_plan::{TfPlan, TfPlanResourceChangeChange}, verb::{resource_to_verb, verb_to_color, verb_to_past_tense, Verb}};

pub type TrowelDiff = Vec<TrowelDiffEntry>;

pub struct TrowelDiffEntry {
    pub verb: Verb,
    pub resource_path: String,
    pub values: HashMap<String, TrowelDiffEntryBeforeAfter>,
}

pub struct TrowelDiffEntryBeforeAfter {
    pub before: TrowelDiffEntryBefore,
    pub after: TrowelDiffEntryAfter,
}

impl TrowelDiffEntryBeforeAfter {
    pub fn fmt(&self) -> String {
        let before = match &self.before {
            TrowelDiffEntryBefore::Known(value) => value.to_string(),
            TrowelDiffEntryBefore::Sensitive(_) => "(sensitive value)".to_string(),
            TrowelDiffEntryBefore::Unknown => "(unknown value)".to_string(),
        };

        let after = match &self.after {
            TrowelDiffEntryAfter::Known(value) => value.to_string(),
            TrowelDiffEntryAfter::Sensitive(_) => "(sensitive value)".to_string(),
            TrowelDiffEntryAfter::Unknown => "(unknown value)".to_string(),
        };

        format!("{} -> {}", before, after)
    }
}

enum TrowelDiffEntryBefore {
    Known(Value),
    Sensitive(Value),
    Unknown,
}

type TrowelDiffEntryAfter = TrowelDiffEntryBefore;

pub fn diff_from_tf_plan(plan: &TfPlan) -> TrowelDiff {
    let mut out = TrowelDiff::new();

    for rc in &plan.resource_changes {
        let verb: Verb = resource_to_verb(rc).expect("Could not get verb for resource");

        if verb != Verb::IGNORE {
            let mut values = HashMap::new();
            let resource_names = all_resource_names(&rc.change);

            for n in resource_names {
                values.insert(
                    n,
                    TrowelDiffEntryBeforeAfter {
                        before: TrowelDiffEntryBefore::Known(json!("0.0.0.0")),
                        after: TrowelDiffEntryBefore::Known(json!("1.1.1.1")),
                    }
                );
            }

            out.push(TrowelDiffEntry {
                verb,
                resource_path: rc.address.clone(),
                values,
          });
        }
    }

    out
}

fn all_resource_names(change: &TfPlanResourceChangeChange) -> Vec<String> {
    let mut names: HashSet<String> = HashSet::new();

    if let Some(map) = change.before.as_ref() {
        for (k, _) in map {
            names.insert(k.to_string());
        }
    }
    if let Some(map) = change.after.as_ref() {
        for (k, _) in map {
            names.insert(k.to_string());
        }
    }
    for (k, _) in &change.after_unknown {
        names.insert(k.to_string());
    }
    let before_sensitive = &change.process_before_sensitive().unwrap();
    if let Some(map) = before_sensitive {
        for (k, _) in map {
            names.insert(k.to_string());
        }
    }
    let after_sensitive = &change.process_after_sensitive().unwrap();
    if let Some(map) = after_sensitive {
        for (k, _) in map {
            names.insert(k.to_string());
        }
    }

    let mut v: Vec<String> = names.into_iter().collect();
    v.sort();
    v
}

pub fn tree_items_from_diff(diff: &TrowelDiff) -> Vec<TreeItem<String>> {
    let mut out = vec![];

    for e in diff {
        let mut values = Vec::new();

        for (k, v) in &e.values {
            values.push(TreeItem::new_leaf(
                format!("{} {}", e.resource_path, k),
                format!("{} {}", k, v.fmt()),
            ))
        }

        out.push(
            TreeItem::new(
                e.resource_path.clone(),
                Line::from(vec![
                    Span::styled(
                        format!("{}", e.resource_path),
                        Style::default().fg(verb_to_color(&e.verb)).add_modifier(Modifier::BOLD)
                    ),
                    Span::from(format!(" will be {}", verb_to_past_tense(&e.verb))),
                ]),
                values,
                // vec![
                //     TreeItem::new_leaf(format!("{}b", e.resource_path), "value \"0.0.0.0\" -> \"0.0.0.0\""),
                //     TreeItem::new_leaf(format!("{}c", e.resource_path), "18 unchanged"),
                // ],
            ).expect("all item identifiers are unique"),
        );
    }

    out
}