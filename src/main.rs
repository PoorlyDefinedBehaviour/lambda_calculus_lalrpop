pub mod ast;
pub mod environment;
pub mod grammar;
pub mod substitution;

use ast::Term;

fn parse(input: &str) -> Term {
  grammar::TermParser::new().parse(input).unwrap()
}

fn main() {
  let term = parse("(((λa.(λb.(a b))) (λx.x)) 1)");

  dbg!(environment::eval(&term).unwrap());
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ast::Term::*;

  fn check(input: &str, expected: &str) {
    let term = parse(input);

    assert_eq!(expected, format!("{}", substitution::eval(&term).unwrap()));

    assert_eq!(expected, format!("{}", environment::eval(&term).unwrap()));
  }

  #[test]
  fn encode_true() {
    check("(((λx.(λy.x)) 1) 2)", "1");
  }

  #[test]
  fn encode_false() {
    check("(((λx.(λy.y)) 1) 2)", "2");
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
      check(input, expected);
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
      check(input, expected);
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
