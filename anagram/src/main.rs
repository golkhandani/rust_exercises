use anagram::*;

fn main() {
    let word = "ΑΒΓ";
    let inputs = &["ΒΓΑ", "ΒΓΔ", "γβα", "αβγ"];
    let output = anagrams_for(word, inputs);

    print!("input-> {} -- output -> {:#?}", word.to_lowercase(), output)
}
