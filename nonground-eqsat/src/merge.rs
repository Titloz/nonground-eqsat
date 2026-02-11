use std::collections::VecDeque;

use crate::class::Class;
use crate::subsumption::check_subsumption;
use crate::util::{mgu, apply, pop_value};
use crate::smt::sat;
use crate::language::Term;

pub(crate) fn merge(m: &Vec<Term>, wo: &mut VecDeque<Class>, us: &mut VecDeque<Class>, c0: Class, nb_vars: &mut usize) -> bool {
    for c1 in wo.clone() {
        for t0 in c0.terms.clone() {
            for t1 in c1.terms.clone() {
                match mgu(&t0, &t1) {
                    None => continue,
                    Some(mu) => {
                        let mut c_new = Class::new();
                        let mut vterms = Vec::new();
                        let mut vconstraints = Vec::new();
                        for t in c0.terms.clone() {
                            vterms.push(apply(&mu, &t));
                        }
                        for t in c1.terms.clone() {
                            vterms.push(apply(&mu, &t));
                        }
                        for t in c0.constraints.clone() {
                            vconstraints.push(apply(&mu, &t));
                        }
                        for t in c1.constraints.clone() {
                            vconstraints.push(apply(&mu,&t));
                        }
                        c_new.terms = vterms;
                        c_new.constraints = vconstraints;
                        if sat(m, &(c_new.constraints)) {
                            let mut subsumed : bool = false;
                            for c in wo.clone() {
                                if check_subsumption(m, &c, &c_new, nb_vars) {
                                    subsumed = true;
                                    break;
                                }
                            }
                            for c in us.clone() {
                                if check_subsumption(m, &c, &c_new, nb_vars) {
                                    subsumed = true;
                                    break;
                                }
                            }
                            if check_subsumption(m, &c0, &c_new, nb_vars) {
                                    subsumed = true;
                            }
                            if !subsumed {
                                for c in wo.clone() {
                                    if check_subsumption(m, &c_new, &c, nb_vars) {
                                        pop_value(wo, &c);
                                        pop_value(us, &c);
                                        if c == c0.clone() {
                                            subsumed = true;
                                        }
                                    }
                                }
                                us.push_back(c_new);
                                if subsumed {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    true
}