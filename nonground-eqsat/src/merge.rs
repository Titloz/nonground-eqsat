use std::collections::VecDeque;

use crate::class::Class;
use crate::subsumption::check_subsumption;
use crate::util::{mgu, apply, pop_value, rename};
use crate::smt::sat;
use crate::language::Term;

pub(crate) fn merge(m: &Vec<Term>, wo: &mut VecDeque<Class>, us: &mut VecDeque<Class>, c0: Class, nb_vars: &mut usize) -> bool {
    print!("merge, c0:{}\n", c0.clone());
    //let sv0 = c0.sepvars();
    for c1 in wo.clone() {
        let mut c2 = c1.clone();
        //let sv = c1.sepvars();
        if c1.share_vars(&c0) { // c1.clone() == c0.clone()      !sv.is_empty() && sv == sv0
            // in that case, we should operate on a renaming of c1!
            c2 = rename(c1, nb_vars);
            //continue;
        }
        for t0 in c0.terms.clone() {
            for t1 in c2.terms.clone() {
                //print!("t0 = {}\n", t0);
                //print!("t1 = {}\n", t1);
                match mgu(&t0, &t1) {
                    None => continue,
                    Some(mu) => {
                        //print!("ok\n");
                        let mut c_new = Class::new();
                        let mut vterms = Vec::new();
                        let mut vconstraints = Vec::new();
                        for t in c0.terms.clone() {
                            let applied = apply(&mu, &t);
                            if !vterms.contains(&applied) {
                                vterms.push(applied);
                            }
                        }
                        for t in c2.terms.clone() {
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
                        for t in c2.constraints.clone() {
                            let applied = apply(&mu, &t);
                            if !vconstraints.contains(&applied){
                                vconstraints.push(applied);
                            }
                        }
                        c_new.terms = vterms;
                        c_new.constraints = vconstraints;
                        // print!("cnew.constraints: \n");
                        // for c in c_new.constraints.clone() {
                        //     print!("{}\n", c);
                        // }
                        //print!("before sat\n");
                        let sat = sat(m, &(c_new.constraints));
                        //print!("is sat : {}\n", sat);
                        if sat {
                            let mut subsumed : bool = false;
                            let mut has_breaked : bool = false;
                            for c in wo.clone() {
                                if check_subsumption(m, &c, &c_new, nb_vars) {
                                    subsumed = true;
                                    has_breaked = true;
                                    //print!("is subsumed by {} in wo\n", c.clone());
                                    break;
                                }
                            }
                            if !has_breaked {
                                for c in us.clone() {
                                    if check_subsumption(m, &c, &c_new, nb_vars) {
                                        subsumed = true;
                                        has_breaked = true;
                                        //print!("is subsumed by {} in us\n", c.clone());
                                        break;
                                    }
                                }
                                if !has_breaked && check_subsumption(m, &c0, &c_new, nb_vars) {
                                        subsumed = true;
                                        //print!("is subsumed by c0={}\n", c0.clone());
                                }
                            }
                            if !subsumed {
                                for c in wo.clone() {
                                    // does enter check_subsumption
                                    if check_subsumption(m, &c_new, &c, nb_vars) {
                                        //print!("subsumes {} in wo\n", c.clone());
                                        pop_value(wo, &c);
                                        pop_value(us, &c);
                                        if c == c0.clone() {
                                            subsumed = true;
                                        }
                                    }
                                    //print!("ok"); 
                                }
                                for c in us.clone() {
                                    if check_subsumption(m, &c_new, &c, nb_vars) {
                                        //print!("subsumes {} in us\n", c.clone());
                                        pop_value(wo, &c);
                                        pop_value(us, &c);
                                        if c == c0.clone() {
                                            subsumed = true;
                                        }
                                    }
                                }
                                if check_subsumption(m, &c_new, &c0, nb_vars) {
                                        //print!("subsumes c0={}\n", c0.clone());
                                        pop_value(wo, &c0);
                                        pop_value(us, &c0);
                                        subsumed = true;
                                }
                                if !subsumed {
                                    //print!("is not subsumed\n");
                                    continue;
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
    print!("end merge\n");
    true
}