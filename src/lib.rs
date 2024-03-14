/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {

    let sentence = sentence.to_lowercase();

    for iterate in 'a'..='z' {
        match sentence.contains(iterate) {
            false => return false,
            true => continue,
        }
    }
    true
}
