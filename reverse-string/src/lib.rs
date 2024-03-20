use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let res = input.graphemes(true).rev().collect::<String>();
    return res;
}
