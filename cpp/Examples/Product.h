#ifndef BENCHMARKING_PRODUCT_H
#define BENCHMARKING_PRODUCT_H

#include <iostream>

using namespace std;

class Product {
public:
    string title;

    Product(string title);

    virtual int price();
};


#endif //BENCHMARKING_PRODUCT_H
