#[derive(Debug, PartialEq, Clone)]
pub enum Term {
  // TODO: add ints
  Var(String),
  Abs(String, Box<Term>),
  App(Box<Term>, Box<Term>),
}
