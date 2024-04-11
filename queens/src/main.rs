use queens::*;
fn main() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? => {}",
        white_queen.can_attack(&black_queen)
    );

    let white_queen = Queen::new(ChessPosition::new(1, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? => {}",
        white_queen.can_attack(&black_queen)
    );
    let white_queen = Queen::new(ChessPosition::new(3, 4).unwrap());
    let black_queen = Queen::new(ChessPosition::new(6, 7).unwrap());

    println!(
        "Is it possible for the queens to attack each other? => {}",
        white_queen.can_attack(&black_queen)
    );
    let white_queen = Queen::new(ChessPosition::new(0, 3).unwrap());
    let black_queen = Queen::new(ChessPosition::new(7, 4).unwrap());

    println!(
        "Is it possible for the queens to attack each other? => {}",
        white_queen.can_attack(&black_queen)
    );
    // let white_queen = Queen::new(ChessPosition::new(1, 2).unwrap());
    // let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
    //
    // println!(
    //     "Is it possible for the queens to attack each other? => {}",
    //     white_queen.can_attack(&black_queen)
    // );
    // let white_queen = Queen::new(ChessPosition::new(1, 2).unwrap());
    // let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
    //
    // println!(
    //     "Is it possible for the queens to attack each other? => {}",
    //     white_queen.can_attack(&black_queen)
    // );
}