pub mod ast;
mod grammar;
mod interpreter;

use ast::Term;

fn main() {
  let term = grammar::TermParser::new().parse("((λx.x) (λy.y))").unwrap();

  match interpreter::eval(&term) {
    Err(message) => println!("{}", message),
    Ok(evaluated_term) => println!("{}", evaluated_term),
  }
}

mod tests {
  use super::*;

  use Term::*;

  #[test]
  fn parse_term() {
    let term = grammar::TermParser::new().parse("x");

    let expected = Ok(Var(String::from("x")));

    assert_eq!(term, expected);
  }

  #[test]
  fn parse_abs() {
    let term = grammar::TermParser::new().parse("(λx.x)");

    let expected = Ok(Abs(String::from("x"), Box::new(Var(String::from("x")))));

    assert_eq!(term, expected);
  }

  #[test]
  fn parse_app() {
    let tests = vec![(
      "((λx.x) y)",
      App(
        Box::new(Abs(String::from("x"), Box::new(Var(String::from("x"))))),
        Box::new(Var(String::from("y"))),
      ),
    )];

    for (input, expected) in tests {
      let term = grammar::TermParser::new().parse(input).unwrap();

      dbg!(&term);
      assert_eq!(term, expected);
    }
  }
}
