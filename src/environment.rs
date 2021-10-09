use crate::ast;
use std::collections::HashMap;
use std::convert::From;
use std::ops::Deref;

type Environment = HashMap<String, Term>;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
  Int(i32),
  Var(String),
  Abs(String, Box<Term>),
  App(Box<Term>, Box<Term>),
  Closure(Environment, String, Box<Term>),
}

impl From<&ast::Term> for Term {
  fn from(term: &ast::Term) -> Self {
    match term {
      ast::Term::Int(x) => Term::Int(*x),
      ast::Term::Var(x) => Term::Var(x.clone()),
      ast::Term::Abs(x, body) => Term::Abs(x.clone(), Box::new(Self::from(body.deref()))),
      ast::Term::App(f, arg) => Term::App(
        Box::new(Self::from(f.deref())),
        Box::new(Self::from(arg.deref())),
      ),
    }
  }
}

impl From<Term> for ast::Term {
  fn from(item: Term) -> ast::Term {
    match item {
      Term::Int(x) => ast::Term::Int(x),
      Term::Var(x) => ast::Term::Var(x),
      Term::Closure(_env, x, body) => ast::Term::Abs(x, Box::new(Self::from(*body))),
      Term::Abs(x, body) => ast::Term::Abs(x, Box::new(Self::from(*body))),
      Term::App(f, arg) => ast::Term::App(Box::new(Self::from(*f)), Box::new(Self::from(*arg))),
    }
  }
}

fn eval_impl(env: &Environment, term: Term) -> Result<Term, String> {
  match term {
    Term::Int(_) => Ok(term),
    Term::Var(x) => match env.get(&x) {
      None => Err(format!("undefined variable {}", x)),
      Some(term) => Ok(term.clone()),
    },
    Term::Abs(x, body) => Ok(Term::Closure(env.clone(), x, body)),
    Term::Closure(_, _, _) => Ok(term.clone()),
    Term::App(f, arg) => match eval_impl(env, f.deref().clone())? {
      Term::Closure(mut closure_env, x, body) => {
        let evaluated_arg = eval_impl(env, *arg)?;

        closure_env.insert(x, evaluated_arg);

        eval_impl(&closure_env, *body)
      }
      _ => Err(format!("tried to apply {:?} to {:?}", f, arg)),
    },
  }
}

pub fn eval(term: &ast::Term) -> Result<ast::Term, String> {
  eval_impl(&Environment::new(), Term::from(term)).map(|term| term.into())
}
