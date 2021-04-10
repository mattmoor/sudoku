use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct BitSet {
    data: usize,
}

impl BitSet {
    pub fn new(values: &[usize]) -> BitSet {
        let mut bs = BitSet { data: 0 };

        for v in values {
            bs = bs.set(*v);
        }
        bs
    }

    pub fn foreach(&self) -> Biterator {
        Biterator {
            data: self.data,
            index: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.data == 0
    }

    pub fn count(&self) -> usize {
        self.data.count_ones() as usize
    }

    pub fn singleton(&self) -> Option<usize> {
        if self.empty() {
            return None;
        }
        if self.data & (self.data - 1) != 0 {
            return None;
        }
        return Some(self.data.trailing_zeros() as usize);
    }

    pub fn has(&self, value: usize) -> bool {
        // Check whether the bit is set.
        self.data & (1 << value) != 0
    }

    pub fn set(&self, value: usize) -> BitSet {
        BitSet {
            data: self.data | 1 << value,
        }
    }

    pub fn unset(&self, value: usize) -> BitSet {
        BitSet {
            data: self.data & !(1 << value),
        }
    }

    pub fn intersect(&self, other: BitSet) -> BitSet {
        BitSet {
            data: self.data & other.data,
        }
    }

    pub fn union(&self, other: BitSet) -> BitSet {
        BitSet {
            data: self.data | other.data,
        }
    }
}

pub struct Biterator {
    data: usize,
    index: usize,
}
impl Iterator for Biterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut value = None;
        while self.index != 16 {
            if self.data & 1 != 0 {
                value = Some(self.index);
            }
            self.data >>= 1;
            self.index += 1;
            if value != None {
                break;
            }
        }
        value
    }
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        let mut comma = "";
        for v in self.foreach() {
            f.write_fmt(format_args!("{}{}", comma, v))?;
            comma = ",";
        }
        f.write_str("}")
    }
}

#[cfg(test)]
mod tests;
