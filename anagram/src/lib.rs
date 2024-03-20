use std::{char, collections::HashSet};

fn sorted_word(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort();
    return chars;
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    /*
     Cloning: The .cloned() method is called on the iterator,
     which clones each element of the iterator.
     This is necessary because the filter produces references to elements
     of possible_anagrams, and
     we want to convert these references to owned values (String instead of &str).
    */
    possible_anagrams
        .iter()
        .filter(|anagram| {
            anagram.to_lowercase() != word.to_lowercase()
                && sorted_word(anagram) == sorted_word(word)
        })
        .cloned()
        .collect()
}
