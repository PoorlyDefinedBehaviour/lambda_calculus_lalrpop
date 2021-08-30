pub mod ast;
mod grammar;
mod interpreter;

fn main() {
  let term = grammar::TermParser::new().parse("((λx.x) (λy.y))").unwrap();

  match interpreter::eval(&term) {
    Err(message) => println!("{}", message),
    Ok(evaluated_term) => println!("{}", evaluated_term),
  }
}

#[cfg(test)]
mod tests {
  use crate::ast::Term::*;
  use crate::grammar;

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

      assert_eq!(term, expected);
    }
  }

  #[test]
  fn parse_i32() {
    let tests = vec![];

    for (input, expected) in tests {
      let term = grammar::TermParser::new().parse(input).unwrap();

      assert_eq!(term, expected);
    }
  }
}
