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
                /*if t0.clone() == Term::A && t1.clone() == Term::A {
                    print!("here!")
                }*/
                match mgu(&t0, &t1) {
                    None => continue,
                    Some(mu) => {
                        let mut c_new = Class::new();
                        let mut vterms = Vec::new();
                        let mut vconstraints = Vec::new();
                        for t in c0.terms.clone() {
                            let applied = apply(&mu, &t);
                            if !vterms.contains(&applied) {
                                vterms.push(applied);
                            }
                        }
                        for t in c1.terms.clone() {
                            let applied = apply(&mu, &t);
                            if !vterms.contains(&applied) {
                                vterms.push(applied);
                            }
                        }
                        for t in c0.constraints.clone() {
                            let applied = apply(&mu, &t);
                            if !vconstraints.contains(&applied){
                                vconstraints.push(applied);
                            }
                        }
                        for t in c1.constraints.clone() {
                            let applied = apply(&mu, &t);
                            if !vconstraints.contains(&applied){
                                vconstraints.push(applied);
                            }
                        }
                        c_new.terms = vterms;
                        c_new.constraints = vconstraints;
                        if sat(m, &(c_new.constraints)) {
                            let mut subsumed : bool = false;
                            let mut has_breaked : bool = false;
                            for c in wo.clone() {
                                if check_subsumption(m, &c, &c_new, nb_vars) {
                                    subsumed = true;
                                    has_breaked = true;
                                    break;
                                }
                            }
                            if !has_breaked {
                                for c in us.clone() {
                                    if check_subsumption(m, &c, &c_new, nb_vars) {
                                        subsumed = true;
                                        has_breaked = true;
                                        break;
                                    }
                                }
                                if !has_breaked && check_subsumption(m, &c0, &c_new, nb_vars) {
                                        subsumed = true;
                                }
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
                                for c in us.clone() {
                                    if check_subsumption(m, &c_new, &c, nb_vars) {
                                        pop_value(wo, &c);
                                        pop_value(us, &c);
                                        if c == c0.clone() {
                                            subsumed = true;
                                        }
                                    }
                                }
                                if check_subsumption(m, &c_new, &c0, nb_vars) {
                                        pop_value(wo, &c0);
                                        pop_value(us, &c0);
                                        subsumed = true;
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