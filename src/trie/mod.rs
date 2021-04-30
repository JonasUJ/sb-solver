use std::collections::HashMap;
use std::hash::Hash;

/// Represents a node in a Trie
pub struct Trie<T> {
    nodes: HashMap<T, Trie<T>>,
    marks_word: bool,
}

impl<T: Eq + Hash> Trie<T> {
    /// Create a new empty Trie
    pub fn new() -> Self {
        Trie {
            nodes: HashMap::new(),
            marks_word: false,
        }
    }

    /// Add an item to the Trie.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use sb_solver::trie::Trie;
    /// let mut trie = Trie::<char>::new();
    ///
    /// trie.add("Hello".chars());
    /// assert!(trie.has("Hello".chars()));
    /// ```
    pub fn add(&mut self, item: impl IntoIterator<Item = T>) {
        let mut iter = item.into_iter();

        let next = match iter.next() {
            Some(v) => v,
            None => {
                self.marks_word = true;
                return;
            }
        };

        let node = self.nodes.entry(next).or_insert_with(|| Trie::new());

        node.add(iter);
    }

    /// Remove an item from the Trie.
    /// Return whether the Trie still stores information.
    ///
    /// # Arguments
    ///
    /// * `item` - The iterator to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// use sb_solver::trie::Trie;
    /// let mut trie = Trie::new();
    ///
    /// trie.add("Hello".chars());
    /// assert!(trie.has("Hello".chars()));
    ///
    /// assert!(!trie.remove("Hello".chars()));
    /// assert!(!trie.has("Hello".chars()));
    /// ```
    pub fn remove(&mut self, item: impl IntoIterator<Item = T>) -> bool {
        let mut iter = item.into_iter();

        match iter.next() {
            Some(next) => {
                if let Some(n) = self.nodes.get_mut(&next) {
                    if !n.remove(iter) {
                        self.nodes.remove(&next);
                    }
                }
            }
            None => self.marks_word = false,
        }

        self.marks_word || !self.nodes.is_empty()
    }

    /// Checks whether an item is contained in the Trie.
    ///
    /// # Arguments
    ///
    /// * `item` - An iterator to check.
    ///
    /// # Examples
    ///
    /// ```
    /// use sb_solver::trie::Trie;
    /// let mut trie = Trie::<char>::new();
    ///
    /// assert!(!trie.has("Hello".chars()));
    /// trie.add("Hello".chars());
    /// assert!(trie.has("Hello".chars()));
    /// ```
    pub fn has(&self, item: impl IntoIterator<Item = T>) -> bool {
        let mut iter = item.into_iter();

        let next = match iter.next() {
            Some(v) => v,
            None => return self.marks_word,
        };

        let node = match self.nodes.get(&next) {
            Some(v) => v,
            None => return false,
        };

        node.has(iter)
    }
}

impl<T: std::fmt::Display> Trie<T> {
    pub fn _debug(&self, indent: &usize) {
        for (c, n) in self.nodes.iter() {
            println!(
                "{:indent$}{}{}",
                "",
                c,
                if n.marks_word { "*" } else { "" },
                indent = indent
            );
            n._debug(&(indent + 2));
        }
    }
}

#[cfg(test)]
mod tests;
