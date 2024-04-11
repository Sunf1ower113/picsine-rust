pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
   
    // for (i, x) in vec.iter().enumerate() {
    //     if i == index {
    //         return x.to_string();
    //     }
    // }
    if index < vec.len() {
         return vec[index].to_string();
    } 
    let a = String::new();
    return a;
}