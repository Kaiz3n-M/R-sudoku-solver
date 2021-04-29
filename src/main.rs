const BOARD_SOLVED:[usize; 2] = [10, 10];
fn main() {
    let board: [[i8; 9]; 9] =   [
                                    [4, 0, 1, 2, 9, 0, 0, 7, 5],
                                    [2, 0, 0, 3, 0, 0, 8, 0, 0],
                                    [0, 7, 0, 0, 8, 0, 0, 0, 6],
                                    [0, 0, 0, 1, 0, 3, 0, 6, 2],
                                    [1, 0, 5, 0, 0, 0, 4, 0, 3],
                                    [7, 3, 0, 6, 0, 8, 0, 0, 0],
                                    [6, 0, 0, 0, 2, 0, 0, 3, 0],
                                    [0, 0, 7, 0, 0, 1, 0, 0, 4],
                                    [8, 9, 0, 0, 6, 5, 1, 0, 7]
                                ];
    
    print_board(&board);

    let solved: [[i8; 9]; 9] = solve(board);

    println!("Solved board:");
    print_board(&solved);
}

fn is_valid(board: [[i8; 9]; 9], number: i8, row: usize, column: usize) -> bool {
    for i in 0..9 {
        if board[row][i] == number {
            return false;
        }
    }

    for j in 0..9 {
        if board[j][column] == number {
            return false;
        }
    }

    let box_x: usize = row / 3 * 3;
    let box_y: usize = column / 3 * 3;

    for i in 0..3 {
        for j in 0..3 {
            if board[box_x + i][box_y + j] == number {
                return false;
            }
        }
    }
    return true;
}

fn find_empty(board: [[i8; 9]; 9]) -> [usize; 2] {
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == 0 {
                return [i, j];
            }
        }
    }
    return BOARD_SOLVED;
}

fn solve(board: [[i8; 9]; 9]) -> [[i8; 9]; 9] {
    let mut result = board;

    let empty: [usize; 2] = find_empty(board);

    if empty == BOARD_SOLVED {
        return result;
    } 

    let row: usize = empty[0];
    let column: usize = empty[1];

    for i in 1..10 {
        if is_valid(board, i, row, column) {
            result[row][column] = i;
            result = solve(result);
            if find_empty(result) == BOARD_SOLVED {
                return result;
            }
            
        }
    }
    return board;
}

fn print_board(board: &[[i8; 9]; 9]) {

    for i in 0..9 {
        if i != 0 && i % 3 == 0 {
            println!("----------------------");
        }
        for j in 0..9 {
            if j != 0 && j % 3 == 0 {
                print!("| ");
            }
            if j == 8 {
                print!("{:?}", board[i][j]);
            } else {
                print!("{:?} ", board[i][j]);
            }
        }
        println!("");
    }
}

