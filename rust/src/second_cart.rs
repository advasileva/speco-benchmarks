use super::movie::Movie;

pub struct Cart2 {
  pub product: Box<Movie>,
  quantity: usize,
}

impl Cart2 {
  pub fn total(&self) -> i128 {
    self.product.price() * self.quantity as i128
  }

  pub fn new(product: Box<Movie>, quantity: usize) -> Self {
    Cart2 { product, quantity }
  }
}
