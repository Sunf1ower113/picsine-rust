pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    return (c, f64::exp(c as f64), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let nums: Vec<String>= a
        .split(" ")
        .map(|x| f64::exp(x.parse::<f64>()
            .unwrap()).to_string())
        .collect();
    return (a, nums.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let res: Vec<f64> = b.iter().map(|x| (*x as f64).abs().ln()).collect();
    return (b, res)
}