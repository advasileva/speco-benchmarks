use super::product::Product;

pub struct Book {
  title: String,
  price: i128,
}

impl Product for Book {
  fn title(&self) -> String {
    self.title.clone()
  }

  fn price(&self) -> i128 {
    self.price
  }
}

impl Book {
  pub fn new(title: String, price: i128) -> Book {
    Book { title, price }
  }

  pub fn price(&self) -> i128 {
    self.price
  }
}
