use crate::class::Class;
use crate::util::{matches, Subst, apply};
use crate::smt::{build_lAC, la_implication_test};

pub(crate) fn check_subsumption(c0: &Class, c1: &Class) -> bool {
    if (&c0).sepvars().is_empty() {
        check_subsumption_fv(c0, c1, &Subst::new()) //, empty_subst
    } else {
        for t0 in c0.terms.clone() {
            for t1 in c1.terms.clone() {
                match matches(&t0, &t1) {
                    None => continue,
                    Some(sigma) => {
                        // weird conditional here : TODO
                        // sigma -= {x->t | x-> is in sigma and x is a free variable} 
                        if check_subsumption_fv(c0, c1, &sigma) {
                            return true;
                        }
                    },
                }
            }
        }
        false
    }
}

fn check_subsumption_fv(c0: &Class, c1: &Class, sigma: &Subst) -> bool { // sigma: &mut Subst
    for t1 in c1.terms.clone() {
        let mut result: bool = false;
        for t0 in c0.terms.clone() {
            let s = apply(sigma, &t0);
            match matches(&s, &t1) {
                None => continue,
                Some(delta) => {
                    let mut v = Vec::new();
                    for term in c0.constraints.clone() {
                        let tsigma = apply(sigma, &term);
                        let tdelta = apply(&delta, &tsigma);
                        v.push(tdelta);
                    }
                    let lac0 = build_lAC(&v);
                    let lac1 = build_lAC(&c1.constraints);
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