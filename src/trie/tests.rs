use super::*;

#[test]
fn test_add() {
    let mut trie = Trie::<char>::new();
    assert!(!trie.has("Test".chars()));
    trie.add("Test".chars());
    assert!(trie.has("Test".chars()));
}

#[test]
fn test_remove() {
    let mut trie = Trie::<char>::new();
    trie.add("te".chars());
    trie.add("test".chars());
    assert!(trie.has("te".chars()));
    assert!(trie.has("test".chars()));
    assert!(trie.remove("t".chars()));
    assert!(trie.remove("test".chars()));
    assert!(trie.has("te".chars()));
    assert!(!trie.has("test".chars()));
}

#[test]
fn test_has() {
    let mut trie = Trie::<char>::new();
    assert!(!trie.has("te".chars()));
    assert!(!trie.has("test".chars()));
    assert!(!trie.has("tests".chars()));
    trie.add("test".chars());
    trie.add("te".chars());
    assert!(!trie.has("t".chars()));
    assert!(trie.has("te".chars()));
    assert!(trie.has("test".chars()));
    assert!(!trie.has("tests".chars()));
}

#[test]
fn test_unicode() {
    let mut trie = Trie::<char>::new();
    trie.add("日本語".chars());
    assert!(trie.has("日本語".chars()));
    trie.remove("日本語".chars());
    assert!(!trie.has("日本語".chars()));
}
