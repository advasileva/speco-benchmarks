class Cart2:
    def __init__(self, m):
        self.p = m
        self.qty = 10
    
    def total(self):
        return self.p.price() * self.qty