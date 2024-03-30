// https://www.codewars.com/kata/5c9d62cbf1613a001af5b067

use std::{collections::HashMap, iter::FromIterator};

/**
 * use std::collections::HashMap;
use itertools::Itertools;
    
fn radix_tree(words: &[&str]) -> RadixTree {
    let mut tree = RadixTree (HashMap::new());
    for (_, g) in &words.iter().copied()
        .filter(|w| !w.is_empty())
        .sorted()
        .group_by(|w| w.chars().next().unwrap())
    {
        let mut rest: Vec<&str> = g.collect();
        let prefix = rest.iter().copied().reduce(common_prefix).unwrap();
        rest = rest.into_iter().map(|w| &w[prefix.len()..]).collect();
        tree.0.insert(prefix.to_string(), radix_tree(&rest));
    }
    tree
}

fn common_prefix<'a>(s1: &'a str, s2: &str) -> &'a str {
    let end: usize = s1
        .chars()
        .zip(s2.chars())
        .take_while(|(a, b)| a == b)
        .count();
    &s1[..end]
}
 */

#[derive(Debug, PartialEq)]
struct RadixTree(HashMap<String, Self>);

/// These are convenience methods used for tests creation, but you
/// are free to use them yourself if you find a reason to.
impl RadixTree { 
  pub fn new<I: IntoIterator<Item=(&'static str, Self)>>(words: I) -> Self {
    Self(HashMap::from_iter(
      words.into_iter().map(|(k, v)| (String::from(k), v)),
    ))
  }

  pub fn empty() -> Self {
    Self(HashMap::new())
  }
}

fn radix_tree(words: &[&str]) -> RadixTree {
  // find a subset between each word and key of the tree

  // rearrange tree by subset as a new key
  todo!()
}

#[test]
fn returns_expected() {
  assert_eq!(1, 1);
}

#[test]
fn test_radix_initial() {
  let mut instance = RadixTree::new(vec![("hello", RadixTree::empty())]);

  assert_eq!(instance.0.contains_key("hello"), true);

  let remove_result = instance.0.remove("hello");
  assert_eq!(remove_result.is_some(), true);
}

#[test]
fn test_related_two_words() {
  let input = &["ape", "apple"];

  let tree = radix_tree(input);

  assert_eq!(tree.0.contains_key("ap"), true);
}
