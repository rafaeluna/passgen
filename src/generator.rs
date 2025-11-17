//! Password generators
//! Generated with ChatGPT

use rand::seq::SliceRandom;
use rand::{Rng, rngs::OsRng};

use crate::words::WORDLIST;

/// Generate default passphrase (4–5 words, random caps, +digits).
pub fn generate_default() -> String {
    let mut rng = OsRng;

    // choose 4 random words
    let mut words: Vec<&str> = (0..4)
        .map(|_| *WORDLIST.choose(&mut rng).unwrap())
        .collect();

    // randomly capitalize 1–4 words
    let caps_count = rng.gen_range(1..=4);
    let mut idxs = [0, 1, 2, 3];
    idxs.shuffle(&mut rng);

    for &i in &idxs[..caps_count] {
        words[i] = capitalize_first(words[i]);
    }

    // add 2 digits
    let d1: u8 = rng.gen_range(0..10);
    let d2: u8 = rng.gen_range(0..10);

    format!("{}-{}{}", words.join("-"), d1, d2)
}

fn capitalize_first(s: &str) -> &'static str {
    let (first, rest) = s.split_at(1);
    Box::leak(format!("{}{}", first.to_ascii_uppercase(), rest).into_boxed_str())
}

const CHARSET_FULL: &[u8] = br#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*(){}[]<>?=+-_.,:;"'|~"#;
const CHARSET_ALNUM: &[u8] = br"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn generate_chars(n: usize) -> String {
    generate_from_charset(n, CHARSET_FULL)
}

pub fn generate_chars_no_special(n: usize) -> String {
    generate_from_charset(n, CHARSET_ALNUM)
}

fn generate_from_charset(len: usize, charset: &[u8]) -> String {
    let mut rng = OsRng;
    let mut out = String::with_capacity(len);

    for _ in 0..len {
        let idx = rng.gen_range(0..charset.len());
        out.push(charset[idx] as char);
    }

    out
}
