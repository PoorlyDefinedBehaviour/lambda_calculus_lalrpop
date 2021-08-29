use crate::ast::Term;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn eval(term: &Term) -> Result<Term, String> {
  match term {
    Term::Var(x) => Err(format!("undefined variable {}", x)),
    Term::Abs(_, _) => Ok(term.clone()),
    Term::App(f, arg) => eval_app(f, arg),
  }
}

fn free_variables(term: &Term) -> HashSet<String> {
  match term {
    Term::Var(x) => {
      let mut set = HashSet::new();
      set.insert(x.clone());
      set
    }
    Term::Abs(x, body) => {
      let mut vars = free_variables(body);
      vars.remove(x);
      vars
    }
    Term::App(f, arg) => {
      let f_vars = free_variables(f);
      let arg_vars = free_variables(arg);
      f_vars.union(&arg_vars).cloned().collect()
    }
  }
}

fn gensym() -> String {
  static COUNTER: AtomicUsize = AtomicUsize::new(0);
  let i = COUNTER.fetch_add(1, Ordering::Relaxed);
  format!("${}", i)
}

fn substitute(term: &Term, var: &String, new_value: &Term) -> Term {
  match term {
    Term::Var(x) => {
      if x == var {
        new_value.clone()
      } else {
        term.clone()
      }
    }
    Term::App(f, arg) => {
      let f_with_substitutions = substitute(f, var, new_value);
      let arg_with_substitutions = substitute(arg, var, new_value);

      Term::App(
        Box::new(f_with_substitutions),
        Box::new(arg_with_substitutions),
      )
    }
    Term::Abs(x, body) => {
      if x != var && !free_variables(new_value).contains(x) {
        Term::Abs(x.clone(), Box::new(substitute(body, var, new_value)))
      } else {
        let new_var = gensym();
        let new_var_term = Term::Var(new_var);
        let body_with_new_var = substitute(body, x, &new_var_term);
        Term::App(
          Box::new(new_var_term),
          Box::new(substitute(&body_with_new_var, var, new_value)),
        )
      }
    }
  }
}

fn eval_app(f: &Term, arg: &Term) -> Result<Term, String> {
  match f {
    Term::Abs(var, body) => {
      let evaluated_arg = eval(arg)?;

      // Given the expression ((λx.x) 2)
      // we are recreating the expression (λx.x) with x substituted by 2
      // (λx.x){2/x}
      // ==> (λx.2)
      Ok(substitute(body, var, &evaluated_arg))
    }
    _ => Err(format!("tried to apply {:?} to {:?}", f, arg)),
  }
}
