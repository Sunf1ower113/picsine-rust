const TIE_MSG: &str = "tie";
const W_O_MSG: &str = "player O won";
const W_X_MSG: &str = "player X won";

pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) {
        return W_O_MSG.to_string();
    }
    if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) {
        return W_X_MSG.to_string();
    }
    return TIE_MSG.to_string();
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    return table[0][0] == player && table[1][1] == player && table[2][2] == player ||
        table[0][2] == player && table[1][1] == player && table[2][0] == player
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    return table[0][0] == player && table[0][1] == player && table[0][2] == player ||
        table[1][0] == player && table[1][1] == player && table[1][2] == player ||
        table[2][0] == player && table[2][1] == player && table[2][2] == player
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    return table[0][0] == player && table[1][0] == player && table[2][0] == player ||
        table[0][1] == player && table[1][1] == player && table[2][1] == player ||
        table[0][2] == player && table[1][2] == player && table[2][2] == player
}