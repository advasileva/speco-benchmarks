mod book;
mod cart;
mod movie;
mod product;
mod straight_cart;

use book::Book;
use cart::Cart;
use movie::Movie;
use straight_cart::StraightCart;

pub fn run_with() {
  let name = String::from("1984");
  let price = 15;
  let movie = Movie::new(name, price);
  let movie = Box::new(movie);

  let quantity = 10;
  let cart = Cart::new(movie, quantity);

  assert_eq!(cart.total(), 150);
}

pub fn run_without() {
  let name = String::from("1984");
  let price = 15;
  let book = Book::new(name, price);
  let book = Box::new(book);

  let quantity = 10;
  let cart = StraightCart::new(book, quantity);

  assert_eq!(cart.total(), 150);
}
