pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut v: Vec<String> = vec![];
    for name in names {
        let i: usize = name.find(' ').unwrap();
        v.push(format!("{}. {}.", name.as_bytes()[0] as char, name.as_bytes()[i+1] as char))
    }
    return v
}