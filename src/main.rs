extern crate argparse;

use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::time::SystemTime;
use std::{collections::HashSet, fs::File};

use argparse::{ArgumentParser, Store};

use sb_solver::trie::Trie;

fn main() -> io::Result<()> {
    let mut words = "words.txt".to_string();
    let mut mandatory = char::MAX;
    let mut characters = String::new();
    let mut quiet = false;
    let mut no_output = false;

    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut words).add_option(
            &["-w", "--words"],
            Store,
            "Wordlist to use (default 'words.txt')",
        );
        parser.refer(&mut mandatory).add_option(
            &["-m", "--mandatory"],
            Store,
            "Single character that must appear in the word.",
        );
        parser.refer(&mut characters).add_option(
            &["-c", "--characters"],
            Store,
            "String of characters that might appear in the word",
        );
        parser
            .refer(&mut quiet)
            .add_option(&["-q", "--quiet"], Store, "No progress messages.");
        parser.refer(&mut no_output).add_option(
            &["-n", "--no-output"],
            Store,
            "Don't output solutions to stdout.",
        );
        parser.parse_args_or_exit();
    }

    if mandatory == char::MAX {
        panic!("--mandatory flag not specified.");
    }

    let f = File::open(&words)?;
    let reader = io::BufReader::new(f);

    let mut trie = Trie::new();

    if !quiet {
        println!("Building Trie...");
    }
    let mut words = 0;

    for line in reader.lines() {
        trie.add(line?.as_str());
        words += 1;
    }

    let before = SystemTime::now();
    if !quiet {
        println!("Finished building Trie containing {} words.", words);
        println!("Finding solutions...");
    }

    let solutions: Vec<String> = trie
        .find_solutions(&mandatory, HashSet::from_iter(characters.chars()))
        .collect();

    if !quiet {
        if let Ok(diff) = SystemTime::now().duration_since(before) {
            println!("Took {:?}", diff);
        }
    }

    if !no_output {
        solutions.iter().for_each(|s| {
            println!("{}", s);
        });
    }

    Ok(())
}
