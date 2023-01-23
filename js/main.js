class Product {
    constructor(title) {
      this.title = title;
    }
  
    price() {
      return this.price;
    } 
}

class Book extends Product {
    constructor(title) {
        super(title);
        this.price = 40;
    }
}

class Movie extends Product {
    constructor(title) {
        super(title);
        this.price = 15;
    }
}

class Cart {
    constructor(p) {
        this.p = p;
        this.qty = 10;
    }

    total() {
        return this.p.price * this.qty;
    }
}

class Cart1 {
    constructor(p) {
        this.p = p;
        this.qty = 10;
    }

    total() {
        return this.p.price * this.qty;
    }

    withMovie(m) {
        return new Cart2(m);
    }
}

class Cart2 {
    constructor(p) {
        this.p = p;
        this.qty = 10;
    }

    total() {
        return this.p.price * this.qty;
    }
}
  
function measureWith() {
    let c = new Cart(new Book("1984"));
    c.p = new Movie("Godfather");
    for (let i = 0; i < 10_000_000_000; i++) {
        let x = c.total();
    }
}

function measureWithout() {
    let c1 = new Cart1(new Book("1984"));
    let c2 = c1.withMovie(new Movie("Godfather"));
    for (let i = 0; i < 10_000_000_000; i++) {
        let x = c2.total();
    }
}

console.time('with');
measureWith();
console.timeEnd('with');

console.log('\n');

console.time('without');
measureWithout();
console.timeEnd('without');