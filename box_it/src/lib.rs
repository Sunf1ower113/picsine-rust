pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let v: Vec<u32> = s.split(" ").map(|x| {

        if x[x.len() - 1..] == "k".to_string() {

            (x[..x.len() - 1].parse::<f64>().unwrap() * 1000.0) as u32
        } else { x.parse::<u32>().unwrap() }
    }).collect();
    Box::new(v)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}