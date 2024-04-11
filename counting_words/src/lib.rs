use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let w:String = words
        .to_lowercase()
        .replace(" '", " ")
        .replace("' ", " ")
        .chars()
        .filter(|x|
            x.is_alphanumeric() ||
                *x == '\'' ||
                x.is_ascii_whitespace())
        .collect();
    for word in w.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    map
}