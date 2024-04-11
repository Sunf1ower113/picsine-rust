use collect::*;

fn main() {
    let ref mut v = vec![1,2,3,4,5];
    let mut b = v.clone();
    bubble_sort(v);
    println!("{:?}", v);

    b.sort();
    println!("{:?}", b);
}