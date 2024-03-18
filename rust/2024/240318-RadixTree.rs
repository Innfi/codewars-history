// https://www.codewars.com/kata/5c9d62cbf1613a001af5b067

use std::{collections::HashMap, iter::FromIterator};

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
  let instance = RadixTree::new([("hello", RadixTree::empty())]);

  assert_eq!(instance.0.contains_key("hello"), true);
}
