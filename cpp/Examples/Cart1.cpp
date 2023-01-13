#include "Cart1.h"

Cart1::Cart1(Book *p) {
    this->p = p;
    this->qty = 10;
}

int Cart1::total() {
    return p->price() * qty;
}

Cart2 Cart1::with(Movie *m) {
    Cart2 c2 = Cart2(m);
    return c2;
}
