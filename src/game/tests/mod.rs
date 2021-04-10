use super::*;

fn assert_cell(lhs: Cell, rhs: Cell) {
    match (lhs, rhs) {
        (Cell::Value(lhs_value), Cell::Value(rhs_value)) => {
            // Values should match.
            assert_eq!(lhs_value, rhs_value);
        }
        (Cell::Value(lhs_value), Cell::Options(rhs_opts)) => {
            // Options should contain the eventual Value
            assert!(rhs_opts.has(lhs_value as usize));
        }
        (Cell::Options(lhs_opts), Cell::Value(rhs_value)) => {
            // Options should contain the eventual Value
            assert!(lhs_opts.has(rhs_value as usize));
        }
        (Cell::Options(lhs_opts), Cell::Options(rhs_opts)) => {
            // Options should have some commonality.
            assert_ne!(rhs_opts.intersect(lhs_opts).count(), 0);
        }
    }
}

fn assert_board(lhs: Board, rhs: Board) {
    for ridx in 0..9 {
        for cidx in 0..9 {
            let lhs_cell = lhs.cells[ridx][cidx];
            let rhs_cell = rhs.cells[ridx][cidx];

            assert_cell(lhs_cell, rhs_cell);
        }
    }
}

#[test]
fn check_bad_board() {
    if let Ok(b) = Board::new([
        [10, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 7, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 9],
    ]) {
        panic!("wanted error due to bad value: 10, got: {:#?}", b)
    }
}

const GOOD_BOARD_ZEROS: &str = "\
005300000
800000020
070010500
400005300
010070006
003200080
060500009
004000030
000009700";

// We use a zero because the "\ seems to trim leading whitespace.
const GOOD_BOARD_SPACES: &str = "\
0 53     
8      2 
 7  1 5  
4    53  
 1  7   6
  32   8 
 6 5    9
  4    3 
     97  ";

const BAD_BOARD_CHAR: &str = "\
00a300000
800000020
070010500
400005300
010070006
003200080
060500009
004000030
000009700";

const BAD_BOARD_TOO_FEW_ROWS: &str = "\
005300000
001200000";

const BAD_BOARD_ROW_TOO_SHORT: &str = "\
005300
800000020
070010500
400005300
010070006
003200080
060500009
004000030
000009700";

#[test]
fn check_parse() {
    Board::parse(GOOD_BOARD_ZEROS.to_string()).expect("good board zeros");
    Board::parse(GOOD_BOARD_SPACES.to_string()).expect("good board spaces");
    if let Ok(b) = Board::parse(BAD_BOARD_CHAR.to_string()) {
        panic!("wanted error due to bad char: 'a', got: {:#?}", b)
    }
    if let Ok(b) = Board::parse(BAD_BOARD_TOO_FEW_ROWS.to_string()) {
        panic!("wanted error due to too few rows, got: {:#?}", b)
    }
    if let Ok(b) = Board::parse(BAD_BOARD_ROW_TOO_SHORT.to_string()) {
        panic!("wanted error due to too few rows, got: {:#?}", b)
    }
}

#[test]
fn check_equality() {
    // Programatically define a board with the diagonal
    // set to their row/col index (1-based)
    let as_code = {
        let mut b = Board::new([[0; 9]; 9]).expect("building board literal");
        for diag in 0..9 {
            b.set(diag, diag, Cell::Value(diag + 1))
                .expect("unexpected error setting up test");
        }
        b
    };
    as_code.check().expect("Failed to validate board.");

    // Define the same as above as a literal.
    let as_literal = Board::new([
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 7, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 9],
    ])
    .expect("building board literal");
    as_literal.check().expect("Failed to validate board.");
    assert_eq!(as_code, as_literal);

    // If we leave things as unspecified everywhere, the comparison should fail.
    let b = Board::new([[0; 9]; 9]).expect("building board literal");

    assert_ne!(b, as_literal);
}

