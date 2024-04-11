use string_permutation::*;

fn main() {
    let word = "codde";
    let word1 = "deeco";
    println!(
        "Is `{}` a permutation of `{}`? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}