use std::{
    collections::{HashMap, HashSet},
    error::Error,
    path::PathBuf,
};

use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The .csv file to balance
    #[arg(value_name = "FILE")]
    sheet: PathBuf,
}

type Owes = HashMap<(String, String), f64>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut names: HashSet<String> = HashSet::new();
    let mut owes: Owes = HashMap::new();
    let cli = Cli::parse();
    let mut rdr = csv::Reader::from_path(cli.sheet)?;
    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|f| f.is_empty()) {
            continue;
        }
        let spender = &record[1];
        let spendees: Vec<&str> = record[2].split(',').map(|s| s.trim()).collect();
        let amt = record[4].parse::<f64>().unwrap() / spendees.len() as f64;
        names.insert(spender.to_owned());
        for spendee in spendees.iter().filter(|s| **s != spender) {
            names.insert(spendee.to_string());
            owes.entry((spendee.to_string(), spender.to_owned()))
                .and_modify(|v| *v += amt)
                .or_insert(amt);
        }
    }
    for pair in names.iter().combinations(2) {
        let Some((ower, owee, amt)) = resolve_owe(&owes, &pair[0], &pair[1]) else {
            continue;
        };
        println!("{ower} owes {owee} {:.2}", amt);
    }
    Ok(())
}

fn resolve_owe(owes: &Owes, person1: &str, person2: &str) -> Option<(String, String, f64)> {
    let amt_1_to_2 = owes.get(&(person1.to_owned(), person2.to_owned()));
    let amt_2_to_1 = owes.get(&(person2.to_owned(), person1.to_owned()));
    match (amt_1_to_2, amt_2_to_1) {
        (None, None) => None,
        (None, Some(amt)) => Some((person2.to_owned(), person1.to_owned(), *amt)),
        (Some(amt), None) => Some((person1.to_owned(), person2.to_owned(), *amt)),
        (Some(amt_1_to_2), Some(amt_2_to_1)) => {
            let diff = amt_1_to_2 - amt_2_to_1;
            if diff > 0.0 {
                Some((person1.to_owned(), person2.to_owned(), diff.abs()))
            } else {
                Some((person2.to_owned(), person1.to_owned(), diff.abs()))
            }
        }
    }
}
