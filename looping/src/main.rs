use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";

    let mut count = 1;
    loop {
        println!("{riddle}");
        let mut buffer = String::new(); 
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim() != answer {
            count += 1;
        } else {
            break;
        }
    }
    println!("Number of trials: {count}")
}
