use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_sorted: Vec<String> = word_lower.graphemes(true).map(|s| s.to_string()).collect();
    word_sorted.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let candidate_lower = candidate.to_lowercase();

            if candidate_lower == word_lower {
                return false;
            }

            if candidate_lower.graphemes(true).count() != word_sorted.len() {
                return false;
            }

            let mut candidate_sorted: Vec<String> = candidate_lower
                .graphemes(true)
                .map(|s| s.to_string())
                .collect();
            candidate_sorted.sort_unstable();

            candidate_sorted == word_sorted
        })
        .copied()
        .collect()
}
