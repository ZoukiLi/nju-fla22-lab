//! this module contains the tape struct and its methods

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ops::Range;

use super::Direction;

/// a tape is a vector of symbols with a head
/// that can move left and right,
/// the tape is infinite in both directions

#[derive(Debug, Clone)]
pub struct Tape {
    /// the symbols on the tape
    tape: VecDeque<Option<char>>,
    /// current position of the head.
    /// this index is for inside, which means the index of the vector.
    /// the outside index usually has special meaning, so it can be negative.
    head: usize,
    /// the index of the first symbol on the tape
    /// head + offset = tape index from outside
    offset: isize,
}

/// frozen tape is a tape that can't be modified,
/// it is used for the tape history and visualization.
/// It only contain the non-empty and head range of the tape.
/// It is the mainly way to get a `Tape`'s inner data.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrozenTape {
    /// the tape's non-empty symbols
    pub tape: String,
    /// the outside index of head,
    /// can be both positive and negative
    pub head: isize,
    /// range of the tape that is not empty
    /// also can be both positive and negative
    pub range: Range<isize>,
}

impl Tape {
    /// creates a new tape with the given string
    /// # Example
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("0101");
    /// ```
    pub fn new(s: &str) -> Self {
        let mut data: VecDeque<_> = s.chars().map(Some).collect();
        if data.is_empty() {
            data.push_back(None);
        }

        Self {
            tape: data,
            head: 0,
            offset: 0,
        }
    }

    /// returns the symbol under the head
    /// # Example
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("0101");
    /// assert_eq!(tape.read(), Some('0'));
    /// ```
    /// if the head is out of bounds, it returns None
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("");
    /// assert_eq!(tape.read(), None);
    /// ```
    pub fn read(&self) -> Option<char> {
        self.tape.get(self.head).and_then(|o| *o)
    }

    /// writes a symbol under the head
    /// # Example
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("0101");
    /// tape.write('1');
    /// assert_eq!(tape.read(), Some('1'));
    /// ```
    /// if the head is out of bounds, adds a new symbol
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("");
    /// tape.write('1');
    /// assert_eq!(tape.read(), Some('1'));
    /// ```
    pub fn write(&mut self, c: char) {
        if let Some(s) = self.tape.get_mut(self.head) {
            *s = Some(c);
        }
    }

    /// write a blank symbol under the head
    /// # Example
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("0101");
    /// tape.write_blank();
    /// assert_eq!(tape.read(), None);
    /// ```
    /// if the head is out of bounds, adds a new symbol
    pub fn write_blank(&mut self) {
        if let Some(s) = self.tape.get_mut(self.head) {
            *s = None;
        }
    }

    /// move the head left
    /// # Example
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("0101");
    /// tape.move_left();
    /// assert_eq!(tape.read(), None);
    /// ```
    /// if the head is out of bounds, adds a new symbol
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("");
    /// tape.move_left();
    /// assert_eq!(tape.read(), None);
    /// ```
    pub fn move_left(&mut self) {
        // if head is at the beginning of the tape,
        // add a new symbol to the beginning
        if self.head == 0 {
            self.tape.push_front(None);
            self.offset -= 1;
        } else {
            self.head -= 1;
        }
    }

    /// move the head right
    /// # Example
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("0101");
    /// tape.move_right();
    /// assert_eq!(tape.read(), Some('1'));
    /// ```
    /// if the head is out of bounds, adds a new symbol
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new("");
    /// tape.move_right();
    /// assert_eq!(tape.read(), None);
    /// ```
    pub fn move_right(&mut self) {
        // if head is at the end of the tape, add a new symbol
        if self.head == self.tape.len() - 1 {
            self.tape.push_back(None);
        }
        self.head += 1;
    }

    /// move the head with given direction,
    /// stays if the direction is `Stay`
    /// # Example
    /// ```
    /// use trm_sim::trm::{Tape, Direction};
    /// let mut tape = Tape::new("0101");
    /// tape.move_to(Direction::Left);
    /// assert_eq!(tape.read(), None);
    /// ```
    pub fn move_to(&mut self, dir: Direction) {
        match dir {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Stay => (),
        }
    }

