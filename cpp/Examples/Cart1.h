#ifndef BENCHMARKING_CART1_H
#define BENCHMARKING_CART1_H


#include "Book.h"
#include "Movie.h"
#include "Cart2.h"

class Cart1 {
public:
    Book *p;
    int qty;

    Cart1(Book *p);

    int total();

    Cart2 with(Movie *m);
};


#endif //BENCHMARKING_CART1_H