#[test]
fn check_iterators() {
    let b = Board::new([
        [0, 4, 0, 7, 0, 1, 0, 0, 3],
        [1, 3, 0, 0, 0, 0, 0, 4, 0],
        [8, 0, 0, 0, 0, 0, 9, 5, 0],
        [0, 8, 0, 3, 0, 2, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 0, 0],
        [9, 0, 0, 5, 0, 6, 0, 3, 0],
        [0, 7, 1, 0, 0, 0, 0, 0, 9],
        [0, 9, 0, 0, 0, 0, 0, 2, 4],
        [3, 0, 0, 4, 0, 8, 0, 7, 0],
    ])
    .expect("building board literal");

    // Check row iterator
    for ridx in 0..9 {
        let mut cidx = 0;
        for got in b.row(ridx) {
            let want = b.cells[ridx][cidx];
            assert_eq!(
                got, want,
                "Saw a mismatch at ({}, {}) = {:#?}, wanted {:#?}",
                ridx, cidx, got, want
            );
            cidx = cidx + 1;
        }
    }

    // Check col iterator
    for cidx in 0..9 {
        let mut ridx = 0;
        for got in b.col(cidx) {
            let want = b.cells[ridx][cidx];
            assert_eq!(
                got, want,
                "Saw a mismatch at ({}, {}) = {:#?}, wanted {:#?}",
                ridx, cidx, got, want
            );
            ridx = ridx + 1;
        }
    }

    // Check subsquare iterator
    for ss_cidx in 0..3 {
        for ss_ridx in 0..3 {
            let mut idx = 0;
            for got in b.subsquare(ss_ridx, ss_cidx) {
                let want = b.cells[ss_ridx * 3 + idx / 3][ss_cidx * 3 + idx % 3];
                assert_eq!(
                    got, want,
                    "Saw a mismatch at ({}, {}) offset {} = {:#?}, wanted {:#?}",
                    ss_ridx, ss_cidx, idx, got, want
                );
                idx = idx + 1;
            }
        }
    }
}

