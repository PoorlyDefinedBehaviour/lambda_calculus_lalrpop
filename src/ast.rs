#[derive(Debug, PartialEq)]
pub enum Term {
  Var(String),
  Abs((String, Box<Term>)),
  App(Box<Term>, Box<Term>),
}
