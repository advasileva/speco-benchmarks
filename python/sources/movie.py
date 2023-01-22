from .product import Product

class Movie(Product):
    def __init__(self, title):
        self.title = title
        Product.curr_price = 15
