use super::product::Product;

pub struct Cart {
  product: Box<dyn Product>,
  quantity: usize,
}

impl Cart {
  pub fn total(&self) -> i128 {
    self.product.price() * self.quantity as i128
  }

  pub fn new(product: Box<dyn Product>, quantity: usize) -> Self {
    Cart { product, quantity }
  }
}
