pub fn sum(a: &[i32]) -> i32 {
    let mut sum = 0;
    for n in a.iter() {
        sum += n;
    }
    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}