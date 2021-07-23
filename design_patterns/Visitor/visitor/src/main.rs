//Encapsulates an algorithm that operates over a heterogenous collection
// of objects. Allows for resusablility of data (or the data's primary behavior)


mod ast {
    pub enum Stmt{
        Expr(Expr),
        Let(Name, Expr),
    }

    pub struct Name {
        value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}


fn main() {
    println!("Hello, world!");
}
