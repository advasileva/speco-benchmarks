use super::product::Product;

pub struct Book {
  name: String,
  price: i128,
}

impl Product for Book {
  fn name(&self) -> String {
    self.name.clone()
  }

  fn price(&self) -> i128 {
    self.price
  }
}

impl Book {
  pub fn new(name: String, price: i128) -> Book {
    Book { name, price }
  }

  pub fn price(&self) -> i128 {
    self.price
  }
}
