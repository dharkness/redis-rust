use std::collections::HashSet;

use itertools::Itertools;

use crate::storage::{IfKindResult, Kind, Store, Value};

pub enum Intersect<'a> {
    Set(HashSet<String>),
    SetRef(&'a HashSet<String>),
    Empty,
    WrongType,
}

pub fn intersect<'a>(store: &'a mut Store, keys: &Vec<String>, limit: usize) -> Intersect<'a> {
    if keys.len() == 1 {
        return match store.get_if_kind(Kind::Set, &keys[0]) {
            IfKindResult::Matched(Value::Set(members)) => {
                if members.len() > limit {
                    Intersect::Set(members.iter().take(limit).cloned().collect())
                } else {
                    Intersect::SetRef(members)
                }
            }
            IfKindResult::NotSet => Intersect::Empty,
            _ => Intersect::WrongType,
        };
    }

    match store.get_multi_if_kind(Kind::Set, keys) {
        IfKindResult::Matched(values) => {
            if values.len() < keys.len() {
                return Intersect::Empty;
            }

            let mut intersection = HashSet::new();
            do_intersect(values, limit, |member| {
                intersection.insert(member.to_string());
                intersection.len()
            });

            if intersection.is_empty() {
                Intersect::Empty
            } else {
                Intersect::Set(intersection)
            }
        }
        IfKindResult::NotSet => Intersect::Empty,
        _ => Intersect::WrongType,
    }
}

pub enum IntersectCard {
    Count(usize),
    Empty,
    WrongType,
}

pub fn intersect_card(store: &mut Store, keys: &Vec<String>, limit: usize) -> IntersectCard {
    if keys.len() == 1 {
        return match store.get_if_kind(Kind::Set, &keys[0]) {
            IfKindResult::Matched(Value::Set(members)) => {
                IntersectCard::Count(usize::min(members.len(), limit))
            }
            IfKindResult::NotSet => IntersectCard::Empty,
            _ => IntersectCard::WrongType,
        };
    }

    match store.get_multi_if_kind(Kind::Set, keys) {
        IfKindResult::Matched(values) => {
            if values.len() < keys.len() {
                return IntersectCard::Empty;
            }

            let mut count = 0;
            do_intersect(values, limit, |_| {
                count += 1;
                count
            });

            if count == 0 {
                IntersectCard::Empty
            } else {
                IntersectCard::Count(count)
            }
        }
        IfKindResult::NotSet => IntersectCard::Empty,
        _ => IntersectCard::WrongType,
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

    'outer: for value in sets[0] {
        for set in &sets[1..] {
            if !set.contains(value) {
                continue 'outer;
            }
        }
        if insert(value) == limit {
            break;
        }
    }
}
