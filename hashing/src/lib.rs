use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let sum = list.iter().sum::<i32>();
    (sum as f64) / (list.len() as f64)
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut s = list.clone();
    s.sort();
    if s.len() % 2 == 0 {
        return (s[s.len()/2-1] + s[s.len()/2]) / 2
    }
    return s[s.len()/2]
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut res = 0;
    let mut max_count = 0;
    let mut m: HashMap<i32, u32> = HashMap::new();
    for &i in list.iter() {
        let count = m.entry(i).or_insert(0);
        *count += 1;
        if max_count < *count {
            res = i;
            max_count = *count;
        }
    }
    return res
}