use std::collections::HashSet;

use itertools::Itertools;

use crate::storage::{IfKindResult, Kind, Store, Value};

pub enum SetOp<'a> {
    Set(HashSet<String>),
    SetRef(&'a HashSet<String>),
    Empty,
    WrongType,
}

pub enum SetOpCard {
    Count(usize),
    Empty,
    WrongType,
}

pub fn diff<'a>(store: &'a mut Store, keys: &Vec<String>, limit: usize) -> SetOp<'a> {
    if keys.len() == 1 {
        return match store.get_if_kind(Kind::Set, &keys[0]) {
            IfKindResult::Matched(Value::Set(members)) => {
                if members.len() > limit {
                    SetOp::Set(members.iter().take(limit).cloned().collect())
                } else {
                    SetOp::SetRef(members)
                }
            }
            IfKindResult::NotSet => SetOp::Empty,
            _ => SetOp::WrongType,
        };
    }

    match store.get_multi_if_kind(Kind::Set, keys) {
        IfKindResult::Matched(values) => {
            let mut diff = HashSet::new();
            do_diff(values, limit, |member| {
                diff.insert(member.to_string());
                diff.len()
            });

            if diff.is_empty() {
                SetOp::Empty
            } else {
                SetOp::Set(diff)
            }
        }
        IfKindResult::NotSet => SetOp::Empty,
        _ => SetOp::WrongType,
    }
}

fn do_diff<Insert>(values: Vec<&Value>, limit: usize, mut insert: Insert)
where
    Insert: FnMut(&str) -> usize,
{
    let sets: Vec<&HashSet<String>> = values.iter().map(|set| set.expect_set()).collect();

    'outer: for member in sets[0] {
        for set in &sets[1..] {
            if set.contains(member) {
                continue 'outer;
            }
        }
        if insert(member) == limit {
            break;
        }
    }
}

pub fn intersect<'a>(store: &'a mut Store, keys: &Vec<String>, limit: usize) -> SetOp<'a> {
    if keys.len() == 1 {
        return match store.get_if_kind(Kind::Set, &keys[0]) {
            IfKindResult::Matched(Value::Set(members)) => {
                if members.len() > limit {
                    SetOp::Set(members.iter().take(limit).cloned().collect())
                } else {
                    SetOp::SetRef(members)
                }
            }
            IfKindResult::NotSet => SetOp::Empty,
            _ => SetOp::WrongType,
        };
    }

    match store.get_multi_if_kind(Kind::Set, keys) {
        IfKindResult::Matched(values) => {
            if values.len() < keys.len() {
                return SetOp::Empty;
            }

            let mut intersection = HashSet::new();
            do_intersect(values, limit, |member| {
                intersection.insert(member.to_string());
                intersection.len()
            });

            if intersection.is_empty() {
                SetOp::Empty
            } else {
                SetOp::Set(intersection)
            }
        }
        IfKindResult::NotSet => SetOp::Empty,
        _ => SetOp::WrongType,
    }
}

pub fn intersect_card(store: &mut Store, keys: &Vec<String>, limit: usize) -> SetOpCard {
    if keys.len() == 1 {
        return match store.get_if_kind(Kind::Set, &keys[0]) {
            IfKindResult::Matched(Value::Set(members)) => {
                SetOpCard::Count(usize::min(members.len(), limit))
            }
            IfKindResult::NotSet => SetOpCard::Empty,
            _ => SetOpCard::WrongType,
        };
    }

    match store.get_multi_if_kind(Kind::Set, keys) {
        IfKindResult::Matched(values) => {
            if values.len() < keys.len() {
                return SetOpCard::Empty;
            }

            let mut count = 0;
            do_intersect(values, limit, |_| {
                count += 1;
                count
            });

            if count == 0 {
                SetOpCard::Empty
            } else {
                SetOpCard::Count(count)
            }
        }
        IfKindResult::NotSet => SetOpCard::Empty,
        _ => SetOpCard::WrongType,
    }
}

fn do_intersect<Insert>(values: Vec<&Value>, limit: usize, mut insert: Insert)
where
    Insert: FnMut(&str) -> usize,
{
    let sets: Vec<&HashSet<String>> = values
        .iter()
        .map(|set| set.expect_set())
        .sorted_by(|a, b| a.len().cmp(&b.len()))
        .collect();

    'outer: for member in sets[0] {
        for set in &sets[1..] {
            if !set.contains(member) {
                continue 'outer;
            }
        }
        if insert(member) == limit {
            break;
        }
    }
}

pub fn union<'a>(store: &'a mut Store, keys: &Vec<String>, limit: usize) -> SetOp<'a> {
    if keys.len() == 1 {
        return match store.get_if_kind(Kind::Set, &keys[0]) {
            IfKindResult::Matched(Value::Set(members)) => {
                if members.len() > limit {
                    SetOp::Set(members.iter().take(limit).cloned().collect())
                } else {
                    SetOp::SetRef(members)
                }
            }
            IfKindResult::NotSet => SetOp::Empty,
            _ => SetOp::WrongType,
        };
    }

    match store.get_multi_if_kind(Kind::Set, keys) {
        IfKindResult::Matched(values) => {
            let mut union = HashSet::new();
            do_union(values, limit, |member| {
                (union.insert(member.to_string()), union.len())
            });

            if union.is_empty() {
                SetOp::Empty
            } else {
                SetOp::Set(union)
            }
        }
        IfKindResult::NotSet => SetOp::Empty,
        _ => SetOp::WrongType,
    }
}

fn do_union<Insert>(values: Vec<&Value>, limit: usize, mut insert: Insert)
where
    Insert: FnMut(&str) -> (bool, usize),
{
    for set in values.iter().map(|value| value.expect_set()) {
        for member in set {
            let (inserted, count) = insert(member);
            if inserted && count == limit {
                return;
            }
        }
    }
}
