use super::product::Product;

pub struct Movie {
  title: String,
  price: i128,
}

impl Product for Movie {
  fn title(&self) -> String {
    self.title.clone()
  }

  fn price(&self) -> i128 {
    self.price
  }
}

impl Movie {
  pub fn new(title: String, price: i128) -> Movie {
    Movie { title, price }
  }

  pub fn price(&self) -> i128 {
    self.price
  }
}
