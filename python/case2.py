from sources import Cart1, Movie, Book

def measureWithout():
    c1 = Cart1(Book("1984"))
    c2 = c1.withMovie(Movie("Godfather"))
    for _ in range(1_000_000_000):
        c2.total()