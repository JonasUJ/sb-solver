use super::*;
use std::collections::HashSet;

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
    assert!(trie.remove("t"));
    assert!(trie.remove("test"));
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
    let set: HashSet<String> = ["te", "test", "testing", "other", "none"].iter().map(|s| String::from(*s)).collect();
    let mut empty = HashSet::<String>::new();
    set.iter().for_each(|x| trie.add(x));
    empty.extend(&trie);
    assert!(set == empty);
}
