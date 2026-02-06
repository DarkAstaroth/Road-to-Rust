use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    println!("{:?}", input.chars().rev().collect::<String>());
    println!("{:?}", input.graphemes(true).rev().collect::<String>());

    if !input.is_empty() {
        input.graphemes(true).rev().collect::<String>()
    } else {
        input.to_string()
    }
}
