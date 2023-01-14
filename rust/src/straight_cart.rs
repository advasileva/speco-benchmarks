use super::book::Book;

pub struct StraightCart {
  book: Box<Book>,
  quantity: usize,
}

impl StraightCart {
  pub fn total(&self) -> i128 {
    self.book.price() * self.quantity as i128
  }

  pub fn new(book: Box<Book>, quantity: usize) -> Self {
    StraightCart { book, quantity }
  }
}
