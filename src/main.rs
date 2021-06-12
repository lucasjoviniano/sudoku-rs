use std::usize;

fn possible(sudoku: &[[usize; 9]; 9], i: usize, j: usize, n: usize) -> bool {
    let (ib, jb) = ((i / 3) * 3, (j / 3) * 3);
    let block = (jb..jb + 3).all(|x| (ib..ib+3).all(|y| sudoku[x][y] != n));
    let row_col = (0..9).all(|x| (sudoku[j][x] != n) && (sudoku[x][i] != n));

    block && row_col
}

fn print_sudoku(sudoku: &[[usize; 9]; 9]) {
    for row in sudoku.iter() {
        println!("{:?}", row)
    }
    println!("---------------------------------");
}

fn solve(sudoku: &mut [[usize; 9]; 9]) -> bool {
    for j in 0..9 {
        for i in 0..9 {
            if sudoku[j][i] == 0 {
                for n in 1..=9 {
                    if possible(sudoku, i, j, n) {
                        sudoku[j][i] = n;
                        if solve(sudoku) {
                            print_sudoku(&sudoku)
                        }
                        sudoku[j][i] = 0;
                    }
                }

                return false;
            }
        }
    }

    true
}

fn main() {
    let mut sudoku: [[usize; 9]; 9] = [
        [0,0,0, 0,5,8, 4,2,7],
        [0,0,8, 0,0,0, 0,1,0],
        [0,0,0, 0,0,7, 6,0,0],
    
        [0,5,4, 2,9,0, 8,7,1],
        [7,6,0, 0,0,0, 0,4,9],
        [0,1,0, 0,0,0, 0,5,0],
    
        [0,2,0, 3,7,0, 0,9,8],
        [4,0,7, 8,0,5, 1,0,0],
        [5,8,3, 0,2,0, 0,6,4],
    ];

    solve(&mut sudoku);
}
