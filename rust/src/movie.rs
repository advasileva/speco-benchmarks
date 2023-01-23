use super::product::Product;

pub struct Movie {
  name: String,
  price: i128,
}

impl Product for Movie {
  fn name(&self) -> String {
    self.name.clone()
  }

  fn price(&self) -> i128 {
    self.price
  }
}

impl Movie {
  pub fn new(name: String, price: i128) -> Movie {
    Movie { name, price }
  }
}
