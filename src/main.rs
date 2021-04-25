extern crate argparse;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use argparse::{ArgumentParser, Store};

use sb_solver::trie::Trie;

fn main() -> io::Result<()> {
    let mut words = "words.txt".to_string();

    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut words).add_option(
            &["-w", "--words"],
            Store,
            "Wordlist to use (default 'words.txt')",
        );
        parser.parse_args_or_exit();
    }

    let f = File::open(&words)?;
    let reader = io::BufReader::new(f);

    println!("Using wordlist: {}", words);

    let mut trie = Trie::new();

    for line in reader.lines() {
        trie.add(line?.as_str());
    }

    trie._debug(&0);

    Ok(())
}
