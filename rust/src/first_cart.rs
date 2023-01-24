use super::movie::Movie;
use super::book::Book;
use super::second_cart::Cart2;

pub struct Cart1 {
  pub product: Box<Book>,
  quantity: usize,
}

impl Cart1 {
  pub fn total(&self) -> i128 {
    self.product.price() * self.quantity as i128
  }

  pub fn new(product: Box<Book>, quantity: usize) -> Self {
    Cart1 { product, quantity }
  }

  pub fn withMovie(&self, product: Box<Movie>) -> Cart2 {
    Cart2::new(product, self.quantity)
  }
}
