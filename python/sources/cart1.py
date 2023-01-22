from .cart2 import Cart2

class Cart1:
    def __init__(self, b):
        self.p = b
        self.qty = 10
    
    def total(self):
        return self.p.price() * self.qty

    def withMovie(self, m):
        return Cart2(m)