    /// returns the tape's frozen version,
    /// removing the empty symbol and None on the tape.
    /// # Example
    /// ```
    /// use trm_sim::trm::Tape;
    /// let mut tape = Tape::new(" 0101 ");
    /// let frozen = tape.freeze(' ');
    /// assert_eq!(frozen.tape, " 0101");
    /// assert_eq!(frozen.head, 0);
    /// assert_eq!(frozen.range, 0..5);
    /// ```
    pub fn freeze(&self, empty: char) -> FrozenTape {
        // get the first non-empty symbol before head
        let start = self
            .tape
            .iter()
            .take(self.head)
            .position(|o| o.map(|c| c != empty).unwrap_or(false))
            .unwrap_or(self.head);
        // get the last non-empty symbol after head
        let end = self
            .tape
            .iter()
            .skip(self.head + 1)
            .rposition(|o| o.map(|c| c != empty).unwrap_or(false))
            .map_or(self.head, |i| i + self.head + 1);
        // get the non-empty symbols
        let tape: String = self
            .tape
            .iter()
            .skip(start)
            .take(end - start + 1)
            .filter_map(|o| *o)
            .collect();
        // get the outside index of head
        let head = self.head as isize + self.offset;
        // get the range of the tape
        let range = start as isize + self.offset..end as isize + self.offset + 1;

        FrozenTape { tape, head, range }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_str_iter() {
        let s = String::from("0101");
        let mut iter = s.chars();
        assert_eq!(iter.next(), Some('0'));
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), Some('0'));
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), None);

        let s_unicode = String::from("👍👍👍👍");
        let mut iter_char = s_unicode.chars();
        let mut iter_byte = s_unicode.bytes();
        assert_eq!(iter_char.next(), Some('👍'));
        println!("{:?}", iter_char.next());
        assert_eq!(iter_byte.next(), Some(240));
        println!("{:?}", iter_byte.next());
    }

    #[test]
    fn test_vec_range() {
        let vec1 = vec![1, 2, 3, 4, 5];
        let len = vec1.len();
        for i in 0..len {
            assert!(vec1.get(i).is_some());
        }
        let range = 0..len;
        for i in range {
            assert!(vec1.get(i).is_some());
        }
    }

    use super::Tape;

    #[test]
    fn test_tape_usage() {
        let mut tape = Tape::new("0101");
        assert_eq!(tape.tape.len(), 4);
        assert_eq!(tape.head, 0);
        assert_eq!(tape.offset, 0);
        assert_eq!(tape.read(), Some('0'));
        assert_eq!(tape.tape[0], Some('0'));
        tape.write('1');
        assert_eq!(tape.read(), Some('1'));
        tape.move_left();
        assert_eq!(tape.read(), None);
        assert_eq!(tape.head, 0);
        assert_eq!(tape.offset, -1);

        let mut null_tape = Tape::new("");
        let mut null_tape2 = null_tape.clone();
        let mut null_tape3 = null_tape.clone();
        assert_eq!(null_tape.read(), None);
        null_tape.move_left();
        assert_eq!(null_tape.read(), None);
        assert_eq!(null_tape.head, 0);
        assert_eq!(null_tape.offset, -1);
        null_tape.move_right();
        assert_eq!(null_tape.read(), None);
        assert_eq!(null_tape.head, 1);
        assert_eq!(null_tape.offset, -1);
        null_tape.write('1');
        assert_eq!(null_tape.read(), Some('1'));
        assert_eq!(null_tape.head, 1);
        assert_eq!(null_tape.offset, -1);

        assert_eq!(null_tape2.read(), None);
        null_tape2.move_right();
        assert_eq!(null_tape2.read(), None);
        null_tape2.move_left();
        assert_eq!(null_tape2.read(), None);

        assert_eq!(null_tape3.read(), None);
        null_tape3.write('1');
        assert_eq!(null_tape3.read(), Some('1'));
        null_tape3.move_left();
        assert_eq!(null_tape3.read(), None);
        null_tape3.move_right();
        assert_eq!(null_tape3.read(), Some('1'));
    }

    #[test]
    fn test_tape_freeze() {
        let tape = Tape::new(" 0101 ");
        let frozen = tape.freeze(' ');
        assert_eq!(frozen.tape, " 0101");
        assert_eq!(frozen.head, 0);
        assert_eq!(frozen.range, 0..5);

        let tape2 = Tape::new("");
        let frozen2 = tape2.freeze(' ');
        println!("{:#?}", frozen2);
    }
}
