pub mod ast;
pub mod grammar;

use ast::Term;

use lalrpop_util::ParseError;

fn main() {
  // λ
  // (λx.M)
  let term = grammar::TermParser::new().parse("λx.x");

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

  #[test]
  fn parse_abs() {
    let term = grammar::TermParser::new().parse("λx.x");

    let expected = Ok(Term::Abs(
      String::from("x"),
      Box::new(Term::Var(String::from("x"))),
    ));

    dbg!(&term);

    assert_eq!(term, expected);
  }
}
