use std::cmp::min;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len()+1;
    let n = target.len()+1;
    let mut mat: Vec<Vec<usize>> = vec![vec![0; m+1]; n+1];
    for i in 1..n+1 {
        mat[i][0] = i;
    }
    for j in 1..m {
        mat[0][j] = j;
    }
    let mut cost = 0;
    for j in 1..n {
        for i in 1..m {
            if source.chars().nth(i-1).unwrap() == target.chars().nth(j-1).unwrap() {
                cost = 0;
            } else {
                cost = 1;
            }
            mat[j][i] = min(mat[j-1][i] + 1, min(mat[j][i-1] + 1, mat[j-1][i-1] + cost));
        }
    }
    return mat[n-1][m-1]
}