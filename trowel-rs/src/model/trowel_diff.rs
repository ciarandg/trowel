use std::collections::{HashMap, HashSet};
use std::io;

use ratatui::style::Color;
use ratatui::{
    style::{Modifier, Style}, text::{Line, Span}
};
use serde_json::Value;
use tui_tree_widget::TreeItem;

use super::{tf_plan::{TfPlan, TfPlanResourceChangeChange}, verb::{resource_to_verb, verb_to_color, verb_to_past_tense, Verb}};

pub struct TrowelDiff(Vec<TrowelDiffEntry>);

impl TrowelDiff {
    pub fn from_tf_plan(plan: &TfPlan) -> Result<TrowelDiff, io::Error> {
        let mut out = TrowelDiff(Vec::new());

        for rc in &plan.resource_changes {
            let verb: Verb = resource_to_verb(rc)?;

            if verb != Verb::Ignore {
                let mut values = HashMap::new();
                let resource_names = all_resource_names(&rc.change)?;

                for n in resource_names {
                    values.insert(
                        n.clone(),
                        TrowelDiffEntryBeforeAfter {
                            before: get_before_value(&n, &rc.change)?,
                            after: get_after_value(&n, &rc.change)?,
                        }
                    );
                }

                out.0.push(TrowelDiffEntry {
                    verb,
                    resource_path: rc.address.clone(),
                    values,
              });
            }
        }

        Ok(out)
    }

    pub fn to_tree_items(&self) -> Result<Vec<TreeItem<String>>, io::Error> {
        let mut out = vec![];

        for e in &self.0 {
            let mut values = Vec::new();

            for (k, v) in e.values_sorted() {
                values.push(TreeItem::new_leaf(
                    format!("{} {}", e.resource_path, k),
                    Line::from(
                        std::iter::once(Span::from(k))
                        .chain(std::iter::once(Span::from(" ")))
                        .chain(v.fmt().into_iter())
                        .collect::<Vec<_>>(),
                    ),
                ))
            }

            let item = TreeItem::new(
                    e.resource_path.clone(),
                    Line::from(vec![
                        Span::styled(
                            e.resource_path.to_string(),
                            Style::default().fg(verb_to_color(&e.verb)).add_modifier(Modifier::BOLD)
                        ),
                        Span::from(format!(" will be {}", verb_to_past_tense(&e.verb))),
                    ]),
                    values,
                )?;

            out.push(item);
        }

        Ok(out)
    }

    pub fn verb_uses(&self) -> HashMap<Verb, u8> {
        let mut out = HashMap::new();
        for
         e in &self.0 {
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
            let color = verb_to_color(verb);
            lines.push(
                Span::styled(
                    plaintext,
                    Style::default().fg(color).add_modifier(Modifier::BOLD)
                )
            );

            if i == uses.len() - 1 {
                lines.push(Span::from(" "));
            }
        }
        Line::from(lines)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct TrowelDiffEntryBeforeAfter {
    before: TrowelDiffEntryBefore,
    after: TrowelDiffEntryAfter,
}

impl TrowelDiffEntryBeforeAfter {
    pub fn fmt(&self) -> Vec<Span<'_>> {
        let before = Span::styled(
            Self::plaintext(&self.before),
            Self::style(&self.before)
        );

        let after = Span::styled(
            Self::plaintext(&self.after),
            Self::style(&self.after)
        );

        vec![before, Span::from(" -> "), after]
    }

    fn plaintext(v: &TrowelDiffEntryBefore) -> String {
        match v {
            TrowelDiffEntryBefore::Known(value) => value.to_string(),
            TrowelDiffEntryBefore::Sensitive => "(sensitive value)".to_string(),
            TrowelDiffEntryBefore::Unknown => "(unknown value)".to_string(),
            TrowelDiffEntryBefore::Absent => "(absent value)".to_string(),
        }
    }

    fn style(v: &TrowelDiffEntryBefore) -> Style {
        match v {
            TrowelDiffEntryBefore::Known(_) => Style::default(),
            TrowelDiffEntryBefore::Sensitive => Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            TrowelDiffEntryBefore::Unknown => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            TrowelDiffEntryBefore::Absent => Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
        }
    }
}

#[derive(Clone)]
enum TrowelDiffEntryBefore {
    Known(Value),
    Sensitive,
    Unknown,
    Absent,
}

type TrowelDiffEntryAfter = TrowelDiffEntryBefore;

fn get_before_value(resource_name: &String, change: &TfPlanResourceChangeChange) -> Result<TrowelDiffEntryBefore, io::Error> {
    let before_sensitive: Option<TrowelDiffEntryBefore> = change.process_before_sensitive()?
        .and_then(|map| map.get(resource_name).cloned())
        .map(|_| TrowelDiffEntryBefore::Sensitive);
    let before: Option<TrowelDiffEntryBefore> = change.before
        .as_ref()
        .and_then(|map| map.get(resource_name).cloned())
        .map(|v| TrowelDiffEntryBefore::Known(v.clone()));

    match before_sensitive {
        Some(a) => Ok(a),
        None => match before {
            Some(b) => Ok(b),
            None => Ok(TrowelDiffEntryBefore::Absent)
        }
    }
}

fn get_after_value(resource_name: &String, change: &TfPlanResourceChangeChange) -> Result<TrowelDiffEntryAfter, io::Error> {
    let after_sensitive: Option<TrowelDiffEntryAfter> = change.process_after_sensitive()?
        .and_then(|map| map.get(resource_name).cloned())
        .map(|_| TrowelDiffEntryAfter::Sensitive);
    let after: Option<TrowelDiffEntryAfter> = change.after
        .as_ref()
        .and_then(|map| map.get(resource_name).cloned())
        .map(|v| TrowelDiffEntryAfter::Known(v.clone()));
    let after_unknown: Option<TrowelDiffEntryAfter> = change.after_unknown
        .get(resource_name)
        .map(|_| TrowelDiffEntryAfter::Unknown);

    match after_sensitive {
        Some(a) => Ok(a),
        None => match after {
            Some(b) => Ok(b),
            None => match after_unknown {
                Some(c) => Ok(c),
                None => Ok(TrowelDiffEntryAfter::Absent),
            }
        }
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
    let before_sensitive = &change.process_before_sensitive()?;
    if let Some(map) = before_sensitive {
        for k in map.keys() {
            names.insert(k.to_string());
        }
    }
    let after_sensitive = &change.process_after_sensitive()?;
    if let Some(map) = after_sensitive {
        for k in map.keys() {
            names.insert(k.to_string());
        }
    }

    let mut v: Vec<String> = names.into_iter().collect();
    v.sort();
    Ok(v)
}