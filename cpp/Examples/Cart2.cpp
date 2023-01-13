#include "Cart2.h"

Cart2::Cart2(Movie *p) {
    this->p = p;
    this->qty = 10;
}

int Cart2::total() {
    return p->price() * qty;
}
