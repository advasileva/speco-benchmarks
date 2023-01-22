from sources import Cart, Movie, Book

def measureWith():
    c = Cart(Book("1984"))
    c.p = Movie("Godfather")
    for _ in range(10_000_000_00):
        c.total()