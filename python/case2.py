from sources import Cart1, Movie, Book

def measureWithout():
    c1 = Cart1(Book("1984"))
    c2 = c1.withMovie(Movie("Godfather"))
    for _ in range(10_000_000_00):
        c2.total()