use crate::class::Class;
use crate::util::{matches, Subst, apply};
use crate::smt::{build_lac, la_implication_test};

pub(crate) fn check_subsumption(c0: &Class, c1: &Class) -> bool {
    if (&c0).sepvars().is_empty() {
        check_subsumption_fv(c0, c1, &Subst::new()) //, empty_subst
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
                        if check_subsumption_fv(c0, c1, &new_sigma) {
                            return true;
                        }
                    },
                }
            }
        }
        false
    }
}

fn check_subsumption_fv(c0: &Class, c1: &Class, sigma: &Subst) -> bool { 
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
                    // we test if the constraints are verified
                    // i might want to replace it with an explicit representation of M
                    // according to def 20, what I need to test is :
                    // forall delta. (delta(c1.constraints) => exists delta'. delta'(delta(tau(sigma(c0.constraints)))))
                    let lac0 = build_lac(&v);
                    let lac1 = build_lac(&c1.constraints);
                    if la_implication_test(lac0, lac1) {
                        result = true;
                        break;
                        // does break have the correct semantics here?
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