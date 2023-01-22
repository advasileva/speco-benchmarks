from sources import Cart, Movie, Book

def measureWith():
    c = Cart(Book("1984"))
    c.p = Movie("Godfather")
    for _ in range(1_000_000_000):
        c.total()