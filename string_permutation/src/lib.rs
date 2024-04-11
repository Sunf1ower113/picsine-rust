use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false
    }
    let mut m1: HashMap<char, u32> = HashMap::new();
    let mut m2: HashMap<char, u32> = HashMap::new();
    for word in s1.chars() {
        let count = m1.entry(word).or_insert(0);
        *count += 1;
    }
    for word in s2.chars() {
        let count = m2.entry(word).or_insert(0);
        *count += 1;
    }
    for (k1, v) in m1.iter() {
        let k2 = m2.get(k1).copied().unwrap_or(0);
        if k2 == 0 {
            return false
        }
        if *v != k2 {
            return  false
        }
    }
    return true
}