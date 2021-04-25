use std::hash::Hash;
use std::collections::HashMap;

/// Split after first char, returning the first char, the rest of the str, and wheter it was the
/// last character.
// fn car_cdr(s: &str) -> Option<(char, &str, bool)> {
//     let mut cs = s.chars();
// 
//     match cs.next() {
//         Some(c) => Some((c, &s[c.len_utf8()..], cs.next().is_none())),
//         None => None,
//     }
// }

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
    /// let mut trie = Trie::new();
    ///
    /// trie.add("Hello");
    /// assert!(trie.has("Hello"));
    /// ```
    pub fn add<I>(&mut self, item: I) -> Option<()>
    where
        I: IntoIterator<Item = T>,
    {
        let iter = item.into_iter();
        let next = iter.next()?;

        let node = self.nodes.entry(next).or_insert_with(|| Trie::new());

        node.marks_word = node.add(iter).is_some();

        Some(())
    }

    /// Remove an item from the Trie.
    /// Return whether the Trie is empty.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to remove.
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
    pub fn remove<I>(&mut self, item: I) -> Option<bool>
    where
        I: IntoIterator<Item = T>,
    {
        let iter = item.into_iter();
        let next = iter.next()?;


        // match self.nodes.get_mut(&c) {
        //     Some(n) => {
        //         if l {
        //             n.marks_word = false;
        //             if n.nodes.len() == 0 {
        //                 self.nodes.remove(&c);
        //             }
        //         } else {
        //             if n.remove(s) {
        //                 self.nodes.remove(&c);
        //             }
        //         }
        //     }
        //     None => return false,
        // };

        Some(self.nodes.len() == 0 && !self.marks_word)
    }

    pub fn has(&self, item: &str) -> bool {
        let (c, s, l) = match car_cdr(item) {
            Some(v) => v,
            None => return false,
        };

        match self.nodes.get(&c) {
            Some(n) => l && n.marks_word || n.has(s),
            None => false,
        }
    }

    // pub fn _debug(&self, indent: &usize) {
    //     for (c, n) in self.nodes.iter() {
    //         println!(
    //             "{:indent$}{}{}",
    //             "",
    //             c,
    //             if n.marks_word { "*" } else { "" },
    //             indent = indent
    //         );
    //         n._debug(&(indent + 2));
    //     }
    // }
}

#[cfg(test)]
mod tests;
