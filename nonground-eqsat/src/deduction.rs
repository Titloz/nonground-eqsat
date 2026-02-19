use std::collections::VecDeque;

use crate::class::Class;
use crate::deduction_intern::deduct_intern;
use crate::language::Term;
use crate::util::pop_value;

pub(crate) fn deduct(m: &Vec<Term>, wo: &mut VecDeque<Class>, us: &mut VecDeque<Class>, c0: Class, nb_vars: &mut usize) -> bool {
    print!("deduct, c0:\n{}\n", c0.clone());
    // the if is experimental
    if !wo.contains(&c0) {
        wo.push_back(c0.clone());
    }
    // for each f in Term with arity(f)=n and n>0 
    let f0 : Term = Term::F(Box::new(Term::Var(*nb_vars))); 
    *nb_vars += 1;
    let f1 : Term = Term::F(Box::new(Term::Var(*nb_vars)));
    *nb_vars += 1;
    if !deduct_intern(m, wo, us, f0, f1, 0, 1, &c0, &mut Class::new(), false, nb_vars) {
        pop_value(wo, &c0);
        return false;
    }
    let g0 : Term = Term::G(Box::new(Term::Var(*nb_vars)));
    *nb_vars += 1;
    let g1 : Term = Term::G(Box::new(Term::Var(*nb_vars)));
    *nb_vars += 1;
    if !deduct_intern(m, wo, us, g0, g1, 0, 1, &c0, &mut Class::new(), false, nb_vars) {
        pop_value(wo, &c0);
        return false;
    }
    let h0 : Term = Term::H(Box::new(Term::Var(*nb_vars))); 
    *nb_vars += 1;
    let h1 : Term = Term::H(Box::new(Term::Var(*nb_vars)));
    *nb_vars += 1;
    if !deduct_intern(m, wo, us, h0, h1, 0, 1, &c0, &mut Class::new(), false, nb_vars) {
        pop_value(wo, &c0);
        return false;
    }
    true
}