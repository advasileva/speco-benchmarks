#ifndef BENCHMARKING_BOOK_H
#define BENCHMARKING_BOOK_H

#include "iostream"
#include "Product.h"


class Book : public Product {
public:
    Book(string title);

    virtual int price();
};


#endif //BENCHMARKING_BOOK_H
