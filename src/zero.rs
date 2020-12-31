//! A first, pretty bad implementation of a singly-linked list.
//! Let's see how much we can improve by following https://rust-unofficial.github.io/too-many-lists/.
//! This implementation wasn't easy for me. I struggled with the ownership rules a lot. Namely, re-designing
//! certain methods to adhere to Rust's compile time checks was a struggle. I found myself inadvertently
//! having multiple exclusive borrows in the same scope.

#![allow(dead_code)]

use std::fmt;

struct Node<'a, T> {
  next: Option<Box<Node<'a, T>>>,
  item: &'a T,
}

impl<'a, T> fmt::Display for Node<'a, T>
where
  T: fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.item)
  }
}

impl<'a, T> fmt::Display for List<'a, T>
where
  T: fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let None = &self.head {
      return write!(f, "[]");
    }

    let mut t = String::from("[");

    let mut p = self.head.as_ref().unwrap();

    loop {
      t.push_str(&p.item.to_string());
      if let Some(g) = &p.next {
        t.push_str(", ");
        p = g;
      } else {
        break;
      }
    }

    t.push_str("]");
    write!(f, "{}", t)
  }
}

pub struct List<'a, T> {
  head: Option<Box<Node<'a, T>>>,
}

impl<'a, T> List<'a, T> {
  pub fn new() -> Self {
    Self {
      head: None,
    }
  }

  pub fn length(&self) -> usize {
    let mut i: usize = 0;
    let mut n = &self.head;

    while let Some(x) = n {
      n = &x.next;
      i += 1;
    }

    i
  }

  pub fn push(&mut self, item: &'a T) {
    let node = Box::new(Node::new(item));

    if let None = self.head {
      self.head = Some(node);
      return;
    }
    let mut p = self.head.as_mut().unwrap();

    loop {
      match &mut p.next {
        Some(g) => {
          p = g;
        }
        next => {
          *next = Some(node);
          break;
        }
      }
    }
  }

  pub fn pop(&mut self) {
    let mut p = &mut self.head;

    loop {
      match p {
        Some(n) if n.next.is_none() => {
          p.take();
        },
        Some(n) => {
          p = &mut n.next;
        },
        None => break
      }
    }
  }
}

impl<'a, T> Node<'a, T> {
  fn new(item: &'a T) -> Self {
    Self {
      next: None,
      item,
    }
  }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn push() {
        let mut l = List::<i32>::new();

        l.push(&1);
        l.push(&2);
        l.push(&3);
        l.push(&4);

        assert_eq!(l.to_string(), "[1, 2, 3, 4]");
        assert_eq!(l.length(), 4);
    }

    #[test]
    fn pop_1() {
        let mut l = List::<i32>::new();

        l.push(&1);
        l.push(&2);
        l.push(&3);

        l.pop();
        l.pop();

        assert_eq!(l.to_string(), "[1]");
        assert_eq!(l.length(), 1);
    }

    #[test]
    fn pop_2() {
        let mut l = List::<i32>::new();

        l.pop();

        l.push(&1);
        l.push(&2);
        l.push(&3);

        l.pop();
        l.push(&4);
        l.push(&5);
        l.pop();
        l.push(&6);

        assert_eq!(l.to_string(), "[1, 2, 4, 6]");
        assert_eq!(l.length(), 4);
    }
}
