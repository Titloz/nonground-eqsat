use crate::class::Class;
use crate::util::{matches, Subst, apply};
use crate::smt::{implication_test};
use crate::language::Term;

pub(crate) fn check_subsumption(m: &Vec<Term>, c0: &Class, c1: &Class, nb_vars: &mut usize) -> bool {
    if (&c0).sepvars().is_empty() {
        check_subsumption_fv(m, c0, c1, &Subst::new(), nb_vars) 
    } else {
        for t0 in c0.terms.clone() {
            for t1 in c1.terms.clone() {
                match matches(&t0, &t1) {
                    None => continue,
                    Some(sigma) => {
                        // sigma -= {x->t | x-> is in sigma and x is a free variable} 
                        let sv = (&c0).sepvars();
                        let mut new_sigma : Subst = Subst::new();
                        for x in sv {
                            match sigma.get(&x) {
                                None => continue,
                                Some(t) => {new_sigma.insert(x, t.clone());},
                            }
                        }
                        if check_subsumption_fv(m, c0, c1, &new_sigma, nb_vars) {
                            return true;
                        }
                    },
                }
            }
        }
        false
    }
}

fn check_subsumption_fv(m: &Vec<Term>, c0: &Class, c1: &Class, sigma: &Subst, _nb_vars: &mut usize) -> bool { 
    // checks if c0 subsumes c1
    for t1 in c1.terms.clone() {
        let mut result: bool = false;
        for t0 in c0.terms.clone() {
            let s = apply(sigma, &t0);
            match matches(&s, &t1) {
                None => continue,
                Some(tau) => {
                    let mut v = Vec::new();
                    for term in c0.constraints.clone() {
                        let tsigma = apply(sigma, &term);
                        let ttau = apply(&tau, &tsigma);
                        v.push(ttau);
                    }
                    if implication_test(&(c1.constraints), &(c0.constraints), m) {
                        result = true;
                        break;
                    }
                },
            }
        }
        if !result {
            return false;
        }
    }
    return true;
}