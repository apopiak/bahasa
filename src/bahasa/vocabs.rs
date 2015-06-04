extern crate rand;

use std::collections::HashMap;

use self::rand::Rng;

pub struct Vocabs<'a> {
    vocabs: HashMap<&'a str, &'a str>,
    vocabs_back: HashMap<&'a str, &'a str>,
    vocab_vector: Vec<&'a str>,
}

impl<'b> Vocabs<'b> {
    pub fn new<'c> () -> Vocabs<'c> {
        Vocabs { vocabs: HashMap::new(), vocabs_back: HashMap::new(), vocab_vector: vec![] }
    }

    pub fn insert(&mut self, first: &'static str, second: &'static str) {
        self.vocabs.insert(first, second);
        self.vocabs_back.insert(second, first);
        self.vocab_vector.push(first);
    }

    pub fn contains(&self, string: &str) {
        self.vocabs.get(string).is_some()
            && self.vocabs_back.get(string).is_some();
    }

    pub fn get(&self, string: &str) -> Option<&&str> {
        if self.vocabs.get(string).is_some() {
            return self.vocabs.get(string);
        } else if self.vocabs_back.get(string).is_some() {
            return self.vocabs_back.get(string);
        } else {
            return None;
        }
    }

    pub fn get_random(&self) -> &str {
        let r = rand::random::<f64>();
        let index = (r * self.vocab_vector.len() as f64) as usize;
        self.vocab_vector[index]
    }
}
