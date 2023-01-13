#ifndef BENCHMARKING_MOVIE_H
#define BENCHMARKING_MOVIE_H

#include "iostream"
#include "Product.h"


class Movie : public Product {
public:
    Movie(string title);

    virtual int price();
};


#endif //BENCHMARKING_MOVIE_H
