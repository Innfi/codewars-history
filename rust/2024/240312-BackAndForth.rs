// https://www.codewars.com/kata/60cc93db4ab0ae0026761232

#[cfg(test)]
mod tests {
  #[derive(PartialEq)]
  enum IndexerState {
    AfterHead,
    Head,
    Tail,
    BeforeHead
  }
  struct Indexer {
    current: usize,
    head: usize,
    tail: usize,
    state: IndexerState,
  }
  impl Indexer {
    fn new(len: usize) -> Self {
      Indexer {
        current: 0,
        head: 0,
        tail: len-1,
        state: IndexerState::Head,
      }
    }

    fn next(&mut self) -> usize {
      if self.state == IndexerState::AfterHead {
        let output = self.current;

        self.current = self.tail;
        self.state = IndexerState::Tail;

        return output;
      }  else if self.state == IndexerState::Tail {
        let output = self.current;

        self.tail -= 1;
        self.current = self.tail;
        self.state = IndexerState::BeforeHead;

        return output;
      } else if self.state == IndexerState::BeforeHead {
        let output = self.current;

        self.head += 1;
        self.current = self.head;
        self.state = IndexerState::Head;

        return output;
      } else {
        let output = self.current;

        self.head += 1;
        self.current = self.head;
        self.state = IndexerState::AfterHead;

        return output;
      }
    }
  }

  #[test]
  fn test_indexer() {
    let mut instance = Indexer::new(6);

    assert_eq!(instance.next(), 0);
    assert_eq!(instance.next(), 5);
    assert_eq!(instance.next(), 4);
    assert_eq!(instance.next(), 1);
    assert_eq!(instance.next(), 0);
    assert_eq!(instance.next(), 0);
  }

  // fn arrange(s: &[i32]) -> Vec<i32> {
  //   // swap and take two?

  //   todo!()
  // }

  // #[test]
  // fn returns_expected() {
  //   assert_eq(arrange(vec![1]), vec![1]);
  //   assert_eq(arrange(vec![]), vec![]);
  //   assert_eq(arrange(vec![1,2]), vec![1,2]);
  //   assert_eq(arrange(vec![4, 3, 2]), vec![4, 2, 3]);
  //   assert_eq(arrange(vec![2, 4, 3, 4]), vec![2, 4, 3, 4]);
  //   assert_eq(arrange(vec![9, 7, -2, 8, 5, -3, 6, 5, 1]), vec![9, 1, 5, 7, -2, 6, -3, 8, 5]);
  // }
}
