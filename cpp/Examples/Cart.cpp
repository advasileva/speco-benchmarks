#include "Cart.h"

Cart::Cart(Product *p) {
    this->p = p;
    this->qty = 10;
}

int Cart::total() {
    return p->price() * qty;
}
