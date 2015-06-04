extern crate rand;

use bahasa::bimap::Bimap;


pub struct Vocabs<'a> {
    vocabs: Bimap<&'a str, &'a str>,
    vocab_vector: Vec<&'a str>,
}

impl<'b> Vocabs<'b> {
    pub fn new<'c> () -> Vocabs<'c> {
        Vocabs { vocabs: Bimap::new(), vocab_vector: vec![] }
    }

    pub fn insert(&mut self, first: &'static str, second: &'static str) {
        self.vocabs.insert(first, second);
        self.vocab_vector.push(first);
    }

    pub fn get<'a>(&'a self, string: &'a str) -> Option<&&str> {
        self.vocabs.get(&string)
    }

    pub fn get_random(&self) -> &str {
        let r = rand::random::<f64>();
        let index = (r * self.vocab_vector.len() as f64) as usize;
        self.vocab_vector[index]
    }
}
