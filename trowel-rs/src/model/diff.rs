use ratatui::{
    style::{Modifier, Style}, text::{Line, Span}
};
use tui_tree_widget::TreeItem;

use super::{tf_plan::TfPlan, verb::{resource_to_verb, verb_to_color, verb_to_past_tense, Verb}};

pub type Diff = Vec<DiffEntry>;

pub struct DiffEntry {
    pub verb: Verb,
    pub resource_path: String,
}

pub fn diff_from_tf_plan(plan: &TfPlan) -> Diff {
    let mut out = Diff::new();

    for rc in &plan.resource_changes {
        let verb: Verb = resource_to_verb(rc).expect("Could not get verb for resource");

        if verb != Verb::IGNORE {
          out.push(DiffEntry {
              verb,
              resource_path: rc.address.clone(),
          });
        }
    }

    out
}

pub fn tree_items_from_diff(diff: &Diff) -> Vec<TreeItem<String>> {
    let mut out = vec![];

    for e in diff {
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
                vec![
                    TreeItem::new_leaf(format!("{}b", e.resource_path), "value \"0.0.0.0\" -> \"0.0.0.0\""),
                    TreeItem::new_leaf(format!("{}c", e.resource_path), "18 unchanged"),
                ],
            ).expect("all item identifiers are unique"),
        );
    }

    out
}