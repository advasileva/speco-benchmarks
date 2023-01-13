#include "Measurable.h"

void measureWith() {
    Cart c = Cart(new Book("1984"));
    c.p = new Movie("Godfather");
    for (size_t i = 0; i < 10000000000L; i++) {
        c.total();
    }
}

void measureWithout() {
    Cart1 c1 = Cart1(new Book("1984"));
    Cart2 c2 = c1.with(new Movie("Godfather"));
    for (size_t i = 0; i < 10000000000L; i++) {
        c2.total();
    }
}