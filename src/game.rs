use std::fmt;
use std::result::Result;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Value(u8),    // Holds the actual value.
    Options(u16), // Holds a bit-mask of availabile options
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Value(v) => return f.write_fmt(format_args!("\t{}", v)),
            Cell::Options(opts) => {
                f.write_str("\t{")?;
                let mut comma = "";
                for idx in 0..9 {
                    if (1 << idx) & opts != 0 {
                        f.write_fmt(format_args!("{}{}", comma, idx + 1))?;
                        comma = ",";
                    }
                }
                f.write_str("}")
            }
        }
    }
}

pub const ALL: u16 = (1 << 9) - 1;

#[derive(Copy, Clone, PartialEq)]
pub struct Board {
    pub cells: [[Cell; 9]; 9],
}
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n")?;
        for row in 0..9 {
            f.write_fmt(format_args!("{:?}\n", self.cells[row]))?;
        }
        Ok(())
    }
}

// These are different "views" of the board.
mod views {
    // Iterate over a particular Row of the board.
    pub struct Row<'a> {
        board: &'a super::Board,
        row_index: usize,
        col_index: usize,
    }
    impl Row<'_> {
        pub fn new(b: &super::Board, idx: usize) -> Row {
            Row {
                board: b,
                row_index: idx,
                col_index: 0,
            }
        }
    }
    impl<'a> Iterator for Row<'a> {
        type Item = super::Cell;

        fn next(&mut self) -> Option<super::Cell> {
            match self.col_index {
                9 => None,
                _ => {
                    let c = self.board.cells[self.row_index][self.col_index];
                    self.col_index += 1;
                    Some(c)
                }
            }
        }
    }

    // Iterate over a particular Column of the board.
    pub struct Column<'a> {
        board: &'a super::Board,
        col_index: usize,
        row_index: usize,
    }
    impl Column<'_> {
        pub fn new(b: &super::Board, idx: usize) -> Column {
            Column {
                board: b,
                col_index: idx,
                row_index: 0,
            }
        }
    }
    impl<'a> Iterator for Column<'a> {
        type Item = super::Cell;

        fn next(&mut self) -> Option<super::Cell> {
            match self.row_index {
                9 => None,
                _ => {
                    let c = self.board.cells[self.row_index][self.col_index];
                    self.row_index += 1;
                    Some(c)
                }
            }
        }
    }

    // Iterate over a particular SubSquare of the board.
    pub struct SubSquare<'a> {
        board: &'a super::Board,
        base_col: usize,
        base_row: usize,
        index: usize,
    }
    impl SubSquare<'_> {
        pub fn new(b: &super::Board, ss_ridx: usize, ss_cidx: usize) -> SubSquare {
            SubSquare {
                board: b,
                base_row: ss_ridx * 3,
                base_col: ss_cidx * 3,
                index: 0,
            }
        }
    }
    impl<'a> Iterator for SubSquare<'a> {
        type Item = super::Cell;

        fn next(&mut self) -> Option<super::Cell> {
            match self.index {
                9 => None,
                _ => {
                    // Every 3 we wrap around to the next row.
                    let (div, modulo) = (self.index / 3, self.index % 3);
                    let c = self.board.cells[self.base_row + div][self.base_col + modulo];
                    self.index += 1;
                    Some(c)
                }
            }
        }
    }

    pub fn check(it: impl Iterator<Item = super::Cell>) -> Result<(), String> {
        let mut mask = 0;
        for elt in it {
            if let super::Cell::Value(v) = elt {
                let bit = 1 << (v - 1);
                if mask & bit != 0 {
                    return Err(format!("Multiple {} seen", v));
                }
                mask |= bit;
            }
        }
        Ok(())
    }

    pub fn mask(it: impl Iterator<Item = super::Cell>) -> u16 {
        let mut mask = super::ALL;
        for elt in it {
            if let super::Cell::Value(v) = elt {
                mask ^= 1 << (v - 1);
            }
        }
        mask
    }

    pub fn frequency(it: impl Iterator<Item = super::Cell>, value: u8) -> u8 {
        let mask = 1 << (value - 1);
        let mut freq = 0;

        for elt in it {
            match elt {
                super::Cell::Value(v) => {
                    if v == value {
                        // There should only be one, but the availability masks may be slightly stale.
                        freq += 1;
                    }
                }
                super::Cell::Options(opts) => {
                    if mask & opts != 0 {
                        freq += 1;
                    }
                }
            };
        }
        freq
    }
}

