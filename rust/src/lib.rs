mod book;
mod cart;
mod movie;
mod product;
mod first_cart;
mod second_cart;

use book::Book;
use cart::Cart;
use first_cart::Cart1;
use second_cart::Cart2;
use movie::Movie;

pub fn run_with() {
  let mut c = Cart::new(Box::new(Book::new(String::from("1984"), 40)), 10);
  c.product = Box::new(Movie::new(String::from("Godfather"), 15));
  for n in 1..10_000_000 {
    c.total();
  }
}

pub fn run_without() {
  let c1 = Cart1::new(Box::new(Book::new(String::from("1984"), 40)), 10);
  let c2 = c1.withMovie(Box::new(Movie::new(String::from("Godfather"), 15)));
  for n in 1..10_000_000 {
    c2.total();
  }
}
