use clap::Args;
use rand::prelude::IndexedRandom;
use regex::Regex;

#[derive(Args)]
pub struct SearchArgs {
    pattern: String,
}

#[derive(Args)]
pub struct QuizArgs {
    num_questions: i32,
}

fn canonical_form(proverb: &str) -> String {
    let re = Regex::new(r"\p{P}").unwrap(); // remove punctuation from string
    re.replace_all(unidecode::unidecode(proverb).to_lowercase().as_str(), "").to_string()
}

pub fn search(proverbs: &Vec<String>, args: SearchArgs) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = args.pattern;
    let results = proverbs.iter()
        .filter(|x| canonical_form(&x).contains(&pattern.to_lowercase()));
    for proverb in results {
        println!("{}", proverb);
    }
    Ok(())
}

pub fn list(proverbs: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for proverb in proverbs {
        println!("{}", proverb);
    }
    Ok(())
}

pub fn random(proverbs: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", proverbs.choose(&mut rand::rng())?);
    Ok(())
}

fn create_hint(proverb: &str) -> String {
    let words: Vec<_> = proverb.split(" ").collect();
    let mut hint = String::new();
    for i in 0..words.len() / 2 {
        hint.push_str(format!("{} ", words[i]).as_str());
    }
    hint.push_str("...");
    hint
}

pub fn quiz(proverbs: &Vec<String>, args: QuizArgs) -> Result<(), Box<dyn std::error::Error>> {
    let num_questions = args.num_questions;
    for i in 0..num_questions {
        println!("Question {}", i + 1);
        let proverb = proverbs.choose(&mut rand::rng())?;
        let hint = create_hint(proverb.as_str());
        println!("Guess the proverb: {}", hint);
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess)?;

        if canonical_form(guess.trim()) == canonical_form(&proverb) {
            println!("Correct!\n");
        } else {
            println!("Wrong! The answer was: {}\n", proverb);
        }
    }
    Ok(())
}
