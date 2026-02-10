use std::collections::{HashMap, HashSet};

use crate::language::Term;
use crate::util::{Subst, apply, matches_all};

pub(crate) fn sat(m: &Vec<Term>, gamma: &Vec<Term>) -> bool {
    //let gamma = &c.constraints;
    /* 
    let g2 = gamma.clone();
    for t in g2 {
        let deltas = matches_all(&t, &gamma);
        if deltas.is_empty() {
            return false
        } else {
            for mut d in deltas {

            }
        }
    }
    */
    true
}

pub(crate) fn implication_test(c0 : &Vec<Term>, c1 : &Vec<Term>, m0 : &Vec<Term>) -> bool {
    fn aux(gamma: &Vec<Term>, dstau : &Vec<Term>, l : &mut Vec<Subst>, m : &Vec<Term>) -> bool {
        if l.is_empty() {
            true
        } else {
            let delta : Subst = l.pop().expect("");
            // if dom(delta) == vars(gamma)
            let dom_delta = delta.len(); 
            let mut vars_gamma = HashSet::new();
            for t in gamma.clone() {
                let vars = t.get_vars();
                for x in vars {
                    vars_gamma.insert(x);
                }
            }
            let nb_vars_gamma = vars_gamma.len();
            if dom_delta == nb_vars_gamma {
                let mut dstau_bis : Vec<Term> = Vec::new();
                for t in dstau.clone() {
                    dstau_bis.push(apply(&delta, &t));
                }
                for t in dstau_bis {
                    if matches_all(&t, m).is_empty() {
                        return false;
                    }
                }
                aux(gamma, dstau, l, m)
            } else {
                // tj = first_non_ground(gamma), we know for a fact that it exists because of the if branching 
                let g2 = gamma.clone();
                let mut tj : Term = Term::A;
                for x in g2 {
                    if !x.is_ground() {
                        tj = x;
                        break;
                    }
                }
                let deltas = matches_all(&tj, m);
                for mut d in deltas {
                    let mut b : bool = true;
                    // i should modify gamma to know where are the new ground terms
                    // here I want to iterate over every new ground terms
                    // for a first impl, just test for every ground term
                    let mut ground_gamma = Vec::new();
                    let g3 = gamma.clone();
                    for x in g3 {
                        let y = apply(&d, &x);
                        if y.is_ground() {
                            ground_gamma.push(y);
                        }
                    }
                    for t in ground_gamma {
                        if matches_all(&t, m).is_empty() {
                            b = false;
                        }
                    }
                    if b {
                        // those operations might be weird in memory let's see what happens there
                        for (k,v) in &delta {
                            d.insert(*k, v.clone());
                        }
                        l.push(d);
                    }
                }
                aux(gamma, dstau, l, m)
            }
        }
    }

    let sigma : Subst = HashMap::new();
    let mut v : Vec<Subst> = Vec::new();
    v.push(sigma);
    aux(c0, c1, &mut v, m0)
}