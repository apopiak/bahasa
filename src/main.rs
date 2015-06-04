extern crate rand;

use std::io::stdin;
use std::io::BufRead;
use std::collections::HashMap;

use rand::Rng;

struct Vocabs<'a> {
    vocabs: HashMap<&'a str, &'a str>,
    vocabs_back: HashMap<&'a str, &'a str>,
    vocab_vector: Vec<&'a str>,
}
// read up on lifetimes!
impl<'b> Vocabs<'b> {
    fn new<'c> () -> Vocabs<'c> {
        Vocabs { vocabs: HashMap::new(), vocabs_back: HashMap::new(), vocab_vector: vec![] }
    }

    fn insert(&mut self, first: &'static str, second: &'static str) {
        self.vocabs.insert(first, second);
        self.vocabs_back.insert(second, first);
        self.vocab_vector.push(first);
    }

    fn contains(&self, string: &str) {
        self.vocabs.get(string).is_some()
            && self.vocabs_back.get(string).is_some();
    }

    fn get(&self, string: &str) -> Option<&&str> {
        if self.vocabs.get(string).is_some() {
            return self.vocabs.get(string);
        } else if self.vocabs_back.get(string).is_some() {
            return self.vocabs_back.get(string);
        } else {
            return None;
        }
    }

    fn get_random(&self) -> &str {
        let r = rand::random::<f64>();
        let index = (r * self.vocab_vector.len() as f64) as usize;
        self.vocab_vector[index]
    }
}

fn main() {
    println!("Hello!");
    println!("Welcome to learning Bahasa Indonesia");
    println!(" ");
    println!("What is dua in English?");

    let mut vocabs = Vocabs::new();
    vocabs.insert("satu", "one");
    vocabs.insert("dua", "two");
    vocabs.insert("tiga", "three");
    vocabs.insert("enam", "four");

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let mut line = line.unwrap();
        line = line.trim().to_string();
        let line_string: &str = line.as_ref();

        let vocab = vocabs.get(line_string);
        if vocab.is_some() {
            let st = vocab.unwrap();
            if  *st == "dua" {
                println!("That's right! It's {}.", line_string);
                continue;
            }
        }
        println!("That's wrong! It's {}!", vocabs.get("dua").unwrap());
        continue;
    }
}
