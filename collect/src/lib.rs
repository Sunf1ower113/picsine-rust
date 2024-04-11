pub fn bubble_sort(vec: &mut Vec<i32>) {
    for i in 0..vec.len()-1 {
        for j in i+1..vec.len() {
            if vec[i] > vec[j] {
                vec.swap(i, j);
            }
        }
    }
}