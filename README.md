# sb-solver
Solves [Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee) puzzles using the specified wordlist and a trie.

### Usage
- The wordlist is specified using the `--words` argument. It defaults to "words.txt".
- The character that must appear in the word is set by the `--mandatory` argument.
- Other characters that the word can consist of are passed as one string to `--characters`.
- `--quiet` and `--no-output` allow disabling progress output and result output respectively.

### Example
Say the puzzle is using the letter "c" with the letters "m", "i", "n", "g", "s", and "t". To solve this, do the following steps:

1. Find a wordlist.
    1. If you are on Ubuntu, install one with `sudo apt install wamerican`.
2. Run the command `sb-solver -w /usr/share/dict/american-english -m c -c mingst`.

That's all.

To get some more useful output in cases where there are many solutions, you might end up running a command like: 

`sb-solver -w /usr/share/dict/american-english -m c -c mingst -q true | awk '{print length(), $0} | sort -nr | cut -d " " -f 2- | less`

This will open all the solutions in less, sorted by length (by points).
