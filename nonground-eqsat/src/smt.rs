use std::collections::HashMap;

use crate::class::Class;
use crate::language::Term;
use crate::util::{Subst, apply, matches, matches_all};

pub(crate) fn sat(c: &Class) -> bool {
    todo!("sat solver for c.constraints")
}

type LA = u8; // to change obviously

pub(crate) fn build_lac(v: &Vec<Term>) -> LA {
    todo!("buildLAC")
}

pub(crate) fn la_implication_test(lac0 : LA, lac1 : LA) -> bool {
    todo!("laImplicationTest")
}

pub(crate) fn implication_test(c0 : &Vec<Term>, c1 : &Vec<Term>, m0 : &Vec<Term>) -> bool {
    fn aux(gamma: &Vec<Term>, dstau : &Vec<Term>, l : &mut Vec<Subst>, m : &Vec<Term>) -> bool {
        if l.is_empty() {
            true
        } else {
            let delta : Subst = l.pop().expect("");
            // if dom(delta) == vars(gamma)
            if true {
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
                // tj = first_non_ground(gamma)
                let tj = &gamma[0];
                let deltas = matches_all(&tj, m);
                for mut d in deltas {
                    let mut b : bool = true;
                    // i should modify gamma to know where are the new ground terms
                    // here I want to iterate over every new ground terms
                    for t in gamma {
                        if matches_all(t, m).is_empty() {
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