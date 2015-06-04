use std::io::stdin;
use std::io::BufRead;

pub mod bahasa;
use bahasa::vocabs::Vocabs;

fn main() {
    println!("Hello!");
    println!("Welcome to learning Bahasa Indonesia");
    println!(" ");

    let mut vocabs = Vocabs::new();
    initialize_vocabs(&mut vocabs);

    loop {
        let word = vocabs.get_random();
        println!("What is {} in English?", word);

        let stdin = stdin();
        for line in stdin.lock().lines() {
            let mut line = line.unwrap();
            line = line.trim().to_string();
            let line_string: &str = line.as_ref();

            let vocab = vocabs.get(line_string);
            if vocab.is_some() {
                let st = vocab.unwrap();
                if  *st == word {
                    println!("That's right! It's {}.", line_string);
                    println!(" ");
                    break;
                }
            }
            println!("That's wrong!");
            continue;
        }
    }
}

fn initialize_vocabs(vocabs: &mut Vocabs) {
    vocabs.insert("satu", "one");
    vocabs.insert("dua", "two");
    vocabs.insert("tiga", "three");
    vocabs.insert("empat", "four");
    vocabs.insert("lima", "five");
    vocabs.insert("enam", "six");
    vocabs.insert("tujuh", "seven");
    vocabs.insert("delapan", "eight");
    vocabs.insert("sembilan", "nine");
    vocabs.insert("sepuluh", "ten");
}
