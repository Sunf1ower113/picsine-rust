
pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: String = String::new();
    let s: Vec<&str> = phrase.split(" ").collect();
    for i in 1 .. s.len()+1 {
        let ind = i.to_string();
        for word in &s {
            if word.contains(&ind) {
                let _ = &res.push_str(&word.replace(&ind, ""));
                if i != s.len() {
                    let _ = &res.push_str(" ");
                }
                break
            }
        }
    }
    res
}


