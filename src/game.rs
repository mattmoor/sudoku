mod bitset;

use std::fmt;
use std::result::Result;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Value(usize),            // Holds the actual value.
    Options(bitset::BitSet), // Holds a bit-mask of availabile options
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Value(v) => return f.write_fmt(format_args!("\t{}", v)),
            Cell::Options(opts) => return f.write_fmt(format_args!("\t{:?}", opts)),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Board {
    cells: [[Cell; 9]; 9],
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

    pub fn mask(it: impl Iterator<Item = super::Cell>) -> super::bitset::BitSet {
        let mut mask = super::bitset::BitSet::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        for elt in it {
            if let super::Cell::Value(v) = elt {
                mask = mask.unset(v as usize);
            }
        }
        mask
    }

    pub fn frequency(it: impl Iterator<Item = super::Cell>, value: usize) -> usize {
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
                    if opts.has(value as usize) {
                        freq += 1;
                    }
                }
            };
        }
        freq
    }
}

impl Board {
    pub fn new(values: [[usize; 9]; 9]) -> Result<Board, String> {
        let all = bitset::BitSet::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut board = Board {
            cells: [[Cell::Options(all); 9]; 9],
        };

        for i in 0..9 {
            for j in 0..9 {
                let c = values[i][j];
                if c > 9 {
                    return Err(format!("Invalid value ({}, {}) = {}", i, j, c));
                }
                if c == 0 {
                    continue; // 0 is used for unspecified, so leave "all" options.
                }
                board.set(i, j, Cell::Value(c))?;
            }
        }

        Ok(board)
    }

    pub fn parse(input: String) -> Result<Board, String> {
        let rows = input
            .trim_end_matches("\n")
            .split("\n")
            .collect::<Vec<&str>>();
        if rows.len() != 9 {
            return Err(format!("input has {} rows, wanted 9", rows.len()));
        }

        let mut raw_board = [[0; 9]; 9];
        let mut i = 0;
        for row in rows {
            if row.len() != 9 {
                return Err(format!("row {} has {} columns, wanted 9", row, row.len()));
            }
            for (j, c) in row.chars().enumerate() {
                match c {
                    // Accept space or 0 as a blank.
                    '0' | ' ' => continue,
                    // Digits become a real value.
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        raw_board[i][j] = c as usize - '0' as usize;
                    }
                    // Anything else is an error.
                    _ => return Err(format!("Found invalid input character: {}", c)),
                }
            }
            i += 1;
        }
        return Board::new(raw_board);
    }

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

        // First we check whether 0 square's available options consist of a single value.
        for ridx in 0..9 {
            for cidx in 0..9 {
                if let Cell::Options(og_opts) = self.cells[ridx][cidx] {
                    let opts = og_opts
                        .intersect(views::mask(self.row(ridx)))
                        .intersect(views::mask(self.col(cidx)))
                        .intersect(views::mask(self.subsquare(ridx / 3, cidx / 3)));
                    if opts.empty() {
                        // If there are no options, then something went wrong.
                        return Err(format!(
                            "There are no remaining options for {}, {}",
                            ridx, cidx
                        ));
                    } else if let Some(value) = opts.singleton() {
                        // If it's a power of two, then there's only one option.
                        self.set(ridx, cidx, Cell::Value(value))?;
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

        // Second we check whether 0 square is the only square in a range that could hold a given value.
        for ridx in 0..9 {
            for cidx in 0..9 {
                if let Cell::Options(opts) = self.cells[ridx][cidx] {
                    for v in opts.foreach() {
                        let value = v;
                        if views::frequency(self.row(ridx), value) == 1
                            || views::frequency(self.col(cidx), value) == 1
                            || views::frequency(self.subsquare(ridx / 3, cidx / 3), value) == 1
                        {
                            self.set(ridx, cidx, Cell::Value(value))?;
                            changed = true;
                            options -= 1; // We counted this cell above, but it's now concrete so remove it.
                            break;
                        }
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
                                if opts.count() <= count {
                                    candidate_rdx = ridx;
                                    candidate_cdx = cidx;
                                    count = opts.count();
                                }
                            }
                        }
                    }
                    if let Cell::Options(opts) = self.cells[candidate_rdx][candidate_cdx] {
                        for v in opts.foreach() {
                            let value = v;
                            // Create a copy of the board with which we will speculate the value of this cell.
                            let mut speculator = *self;
                            speculator.set(candidate_rdx, candidate_cdx, Cell::Value(value))?;
                            // Try to recursively solve the board.
                            if speculator.solve().is_ok() {
                                self.cells = speculator.cells;
                                return Ok(());
                            }
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
mod tests;
