use std::collections::hash_map::{HashMap, Iter};

/// Represents a node in a Trie
pub struct Trie {
    nodes: HashMap<char, Trie>,
    marks_word: bool,
}

pub struct TrieIterator<'a> {
    stack: Vec<Iter<'a, char, Trie>>,
    string: String,
    cur: Option<&'a Trie>,
}

impl<'a> IntoIterator for &'a Trie {
    type Item = String;
    type IntoIter = TrieIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TrieIterator {
            stack: vec![self.nodes.iter()],
            string: String::new(),
            cur: None,
        }
    }
}

impl<'a> Iterator for TrieIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(cur) = self.cur {
                self.stack.push(cur.nodes.iter());
            } 

            #[allow(unused_mut)] // It's not actually unused. It's used by top.next().
            if let Some(mut top) = self.stack.last_mut() {
                if let Some((c, t)) = top.next() {
                    self.string.push(*c);
                    self.cur = Some(t);
                    if t.marks_word {
                        return Some(self.string.clone());
                    }
                } else {
                    self.cur = None;
                    self.stack.pop();
                    self.string.pop();
                }
            } else {
                return None;
            }
        };
    }
}

impl Trie {
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
    /// let mut trie = Trie::new();
    ///
    /// trie.add("Hello");
    /// assert!(trie.has("Hello"));
    /// ```
    pub fn add(&mut self, item: &str) {
        self._add(item.chars());
    }

    fn _add<'a>(&mut self, mut iter: ::std::str::Chars<'a>) {
        let next = match iter.next() {
            Some(v) => v,
            None => {
                self.marks_word = true;
                return;
            }
        };

        let node = self.nodes.entry(next).or_insert_with(Trie::new);

        node._add(iter);
    }

    /// Remove an item from the Trie.
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
    /// trie.add("Hello");
    /// assert!(trie.has("Hello"));
    ///
    /// trie.remove("Hello");
    /// assert!(!trie.has("Hello"));
    /// ```
    pub fn remove(&mut self, item: &str) {
        self._remove(item.chars());
    }

    fn _remove<'a>(&mut self, mut iter: ::std::str::Chars<'a>) -> bool {
        match iter.next() {
            Some(next) => {
                if let Some(n) = self.nodes.get_mut(&next) {
                    if !n._remove(iter) {
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
    /// let mut trie = Trie::new();
    ///
    /// assert!(!trie.has("Hello"));
    /// trie.add("Hello");
    /// assert!(trie.has("Hello"));
    /// ```
    pub fn has(&self, item: &str) -> bool {
        self._has(item.chars())
    }

    fn _has<'a>(&self, mut iter: ::std::str::Chars<'a>) -> bool {
        let next = match iter.next() {
            Some(v) => v,
            None => return self.marks_word,
        };

        let node = match self.nodes.get(&next) {
            Some(v) => v,
            None => return false,
        };

        node._has(iter)
    }

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
