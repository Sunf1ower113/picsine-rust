pub fn capitalize_first(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string()
    }
    input[..1].to_uppercase() + &input[1..].to_string()
}

pub fn title_case(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string()
    }
    let w: Vec<String> = input
        .split(" ")
        .map(|v| (v[..1].to_uppercase() + &v[1..].to_string()))
        .collect();
    w.join(" ").to_string()
}

pub fn change_case(input: &str) -> String {
    let mut res = String::from("");
    for l in input.chars() {
        if l.is_lowercase() {
            res.push(l.to_ascii_uppercase());
        } else if l.is_uppercase() {
            res.push(l.to_ascii_lowercase());
        } else {
            res.push(l);
        }
    }
    res
}