use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
  Int(i32),
  Var(String),
  Abs(String, Box<Term>),
  App(Box<Term>, Box<Term>),
}

impl Display for Term {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Term::Int(i) => i.fmt(f),
      Term::Var(x) => x.fmt(f),
      Term::Abs(x, body) => {
        write!(f, "(λ")?;
        x.fmt(f)?;
        write!(f, ".")?;
        body.fmt(f)?;
        write!(f, ")")
      }
      Term::App(function, arg) => {
        write!(f, "(")?;
        function.fmt(f)?;
        arg.fmt(f)?;
        write!(f, ")")
      }
    }
  }
}
