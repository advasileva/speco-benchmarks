package benchmarks

type Product interface {
	price() int
}

type Book struct {
	title string
}

func (b Book) price() int {
	return 40
}

type Movie struct {
	title string
}

func (m Movie) price() int {
	return 15
}

type Cart struct {
	p   Product
	qty int
}

func (c Cart) total() int {
	return c.p.price() * c.qty
}

type Cart2 struct {
	p   Movie
	qty int
}

func (c Cart2) total() int {
	return c.p.price() * c.qty
}

type Cart1 struct {
	p   Book
	qty int
}

func (c Cart1) total() int {
	return c.p.price() * c.qty
}

func (c Cart1) with(m Movie) Cart2 {
	return Cart2{m, c.qty}
}