#[test]
fn check_solve_fails() {
    let mut input = Board::new([
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 9, 8, 7, 6, 5, 4, 3, 2],
        [2, 0, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 0, 0],
        [6, 0, 0, 0, 0, 0, 0, 0, 0],
        [7, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .expect("building board literal");
    input.check().expect("Failed to validate board.");

    // This should close in a single pass of the solver.
    match input.solve_one() {
        Ok(_) => {
            panic!("Expected failure solving board, but got: {:#?}", input)
        }
        _ => {}
    }
}

#[test]
fn check_solve_super_easy() {
    let mut input = Board::new([
        [0, 4, 9, 7, 5, 1, 8, 6, 3],
        [1, 0, 5, 8, 6, 9, 7, 4, 2],
        [8, 6, 0, 2, 4, 3, 9, 5, 1],
        [7, 8, 6, 0, 1, 2, 4, 9, 5],
        [5, 2, 3, 9, 0, 4, 6, 1, 7],
        [9, 1, 4, 5, 7, 0, 2, 3, 8],
        [4, 7, 1, 6, 2, 5, 0, 8, 9],
        [6, 9, 8, 1, 3, 7, 5, 0, 4],
        [3, 5, 2, 4, 9, 8, 1, 7, 0],
    ])
    .expect("building board literal");
    input.check().expect("Failed to validate board.");

    // This should close in a single pass of the solver.
    let (options, changed) = input.solve_one().expect("error during solve_one");
    assert_eq!(changed, true);
    assert_eq!(options, 0);

    let solution = Board::new([
        [2, 4, 9, 7, 5, 1, 8, 6, 3],
        [1, 3, 5, 8, 6, 9, 7, 4, 2],
        [8, 6, 7, 2, 4, 3, 9, 5, 1],
        [7, 8, 6, 3, 1, 2, 4, 9, 5],
        [5, 2, 3, 9, 8, 4, 6, 1, 7],
        [9, 1, 4, 5, 7, 6, 2, 3, 8],
        [4, 7, 1, 6, 2, 5, 3, 8, 9],
        [6, 9, 8, 1, 3, 7, 5, 2, 4],
        [3, 5, 2, 4, 9, 8, 1, 7, 6],
    ])
    .expect("building board literal");
    solution.check().expect("Failed to validate board.");

    assert_eq!(input, solution);
}

#[test]
fn check_solve_super_hard() {
    let mut input = Board::new([
        [0, 4, 0, 7, 0, 1, 0, 0, 3],
        [1, 3, 0, 0, 0, 0, 0, 4, 0],
        [8, 0, 0, 0, 0, 0, 9, 5, 0],
        [0, 8, 0, 3, 0, 2, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 0, 0],
        [9, 0, 0, 5, 0, 6, 0, 3, 0],
        [0, 7, 1, 0, 0, 0, 0, 0, 9],
        [0, 9, 0, 0, 0, 0, 0, 2, 4],
        [3, 0, 0, 4, 0, 8, 0, 7, 0],
    ])
    .expect("building board literal");
    input.check().expect("Failed to validate board.");

    let solution = Board::new([
        [2, 4, 9, 7, 5, 1, 8, 6, 3],
        [1, 3, 5, 8, 6, 9, 7, 4, 2],
        [8, 6, 7, 2, 4, 3, 9, 5, 1],
        [7, 8, 6, 3, 1, 2, 4, 9, 5],
        [5, 2, 3, 9, 8, 4, 6, 1, 7],
        [9, 1, 4, 5, 7, 6, 2, 3, 8],
        [4, 7, 1, 6, 2, 5, 3, 8, 9],
        [6, 9, 8, 1, 3, 7, 5, 2, 4],
        [3, 5, 2, 4, 9, 8, 1, 7, 6],
    ])
    .expect("building board literal");
    solution.check().expect("Failed to validate board.");

    // Run the solver a single iteration
    input.solve_one().expect("error finding solution");
    // These test the paths thru assert_board:
    assert_board(input, input);
    assert_board(input, solution);
    assert_board(solution, input);

    // Run the solver to completion.
    input.check().expect("Failed to validate board.");
    input.solve().expect("error finding solution");

    assert_board(input, solution);
    assert_eq!(input, solution);
}

#[test]
fn check_solve_hardest() {
    let solution = Board::new([
        [1, 4, 5, 3, 2, 7, 6, 9, 8],
        [8, 3, 9, 6, 5, 4, 1, 2, 7],
        [6, 7, 2, 9, 1, 8, 5, 4, 3],
        [4, 9, 6, 1, 8, 5, 3, 7, 2],
        [2, 1, 8, 4, 7, 3, 9, 5, 6],
        [7, 5, 3, 2, 9, 6, 4, 8, 1],
        [3, 6, 7, 5, 4, 2, 8, 1, 9],
        [9, 8, 4, 7, 6, 1, 2, 3, 5],
        [5, 2, 1, 8, 3, 9, 7, 6, 4],
    ])
    .expect("building board literal");
    solution.check().expect("Failed to validate board.");

    // This is from:
    // https://punemirror.indiatimes.com/pune/cover-story/worlds-toughest-sudoku-is-here-can-you-crack-it/articleshow/32299679.cms
    let mut input = Board::new([
        [0, 0, 5, 3, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 2, 0],
        [0, 7, 0, 0, 1, 0, 5, 0, 0],
        [4, 0, 0, 0, 0, 5, 3, 0, 0],
        [0, 1, 0, 0, 7, 0, 0, 0, 6],
        [0, 0, 3, 2, 0, 0, 0, 8, 0],
        [0, 6, 0, 5, 0, 0, 0, 0, 9],
        [0, 0, 4, 0, 0, 0, 0, 3, 0],
        [0, 0, 0, 0, 0, 9, 7, 0, 0],
    ])
    .expect("building board literal");
    input.check().expect("Failed to validate board.");
    println!("input: {:#?}", input);

    // Run the solver.
    input.solve().expect("error finding solution");

    assert_board(input, solution);
    assert_eq!(input, solution);
}
