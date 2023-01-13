#ifndef BENCHMARKING_CART2_H
#define BENCHMARKING_CART2_H

#include "iostream"
#include "Movie.h"


class Cart2 {
public:
    Movie *p;
    int qty;

    Cart2(Movie *p);

    int total();
};


#endif //BENCHMARKING_CART2_H
