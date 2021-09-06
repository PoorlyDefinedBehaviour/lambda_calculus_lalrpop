pub mod ast;
mod grammar;
mod substitution;

use ast::Term;

fn parse(input: &str) -> Term {
  grammar::TermParser::new().parse(input).unwrap()
}

fn main() {
  let term = parse("(((λx.(λy.x)) 1) 2)");

  match substitution::eval(&term) {
    Err(message) => println!("{}", message),
    Ok(evaluated_term) => println!("{}", evaluated_term),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ast::Term::*;
  use crate::substitution::eval;

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
    let tests = vec![
      (
        "(((λa.(λb.((a (λt.(λf.t))) b))) (λt.(λf.t))) (λt.(λf.f)))",
        "(λt.(λf.t))",
      ),
      (
        "(((λa.(λb.((a (λt.(λf.t))) b))) (λt.(λf.f))) (λt.(λf.t)))",
        "(λt.(λf.t))",
      ),
      (
        "(((λa.(λb.((a (λt.(λf.t))) b))) (λt.(λf.t))) (λt.(λf.t)))",
        "(λt.(λf.t))",
      ),
      (
        "(((λa.(λb.((a (λt.(λf.t))) b))) (λt.(λf.f))) (λt.(λf.f)))",
        "(λt.(λf.f))",
      ),
    ];

    for (input, expected) in tests {
      let term = eval(&parse(input)).unwrap();

      assert_eq!(expected, format!("{}", term));
    }
  }

  #[test]
  fn encode_and() {
    let tests = vec![
      (
        "(((λa.(λb.((a b) (λt.(λf.f))))) (λt.(λf.t))) (λt.(λf.f)))",
        "(λt.(λf.f))",
      ),
      (
        "(((λa.(λb.((a b) (λt.(λf.f))))) (λt.(λf.f))) (λt.(λf.t)))",
        "(λt.(λf.f))",
      ),
      (
        "(((λa.(λb.((a b) (λt.(λf.f))))) (λt.(λf.f))) (λt.(λf.f)))",
        "(λt.(λf.f))",
      ),
      (
        "(((λa.(λb.((a b) (λt.(λf.f))))) (λt.(λf.t))) (λt.(λf.t)))",
        "(λt.(λf.t))",
      ),
    ];

    for (input, expected) in tests {
      let term = eval(&parse(input)).unwrap();

      assert_eq!(expected, format!("{}", term));
    }
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
