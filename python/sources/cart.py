class Cart:
    def __init__(self, p):
        self.p = p
        self.qty = 10
    
    def total(self):
        return self.p.price() * self.qty
