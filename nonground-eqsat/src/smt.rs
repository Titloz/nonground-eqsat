use std::collections::{HashMap, HashSet};

use crate::language::Term;
use crate::util::{Subst, apply, matches_all}; // , print_subst

/* 
pub(crate) fn _sat(m0: &Vec<Term>, gamma: &Vec<Term>) -> bool {

    fn aux(m: &Vec<Term>, g: &Vec<Term>, l: &mut Vec<Subst>) -> bool {
        //print!("sat_aux\n");
        if l.is_empty() {
            false
        } else {
            // get the variables appearing in delta and in gamma
            let delta : Subst = l.pop().expect("");
            let dom_delta = delta.len();
            let mut vars_gamma = HashSet::new();
            for t in g.clone() {
                let vars = t.get_vars();
                for x in vars {
                    vars_gamma.insert(x);
                }
            }
            let nb_vars_gamma = vars_gamma.len();
            if dom_delta == nb_vars_gamma {
                // delta is grounding for gamma
                let applied = g.clone().into_iter().map(|t| apply(&delta, &t));
                // we check that every term is indeed in M
                for t in applied {
                    if matches_all(&t, m).is_empty() {
                        return aux(m, g, l);
                    }
                }
                true
            } else {
                let applied = g.clone().into_iter().map(|t| apply(&delta, &t));
                // check that every ground term is in M
                for t in applied.clone() {
                    if t.is_ground() && matches_all(&t, m).is_empty() {
                        // if not, test an other substitution
                        return aux(m, g, l);
                    }
                }
                // extend delta
                for t in applied {
                    // one must exist
                    if !t.is_ground() {
                        let deltas = matches_all(&t, m); 
                        for mut d in deltas {
                            for (k,v) in delta.clone() {
                                d.insert(k, v);
                            }
                            if !l.contains(&d) {
                                l.push(d);
                            }
                        }
                        break
                    }
                }
                aux(m, g, l)
            }
        }
    }
    
    let mut l = Vec::new();
    let d : Subst = HashMap::new();
    l.push(d);
    aux(m0, gamma, &mut l) 
    
}
*/
// let's rewrite sat to not be recursive
pub(crate) fn sat(m : &Vec<Term>, gamma : &Vec<Term>) -> bool {
    let mut l = Vec::new();
    let d : Subst = HashMap::new();
    l.push(d);

    while !l.is_empty() {
        // get the variables appearing in delta and in gamma
        let delta : Subst = l.pop().expect("");
        let dom_delta = delta.len();
        let mut vars_gamma = HashSet::new();
        for t in gamma.clone() {
            let vars = t.get_vars();
            for x in vars {
                vars_gamma.insert(x);
            }
        }
        let nb_vars_gamma = vars_gamma.len();
        if nb_vars_gamma == dom_delta {
            // delta is grounding for gamma
            let applied = gamma.clone().into_iter().map(|t| apply(&delta, &t));
            let mut are_in_m = true;
            // we check that every term is indeed in M
            for t in applied {
                if matches_all(&t, m).is_empty() {
                    //return aux(m, g, l);
                    are_in_m = false;
                    break;
                }
            }
            if are_in_m {
                return true;
            }
        } else {
            let applied = gamma.clone().into_iter().map(|t| apply(&delta, &t));
            let mut bad_subst = false;
            // check that every ground term is in M
            for t in applied.clone() {
                if t.is_ground() && matches_all(&t, m).is_empty() {
                    // if not, test an other substitution
                    bad_subst = true;
                    break
                }
            }
            if !bad_subst {
                // extend delta
                for t in applied {
                    // one must exist
                    if !t.is_ground() {
                        let deltas = matches_all(&t, m); 
                        for mut d in deltas {
                            for (k,v) in delta.clone() {
                                d.insert(k, v);
                            }
                            if !l.contains(&d) {
                                l.push(d);
                            }
                        }
                        break
                    }
                }
            }
        }
    }
    false
}

/*
pub(crate) fn _implication_test(c0 : &Vec<Term>, c1 : &Vec<Term>, m0 : &Vec<Term>) -> bool {

    fn aux(gamma: &Vec<Term>, dstau : &Vec<Term>, l : &mut Vec<Subst>, m : &Vec<Term>) -> bool {
        print!("implication_test_aux with l =\n");
        for s in l.clone() {
            print_subst(s);
        }
        print!("\n");
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
                // tj = first_non_ground(gamma), we know for a fact that it exists because dom(delta) != vars(gamma)
                let g2 = gamma.clone();
                let mut tj : Term = Term::A;
                let mut should_break = false;
                for x in g2 {
                    for v in x.get_vars() {
                        if !delta.contains_key(&v) {
                            tj = x;
                            should_break = true;
                            break;
                        }
                    }
                    if should_break {
                        break;
                    }
                }
                let deltas = matches_all(&tj, m);
                for mut d in deltas {
                    let mut d_is_acceptable : bool = true;
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
                            d_is_acceptable = false;
                        }
                    }
                    if d_is_acceptable {
                        for (k,v) in &delta {
                            d.insert(*k, v.clone());
                        }
                        if !l.contains(&d) {
                            l.push(d);
                        }
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
*/

// I must rewrite the implication_test to not be recursive. 

pub(crate) fn implication_test(gamma : &Vec<Term>, dstau : &Vec<Term>, m : &Vec<Term>) -> bool {
    let mut l : Vec<Subst> = Vec::new();
    let empty : Subst = Subst::new();
    l.push(empty);

    while !l.is_empty() {
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
        } else {
            // tj = first_non_ground(gamma), we know for a fact that it exists because dom(delta) != vars(gamma)
            let g2 = gamma.clone();
            let mut tj : Term = Term::A;
            let mut should_break = false;
            for x in g2 {
                for v in x.get_vars() {
                    if !delta.contains_key(&v) {
                        tj = x;
                        should_break = true;
                        break;
                    }
                }
                if should_break {
                    break;
                }
            }
            let deltas = matches_all(&tj, m);
            for mut d in deltas {
                let mut d_is_acceptable : bool = true;
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
                        d_is_acceptable = false;
                    }
                }
                if d_is_acceptable {
                    for (k,v) in &delta {
                        d.insert(*k, v.clone());
                    }
                    if !l.contains(&d) {
                        l.push(d);
                    }
                }
            }
        }
    }
    true
}