use std::cmp::max;

pub fn delete_and_backspace(s: &mut String) {
    let mut stack = String::from("");
    let mut count = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '-' {
            if stack.len() != 0 {
                stack.pop();
            }
        } else if c == '+' {
            if i != s.len()-1 {
                count += 1;
            }
        } else {
            if count != 0 {
                count -= 1;
            } else {
                stack.push(c);
            }
        }
    }
    *s = stack;
}

pub fn do_operations(v: &mut Vec<String>) {

    let mut stack: Vec<String> = vec![];
    for (i, s) in v.iter().enumerate() {
        let mut val: i32 = 0;
        let mut t: Vec<String> = s.split('+').map(|x| x.to_string()).collect();
        if t.len() == 2 {
            val = t.pop().unwrap().parse::<i32>().unwrap();
            val += t.pop().unwrap().parse::<i32>().unwrap();
        } else {
            let mut t: Vec<String> = s.split('-').map(|x| x.to_string()).collect();
            if t.len() == 2 {
                val -= t.pop().unwrap().parse::<i32>().unwrap();
                val += t.pop().unwrap().parse::<i32>().unwrap();
            } else {
                val = t.pop().unwrap().parse::<i32>().unwrap();
            }
        }
        stack.push(val.to_string())
    }
    *v = stack
}