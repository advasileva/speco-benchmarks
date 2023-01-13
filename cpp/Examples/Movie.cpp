#include "Movie.h"

Movie::Movie(string title) : Product(title) {
}

int Movie::price() {
    return 15;
}