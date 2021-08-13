use super::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_add() {
    let mut trie = Trie::new();
    assert!(!trie.has("test"));
    trie.add("test");
    assert!(trie.has("test"));
}

#[test]
fn test_remove() {
    let mut trie = Trie::new();
    trie.add("te");
    trie.add("test");
    assert!(trie.has("te"));
    assert!(trie.has("test"));
    trie.remove("t");
    trie.remove("test");
    assert!(trie.has("te"));
    assert!(!trie.has("test"));
}

#[test]
fn test_has() {
    let mut trie = Trie::new();
    assert!(!trie.has("te"));
    assert!(!trie.has("test"));
    assert!(!trie.has("tests"));
    trie.add("test");
    trie.add("te");
    assert!(!trie.has("t"));
    assert!(trie.has("te"));
    assert!(trie.has("test"));
    assert!(!trie.has("tests"));
}

#[test]
fn test_unicode() {
    let mut trie = Trie::new();
    trie.add("日本語");
    assert!(trie.has("日本語"));
    trie.remove("日本語");
    assert!(!trie.has("日本語"));
}

#[test]
fn test_iter() {
    let mut trie = Trie::new();
    let set: HashSet<String> = ["te", "test", "testing", "other", "none"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut empty = HashSet::<String>::new();
    set.iter().for_each(|x| trie.add(x));
    empty.extend(&trie);
    assert!(set == empty);
}

#[test]
fn test_with_prefix() {
    let mut trie = Trie::new();
    let set: HashSet<String> = ["te", "test", "testing", "other", "none"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let subset: HashSet<String> = ["test", "testing"].iter().map(|s| s.to_string()).collect();
    let mut empty = HashSet::<String>::new();
    set.iter().for_each(|x| trie.add(x));
    empty.extend(trie.with_prefix("test"));
    assert!(subset == empty);
}

#[test]
fn test_find_solutions() {
    let mut trie = Trie::new();
    let set: HashSet<String> = ["te", "test", "testing", "other", "none"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let subset: HashSet<String> = ["test", "testing"].iter().map(|s| s.to_string()).collect();
    let mut empty = HashSet::<String>::new();
    set.iter().for_each(|x| trie.add(x));
    empty.extend(trie.find_solutions(
        &'s',
        HashSet::from_iter(['t', 'e', 'i', 'n', 'g'].iter().cloned()),
    ));
    assert!(subset == empty);
}
