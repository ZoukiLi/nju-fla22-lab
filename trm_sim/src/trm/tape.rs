//! this module contains the tape struct and its methods


use std::collections::VecDeque;

/// a tape is a vector of symbols with a head
/// that can move left and right
/// the head is *always* on a symbol
/// the tape is infinite in both directions

#[derive(Debug, Clone)]
pub struct Tape {
    /// the symbols on the tape
    tape: VecDeque<Option<char>>,
    /// current position of the head
    head: usize,
    /// the index of the first symbol on the tape
    /// head + offset = tape index from outside
    offset: isize,
}

impl Tape {
    /// creates a new tape with the given string
    /// # Example
    /// ```
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("0101");
    /// ```
    pub fn new(s: &str) -> Self {
        Self {
            tape: s.chars().map(Some).collect(),
            head: 0,
            offset: 0,
        }
    }

    /// returns the symbol under the head
    /// # Example
    /// ```
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("0101");
    /// assert_eq!(tape.read(), Some('0'));
    /// ```
    /// if the head is out of bounds, it returns None
    /// ```
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("");
    /// assert_eq!(tape.read(), None);
    /// ```
    pub fn read(&self) -> Option<char> {
        self.tape.get(self.head).and_then(|o| *o)
    }

    /// writes a symbol under the head
    /// # Example
    /// ```
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("0101");
    /// tape.write('1');
    /// assert_eq!(tape.read(), Some('1'));
    /// ```
    /// if the head is out of bounds, adds a new symbol
    /// ```
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("");
    /// tape.write('1');
    /// assert_eq!(tape.read(), Some('1'));
    /// ```
    pub fn write(&mut self, c: char) {
        if let Some(s) = self.tape.get_mut(self.head) {
            *s = Some(c);
        } else {
            self.tape.push_back(Some(c));
        }
    }

    /// move the head left
    /// # Example
    /// ```
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("0101");
    /// tape.move_left();
    /// assert_eq!(tape.read(), None);
    /// ```
    /// if the head is out of bounds, adds a new symbol
    /// ```
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("");
    /// tape.move_left();
    /// assert_eq!(tape.read(), None);
    /// ```
    pub fn move_left(&mut self) {
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
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("0101");
    /// tape.move_right();
    /// assert_eq!(tape.read(), Some('1'));
    /// ```
    /// if the head is out of bounds, adds a new symbol
    /// ```
    /// use trm_sim::trm::tape::Tape;
    /// let mut tape = Tape::new("");
    /// tape.move_right();
    /// assert_eq!(tape.read(), None);
    /// ```
    pub fn move_right(&mut self) {
        // if current symbol is None, add a new symbol
        if self.tape.get(self.head).is_none() {
            self.tape.push_back(None);
        }
        // if head is at the end of the tape, add a new symbol
        if self.head == self.tape.len() - 1 {
            self.tape.push_back(None);
        }
        self.head += 1;
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
}