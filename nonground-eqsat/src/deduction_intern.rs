use std::collections::VecDeque;

use crate::class::Class;
use crate::subsumption::check_subsumption;
use crate::language::Term;
use crate::util::{symdiff, pop_value};
use crate::smt::sat;

pub(crate) fn deduct_intern(m: &Vec<Term>, wo: &mut VecDeque<Class>, us: &mut VecDeque<Class>, t0: Term, t1: Term, i: usize, n: usize, c0: &Class, c_new: &mut Class, used: bool, nb_vars: &mut usize) -> bool {
    print!("deduct_intern, used: {}, c0:\n{}\n, c_new:\n{}\n", used, c0.clone(), c_new.clone());
    if i != n {
        for c in wo.clone() {
            let cbis = &c.clone();
            let cter = &c.clone();
            let cquadr = &c.clone();
            let cquint = &c.clone();
            for x in &cbis.constraints {
                if !c_new.constraints.contains(&x) {
                    c_new.constraints.push(x.clone());
                }
            }
            for s0 in &cter.terms {
                for s1 in &cquadr.terms {
                    // t0 = t0[s0]_i
                    // t1 = t1[s1]_i
                    let t0_new : Term = match t0.clone() {
                        Term::F(t) => {
                            match *t {
                                Term::Var(_) => {
                                    let b = Box::new(s0.clone());
                                    Term::F(b)
                                },
                                _ => t0.clone(),
                            }
                        },
                        Term::G(t) => {
                            match *t {
                                Term::Var(_) => {
                                    let b = Box::new(s0.clone());
                                    Term::G(b)
                                },
                                _ => t0.clone(),
                            }
                        },
                        Term::H(t) => {
                            match *t {
                                Term::Var(_) => {
                                    let b = Box::new(s0.clone());
                                    Term::H(b)
                                },
                                _ => t0.clone(),
                            }
                        },
                        _ => t0.clone(),
                    };
                    let t1_new = match t1.clone() {
                        Term::F(t) => {
                            match *t {
                                Term::Var(_) => {
                                    let b = Box::new(s1.clone());
                                    Term::F(b)
                                },
                                _ => t1.clone(),
                            }
                        },
                        Term::G(t) => {
                            match *t {
                                Term::Var(_) => {
                                    let b = Box::new(s1.clone());
                                    Term::G(b)
                                },
                                _ => t1.clone(),
                            }
                        },
                        Term::H(t) => {
                            match *t {
                                Term::Var(_) => {
                                    let b = Box::new(s1.clone());
                                    Term::H(b)
                                },
                                _ => t1.clone(),
                            }
                        },
                        _ => t1.clone(),
                    };
                    if !deduct_intern(m, wo, us, t0_new, t1_new, i+1, n, c0, c_new, (c == *c0) || used, nb_vars) {
                        return false;
                    }
                }
            }
            let cnewnew = c_new.clone();
            c_new.constraints = symdiff(cnewnew.constraints, cquint.constraints.clone());
        }
    } else if used {
        c_new.terms = Vec::new();
        c_new.terms.push(t0.clone());
        if !c_new.terms.contains(&t1) {
            c_new.terms.push(t1.clone());
        }
        // avoid doublons
        // still weird to me to keep the old constraints ...
        if !c_new.constraints.contains(&t0) {
            c_new.constraints.push(t0.clone());
        }
        if !c_new.constraints.contains(&t1) {
            c_new.constraints.push(t1.clone());
        }
        if sat(m, &(c_new.constraints)) {
            for c in wo.clone() {
                if check_subsumption(m, &c, &c_new, nb_vars) {
                    return true;
                }
            }
            for c in us.clone() {
                if check_subsumption(m, &c, &c_new, nb_vars) {
                    return true;
                }
            }
            let mut subsumed : bool = false;
            for c in wo.clone() {
                if check_subsumption(m, &c_new, &c, nb_vars) {
                    pop_value(wo, &c);
                    pop_value(us, &c);
                    if c==(*c0) {
                        subsumed = true;
                    }
                }
            }
            for c in us.clone() {
                if check_subsumption(m, &c_new, &c, nb_vars) {
                    pop_value(wo, &c);
                    pop_value(us, &c);
                    if c==(*c0) {
                        subsumed = true;
                    }
                }
            }
            // if condition not needed as we test for subsumption
            //if !us.contains(c_new) { 
            // test to "correct" their pseudo-code.
            us.push_back(c_new.clone());
            //} 
            if subsumed {
                return false;
            }
        }
    }
    return true;
}