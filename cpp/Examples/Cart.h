#ifndef BENCHMARKING_CART_H
#define BENCHMARKING_CART_H

#include "iostream"
#include "Product.h"


class Cart {
public:
    Product *p;
    int qty;

    Cart(Product *p);

    int total();
};


#endif //BENCHMARKING_CART_H
