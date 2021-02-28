use toml::Value;

use rand::prelude::*;

use std::fs;
use std::io;
use std::collections::HashMap;

#[derive(Clone)]
struct Language {
    sounds: Vec<String>,
    av_sound_len: i64,
    // key = input word, value = generated word
    words: HashMap<String, String>,
    rng: StdRng
}

impl Language {
    // Most of its job is just parsing the toml file.
    fn new(sound_file: &str) -> Language {
        let toml = fs::read_to_string(sound_file).unwrap().parse::<Value>().unwrap();
        let sounds = toml["sounds"].as_array().unwrap();
        let av_sound_len = &toml["av_sound_len"];

        let av_sound_len = match av_sound_len {
            toml::Value::Integer(v) => v,
            _ => panic!("Invalid av_sound_len value found in TOML file"),
        };

        let mut string_sounds = Vec::new();
        for s in sounds {
            match s {
                toml::Value::String(st) => string_sounds.push(st.clone()),
                _ => panic!("Invalid sound value found in TOML file")
            }
        }

        println!("sounds: {:?}", string_sounds);

        Language {
            sounds: string_sounds,
            av_sound_len: *av_sound_len,
            words: HashMap::new(),
            rng: StdRng::seed_from_u64(4254),
        }
    }

    fn gen_sentance(&mut self, sentance: &str) -> String {
        let old_sentance: Vec<&str> = sentance.trim().split(" ").collect();
        let mut new_sentance: Vec<String> = Vec::new();

        for word in old_sentance.clone() {
            if self.words.contains_key(word) {
                println!("Word '{}' was already in HashMap", word);
                new_sentance.push(self.words[word].clone());
            } else {
                let new_word = self.gen_word(word.to_string());
                self.words.insert(word.to_string(), new_word.clone());
                println!("Word '{}' now equals '{}'", word, new_word);
                new_sentance.push(new_word);
            }
        }

        println!("Old: {:?}\nNew: {:?}", old_sentance, new_sentance);
        let mut end_sentance = String::new();
        for word in new_sentance {
            end_sentance = format!("{}{} ", end_sentance, word);
        }
        println!("{}", end_sentance);
        end_sentance
    }

    fn gen_word(&mut self, word: String) -> String {
        let mut tmp: Vec<isize> = Vec::new();
        for i in 2..6 {
            tmp.push(i);
        }

        let new_length = bias_vec(&mut self.rng, tmp);

        let mut new_word = "".to_string();
        for _ in 0..new_length {
            new_word = new_word + &rand_vec(&mut self.rng, self.sounds.clone());
        }
        new_word
    }
}

// Get a random item from a vector
fn rand_vec<T: Clone>(rng: &mut StdRng, vec: Vec<T>) -> T {
    let i = rng.gen_range(0, vec.len());
    vec[i].clone()
}

// Get random item from vector, TODO: biased towards earlier items in vec
fn bias_vec<T: Clone>(rng: &mut StdRng, vec: Vec<T>) -> T {
    let i = rng.gen_range(0, vec.len());
    vec[i].clone()
}

fn main() {
    let mut lang = Language::new("eng-sounds.toml");

    let mut sentance = String::new();
    io::stdin().read_line(&mut sentance).expect("Error reading user input");

    lang.gen_sentance(&sentance);
}