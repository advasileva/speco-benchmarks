#include "Book.h"

Book::Book(string title) : Product(title) {
}

int Book::price() {
    return 40;
}
