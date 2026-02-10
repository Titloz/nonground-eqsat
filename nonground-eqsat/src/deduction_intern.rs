use std::collections::VecDeque;

use crate::class::Class;
use crate::subsumption::check_subsumption;
use crate::language::Term;
use crate::util::{symdiff, pop_value};
use crate::smt::sat;

pub(crate) fn deduct_intern(m: &Vec<Term>, wo: &mut VecDeque<Class>, us: &mut VecDeque<Class>, t0: Term, t1: Term, i: usize, n: usize, c0: &Class, c_new: &mut Class, used: bool) -> bool {
    if i!=n {
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
                    //todo!("see comments deduct_intern");
                    // t0 = t0[s0]_i
                    // t1 = t1[s1]_i
                    if !deduct_intern(m, wo, us, t0.clone(), t1.clone(), i+1, n, c0, c_new, (c == *c0) || used) {
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
        c_new.terms.push(t1.clone());
        c_new.constraints.push(t0.clone());
        c_new.constraints.push(t1.clone());
        if sat(m, &(c_new.constraints)) {
            for c in wo.clone() {
                if check_subsumption(m, &c, &c_new) {
                    return true;
                }
            }
            for c in us.clone() {
                if check_subsumption(m, &c, &c_new) {
                    return true;
                }
            }
            let mut subsumed : bool = false;
            for c in wo.clone() {
                if check_subsumption(m, &c_new, &c) {
                    pop_value(wo, &c);
                    pop_value(us, &c);
                    if c==(*c0) {
                        subsumed = true;
                    }
                }
            }
            for c in us.clone() {
                if check_subsumption(m, &c_new, &c) {
                    pop_value(wo, &c);
                    pop_value(us, &c);
                    if c==(*c0) {
                        subsumed = true;
                    }
                }
            }
            us.push_back(c_new.clone());
            if subsumed {
                return false;
            }
        }
    }
    return true;
}