pub mod ast;
pub mod grammar;

use ast::Term;

fn main() {
  // λ
  // (λx.M)
  let term = grammar::TermParser::new().parse("x");

  dbg!(&term);
}

mod tests {
  use super::*;

  #[test]
  fn parse_term() {
    let term = grammar::TermParser::new().parse("x");

    let expected = Ok(Term::Var(String::from("x")));

    assert_eq!(term, expected);
  }
}