impl Board {
    fn set(&mut self, row: usize, col: usize, value: Cell) -> Result<(), String> {
        self.cells[row][col] = value;
        self.check()
    }

    fn row(&self, idx: usize) -> views::Row {
        views::Row::new(self, idx)
    }

    fn col(&self, idx: usize) -> views::Column {
        views::Column::new(self, idx)
    }

    fn subsquare(&self, ridx: usize, cidx: usize) -> views::SubSquare {
        views::SubSquare::new(self, ridx, cidx)
    }

    pub fn check(&self) -> Result<(), String> {
        // Check each row
        for idx in 0..9 {
            views::check(self.row(idx))?;
        }

        // Check each column
        for idx in 0..9 {
            views::check(self.col(idx))?;
        }

        // Check each subsquare
        for idx in 0..9 {
            views::check(self.subsquare(idx / 3, idx % 3))?;
        }

        // If everything checks out, then we are good!
        Ok(())
    }

    fn solve_one(&mut self) -> Result<(u32, bool), String> {
        let mut options = 0;
        let mut changed = false;

        // First we check whether any square's available options consist of a single value.
        for ridx in 0..9 {
            for cidx in 0..9 {
                if let Cell::Options(og_opts) = self.cells[ridx][cidx] {
                    let opts = og_opts
                        & views::mask(self.row(ridx))
                        & views::mask(self.col(cidx))
                        & views::mask(self.subsquare(ridx / 3, cidx / 3));
                    if opts == 0 {
                        // If there are no options, then something went wrong.
                        return Err(format!(
                            "There are no remaining options for {}, {}",
                            ridx, cidx
                        ));
                    } else if opts & (opts - 1) == 0 {
                        // If it's a power of two, then there's only one option.
                        self.set(ridx, cidx, Cell::Value((opts.trailing_zeros() + 1) as u8))?;
                        changed = true;
                    } else {
                        self.set(ridx, cidx, Cell::Options(opts))?;
                        if og_opts != opts {
                            changed = true;
                        }
                        options += 1; // This cell remains an Options.
                    }
                }
            }
        }
        self.check()?;

        // Second we check whether any square is the only square in a range that could hold a given value.
        for ridx in 0..9 {
            for cidx in 0..9 {
                if let Cell::Options(opts) = self.cells[ridx][cidx] {
                    let mut value = 1;
                    let mut shifter = opts;
                    while shifter != 0 {
                        if (shifter & 1 != 0)
                            && (views::frequency(self.row(ridx), value) == 1
                                || views::frequency(self.col(cidx), value) == 1
                                || views::frequency(self.subsquare(ridx / 3, cidx / 3), value) == 1)
                        {
                            self.set(ridx, cidx, Cell::Value(value))?;
                            changed = true;
                            options -= 1; // We counted this cell above, but it's now concrete so remove it.
                            break;
                        }
                        value += 1;
                        shifter >>= 1;
                    }
                }
            }
        }
        self.check()?;

        Ok((options, changed))
    }

