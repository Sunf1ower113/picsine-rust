use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max = -2 << 31;
    for (_, v) in &h {
        if max < *v {
            max = *v;
        }
    }
    return max
}