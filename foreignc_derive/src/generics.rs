
use syn::*;

pub struct GenericGenerator {
    alphabete: Vec<char>,
    i: usize,
    pub generics: Vec<char>
}

impl GenericGenerator {
    pub fn new() -> GenericGenerator {
        GenericGenerator {
            alphabete: (b'A'..=b'Z')
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect(),
            i: 0,
            generics: Vec::new()
        }
    }

    pub fn get_generic(&mut self) -> char {
        let c = self.alphabete[self.i];
        self.generics.push(c);
        self.i += 1;
        c
    }

    pub fn generate(&self) -> Vec<GenericParam> {
        self.generics
        .iter()
        .map(|g| parse_str::<TypeParam>(&format!("{}", g)).unwrap().into())
        .collect()
    }
}