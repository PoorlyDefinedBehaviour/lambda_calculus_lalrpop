pub mod ast;
mod grammar;
mod interpreter;

use ast::Term;

fn parse(input: &str) -> Term {
  grammar::TermParser::new().parse(input).unwrap()
}

fn main() {
  let term = parse("(((λx.(λy.x)) 1) 2)");

  dbg!(&term);

  match interpreter::eval(&term) {
    Err(message) => println!("{}", message),
    Ok(evaluated_term) => println!("{}", evaluated_term),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ast::Term::*;
  use crate::interpreter::eval;

  #[test]
  fn encode_true() {
    let term = eval(&parse("(((λx.(λy.x)) 1) 2)")).unwrap();

    assert_eq!(Int(1), term);
  }

  #[test]
  fn encode_false() {
    let term = eval(&parse("(((λx.(λy.y)) 1) 2)")).unwrap();

    assert_eq!(Int(2), term);
  }

  #[test]
  fn encode_or() {
    let term = eval(&parse(
      "(((λa.(λb.((a (λt.(λf.t))) b))) (λt.(λf.t))) (λt.(λf.f)))",
    ))
    .unwrap();

    assert_eq!("(λt.(λf.t))", format!("{}", term));
  }

  #[test]
  fn parse_term() {
    let term = parse("x");

    let expected = Var(String::from("x"));

    assert_eq!(term, expected);
  }

  #[test]
  fn parse_abs() {
    let term = parse("(λx.x)");

    let expected = Abs(String::from("x"), Box::new(Var(String::from("x"))));

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
      let term = parse(input);

      assert_eq!(term, expected);
    }
  }

  #[test]
  fn parse_i32() {
    let tests = vec![("0", Int(0)), ("1", Int(1)), ("12345", Int(12345))];

    for (input, expected) in tests {
      let term = parse(input);

      assert_eq!(term, expected);
    }
  }
}