    pub fn solve(&mut self) -> Result<(), String> {
        for _ in 1..1000 {
            match self.solve_one() {
                Ok((options, changed)) => {
                    // If there are no options left, then we have completely solved the puzzle!
                    if options == 0 {
                        return Ok(());
                    }

                    // If the puzzle changed this iteration, then we're still making progress.  Keep going!
                    if changed {
                        continue;
                    }

                    // There are options left, but we have stalled.  Find one of the remaining
                    // options and try to recursively solve a copy of the board for each option
                    // until one succeeds.
                    // We pick the cell with the fewest options as our speculation candidate.
                    let (mut candidate_rdx, mut candidate_cdx, mut count) = (0, 0, 9);
                    for ridx in 0..9 {
                        for cidx in 0..9 {
                            if let Cell::Options(opts) = self.cells[ridx][cidx] {
                                if opts.count_ones() <= count {
                                    candidate_rdx = ridx;
                                    candidate_cdx = cidx;
                                    count = opts.count_ones();
                                }
                            }
                        }
                    }
                    if let Cell::Options(opts) = self.cells[candidate_rdx][candidate_cdx] {
                        let mut value = 1;
                        let mut shifter = opts;

                        while shifter != 0 {
                            if shifter & 1 != 0 {
                                // Create a copy of the board with which we will speculate the value of this cell.
                                let mut speculator = *self;
                                speculator.set(candidate_rdx, candidate_cdx, Cell::Value(value))?;
                                // Try to recursively solve the board.
                                if speculator.solve().is_ok() {
                                    self.cells = speculator.cells;
                                    return Ok(());
                                }
                            }
                            value += 1;
                            shifter >>= 1;
                        }
                        return Err("All options lead to failure!".to_string());
                    }
                }
                Err(s) => {
                    return Err(s);
                }
            }
        }
        Err("Solution did not close in 1000 iterations".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANY: Cell = Cell::Options(ALL);

    // Allow for shorter-hand test literals.
    fn v(x: u8) -> Cell {
        Cell::Value(x)
    }

    fn assert_cell(lhs: &Cell, rhs: &Cell) {
        match (lhs, rhs) {
            (Cell::Value(lhs_value), Cell::Value(rhs_value)) => {
                // Values should match.
                assert_eq!(lhs_value, rhs_value);
            }
            (Cell::Value(lhs_value), Cell::Options(rhs_opts)) => {
                // Options should contain the eventual Value
                assert_ne!(rhs_opts & 1 << (lhs_value - 1), 0);
            }
            (Cell::Options(lhs_opts), Cell::Value(rhs_value)) => {
                // Options should contain the eventual Value
                assert_ne!(lhs_opts & 1 << (rhs_value - 1), 0);
            }
            (Cell::Options(lhs_opts), Cell::Options(rhs_opts)) => {
                // Options should have some commonality.
                assert_ne!(rhs_opts & lhs_opts, 0);
            }
        }
    }

    fn assert_board(lhs: Board, rhs: Board) {
        for ridx in 0..9 {
            for cidx in 0..9 {
                let lhs_cell = lhs.cells[ridx][cidx];
                let rhs_cell = rhs.cells[ridx][cidx];

                assert_cell(&lhs_cell, &rhs_cell);
            }
        }
    }

    #[test]
    fn check_equality() {
        // Programatically define a board with the diagonal
        // set to their row/col index (1-based)
        let as_code = {
            let mut b = Board {
                cells: [[ANY; 9]; 9],
            };
            for diag in 0..9 {
                b.set(diag, diag, Cell::Value((diag + 1) as u8))
                    .expect("unexpected error setting up test");
            }
            b
        };
        as_code.check().expect("Failed to validate board.");

        // Define the same as above as a literal.
        let as_literal = Board {
            cells: [
                [v(1), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [ANY, v(2), ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [ANY, ANY, v(3), ANY, ANY, ANY, ANY, ANY, ANY],
                [ANY, ANY, ANY, v(4), ANY, ANY, ANY, ANY, ANY],
                [ANY, ANY, ANY, ANY, v(5), ANY, ANY, ANY, ANY],
                [ANY, ANY, ANY, ANY, ANY, v(6), ANY, ANY, ANY],
                [ANY, ANY, ANY, ANY, ANY, ANY, v(7), ANY, ANY],
                [ANY, ANY, ANY, ANY, ANY, ANY, ANY, v(8), ANY],
                [ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY, v(9)],
            ],
        };
        as_literal.check().expect("Failed to validate board.");
        assert_eq!(as_code, as_literal);

        // If we leave things as ANY everywhere, the comparison should fail.
        let b = Board {
            cells: [[ANY; 9]; 9],
        };
        assert_ne!(b, as_literal);
    }

    #[test]
    fn check_iterators() {
        let b = Board {
            cells: [
                [ANY, v(4), ANY, v(7), ANY, v(1), ANY, ANY, v(3)],
                [v(1), v(3), ANY, ANY, ANY, ANY, ANY, v(4), ANY],
                [v(8), ANY, ANY, ANY, ANY, ANY, v(9), v(5), ANY],
                [ANY, v(8), ANY, v(3), ANY, v(2), ANY, ANY, v(5)],
                [ANY, ANY, ANY, ANY, v(8), ANY, ANY, ANY, ANY],
                [v(9), ANY, ANY, v(5), ANY, v(6), ANY, v(3), ANY],
                [ANY, v(7), v(1), ANY, ANY, ANY, ANY, ANY, v(9)],
                [ANY, v(9), ANY, ANY, ANY, ANY, ANY, v(2), v(4)],
                [v(3), ANY, ANY, v(4), ANY, v(8), ANY, v(7), ANY],
            ],
        };

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
        let mut input = Board {
            cells: [
                [v(1), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [ANY, v(9), v(8), v(7), v(6), v(5), v(4), v(3), v(2)],
                [v(2), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [v(3), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [v(4), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [v(5), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [v(6), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [v(7), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
                [v(8), ANY, ANY, ANY, ANY, ANY, ANY, ANY, ANY],
            ],
        };
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
        let mut input = Board {
            cells: [
                [ANY, v(4), v(9), v(7), v(5), v(1), v(8), v(6), v(3)],
                [v(1), ANY, v(5), v(8), v(6), v(9), v(7), v(4), v(2)],
                [v(8), v(6), ANY, v(2), v(4), v(3), v(9), v(5), v(1)],
                [v(7), v(8), v(6), ANY, v(1), v(2), v(4), v(9), v(5)],
                [v(5), v(2), v(3), v(9), ANY, v(4), v(6), v(1), v(7)],
                [v(9), v(1), v(4), v(5), v(7), ANY, v(2), v(3), v(8)],
                [v(4), v(7), v(1), v(6), v(2), v(5), ANY, v(8), v(9)],
                [v(6), v(9), v(8), v(1), v(3), v(7), v(5), ANY, v(4)],
                [v(3), v(5), v(2), v(4), v(9), v(8), v(1), v(7), ANY],
            ],
        };
        input.check().expect("Failed to validate board.");

        // This should close in a single pass of the solver.
        let (options, changed) = input.solve_one().expect("error during solve_one");
        assert_eq!(changed, true);
        assert_eq!(options, 0);

        let solution = Board {
            cells: [
                [v(2), v(4), v(9), v(7), v(5), v(1), v(8), v(6), v(3)],
                [v(1), v(3), v(5), v(8), v(6), v(9), v(7), v(4), v(2)],
                [v(8), v(6), v(7), v(2), v(4), v(3), v(9), v(5), v(1)],
                [v(7), v(8), v(6), v(3), v(1), v(2), v(4), v(9), v(5)],
                [v(5), v(2), v(3), v(9), v(8), v(4), v(6), v(1), v(7)],
                [v(9), v(1), v(4), v(5), v(7), v(6), v(2), v(3), v(8)],
                [v(4), v(7), v(1), v(6), v(2), v(5), v(3), v(8), v(9)],
                [v(6), v(9), v(8), v(1), v(3), v(7), v(5), v(2), v(4)],
                [v(3), v(5), v(2), v(4), v(9), v(8), v(1), v(7), v(6)],
            ],
        };
        solution.check().expect("Failed to validate board.");

        assert_eq!(input, solution);
    }

    #[test]
    fn check_solve_super_hard() {
        let mut input = Board {
            cells: [
                [ANY, v(4), ANY, v(7), ANY, v(1), ANY, ANY, v(3)],
                [v(1), v(3), ANY, ANY, ANY, ANY, ANY, v(4), ANY],
                [v(8), ANY, ANY, ANY, ANY, ANY, v(9), v(5), ANY],
                [ANY, v(8), ANY, v(3), ANY, v(2), ANY, ANY, v(5)],
                [ANY, ANY, ANY, ANY, v(8), ANY, ANY, ANY, ANY],
                [v(9), ANY, ANY, v(5), ANY, v(6), ANY, v(3), ANY],
                [ANY, v(7), v(1), ANY, ANY, ANY, ANY, ANY, v(9)],
                [ANY, v(9), ANY, ANY, ANY, ANY, ANY, v(2), v(4)],
                [v(3), ANY, ANY, v(4), ANY, v(8), ANY, v(7), ANY],
            ],
        };
        input.check().expect("Failed to validate board.");

        let solution = Board {
            cells: [
                [v(2), v(4), v(9), v(7), v(5), v(1), v(8), v(6), v(3)],
                [v(1), v(3), v(5), v(8), v(6), v(9), v(7), v(4), v(2)],
                [v(8), v(6), v(7), v(2), v(4), v(3), v(9), v(5), v(1)],
                [v(7), v(8), v(6), v(3), v(1), v(2), v(4), v(9), v(5)],
                [v(5), v(2), v(3), v(9), v(8), v(4), v(6), v(1), v(7)],
                [v(9), v(1), v(4), v(5), v(7), v(6), v(2), v(3), v(8)],
                [v(4), v(7), v(1), v(6), v(2), v(5), v(3), v(8), v(9)],
                [v(6), v(9), v(8), v(1), v(3), v(7), v(5), v(2), v(4)],
                [v(3), v(5), v(2), v(4), v(9), v(8), v(1), v(7), v(6)],
            ],
        };
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
        let solution = Board {
            cells: [
                [v(1), v(4), v(5), v(3), v(2), v(7), v(6), v(9), v(8)],
                [v(8), v(3), v(9), v(6), v(5), v(4), v(1), v(2), v(7)],
                [v(6), v(7), v(2), v(9), v(1), v(8), v(5), v(4), v(3)],
                [v(4), v(9), v(6), v(1), v(8), v(5), v(3), v(7), v(2)],
                [v(2), v(1), v(8), v(4), v(7), v(3), v(9), v(5), v(6)],
                [v(7), v(5), v(3), v(2), v(9), v(6), v(4), v(8), v(1)],
                [v(3), v(6), v(7), v(5), v(4), v(2), v(8), v(1), v(9)],
                [v(9), v(8), v(4), v(7), v(6), v(1), v(2), v(3), v(5)],
                [v(5), v(2), v(1), v(8), v(3), v(9), v(7), v(6), v(4)],
            ],
        };
        solution.check().expect("Failed to validate board.");

        // This is from:
        // https://punemirror.indiatimes.com/pune/cover-story/worlds-toughest-sudoku-is-here-can-you-crack-it/articleshow/32299679.cms
        let mut input = Board {
            cells: [
                [ANY, ANY, v(5), v(3), ANY, ANY, ANY, ANY, ANY],
                [v(8), ANY, ANY, ANY, ANY, ANY, ANY, v(2), ANY],
                [ANY, v(7), ANY, ANY, v(1), ANY, v(5), ANY, ANY],
                [v(4), ANY, ANY, ANY, ANY, v(5), v(3), ANY, ANY],
                [ANY, v(1), ANY, ANY, v(7), ANY, ANY, ANY, v(6)],
                [ANY, ANY, v(3), v(2), ANY, ANY, ANY, v(8), ANY],
                [ANY, v(6), ANY, v(5), ANY, ANY, ANY, ANY, v(9)],
                [ANY, ANY, v(4), ANY, ANY, ANY, ANY, v(3), ANY],
                [ANY, ANY, ANY, ANY, ANY, v(9), v(7), ANY, ANY],
            ],
        };
        input.check().expect("Failed to validate board.");
        println!("input: {:#?}", input);

        // Run the solver.
        input.solve().expect("error finding solution");

        assert_board(input, solution);
        assert_eq!(input, solution);
    }
}
