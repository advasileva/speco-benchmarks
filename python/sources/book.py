from .product import Product

class Book(Product):
    def __init__(self, title):
        self.title = title
        Product.curr_price = 40
