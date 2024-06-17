use std::collections::HashSet;

use itertools::Itertools;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

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
        return trim_to_limit(store, &keys[0], limit);
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
        return trim_to_limit(store, &keys[0], limit);
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
        return trim_to_limit(store, &keys[0], limit);
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

fn trim_to_limit<'a>(store: &'a Store, key: &str, limit: usize) -> SetOp<'a> {
    return match store.get_if_kind(Kind::Set, key) {
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

pub enum Random {
    Single(String),
    Elements(Vec<String>),
    Empty,
    NotSet,
    WrongType,
}

pub fn random_members(store: &mut Store, key: &str, count: usize, dupes: bool) -> Random {
    if count == 0 {
        return Random::Empty;
    }

    match store.get_mut_if_kind(Kind::Set, key) {
        IfKindResult::Matched(Value::Set(ref mut members)) => {
            let mut rng = thread_rng();

            if count == 1 {
                let member = members
                    .iter()
                    .nth(rng.gen_range(0..members.len()))
                    .unwrap()
                    .clone();

                return Random::Single(member);
            }

            let mut pool = members.iter().collect_vec();
            let chosen = if dupes {
                let mut chosen = Vec::with_capacity(count);
                for _ in 0..count {
                    chosen.push(pool[rng.gen_range(0..pool.len())].clone());
                }
                chosen
            } else {
                pool.shuffle(&mut rng);
                pool.into_iter().take(count).cloned().collect_vec()
            };

            Random::Elements(chosen)
        }
        IfKindResult::NotSet => Random::NotSet,
        _ => Random::WrongType,
    }
}

pub fn pop_random_members(store: &mut Store, key: &str, count: usize) -> Random {
    if count == 0 {
        return Random::Empty;
    }

    match store.get_mut_if_kind(Kind::Set, key) {
        IfKindResult::Matched(Value::Set(ref mut members)) => {
            let mut rng = thread_rng();

            if count == 1 {
                let member = if members.len() == 1 {
                    let member = members.iter().next().expect("non-empty set").clone();
                    store.remove(key);
                    member
                } else {
                    let member = members
                        .iter()
                        .nth(rng.gen_range(0..members.len()))
                        .unwrap()
                        .clone();
                    members.remove(&member);
                    member
                };

                return Random::Single(member);
            }

            let chosen = if members.len() <= count {
                let mut chosen = members.iter().cloned().collect_vec();
                store.remove(key);
                chosen.shuffle(&mut rng);
                chosen
            } else {
                let mut pool = members.iter().cloned().collect_vec();
                pool.shuffle(&mut rng);
                let chosen = pool.into_iter().take(count).collect_vec();
                for member in &chosen {
                    members.remove(member);
                }
                chosen
            };

            Random::Elements(chosen)
        }
        IfKindResult::NotSet => Random::NotSet,
        _ => Random::WrongType,
    }
}
