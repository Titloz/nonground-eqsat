use std::collections::HashMap;

// I need substitutions, mgu, unifiable, match?
use crate::language::Term;

pub(crate) type Subst = HashMap<Term,Term>; // invariant to maintain : I only add variables ...

pub(crate) fn unifiable(t0: &Term, t1: &Term) -> bool {
    match mgu(&t0, &t1) {
        None => false,
        Some(_) => true,
    }
}

pub(crate) fn mgu(t0: &Term, t1: &Term) -> Option<Subst> {
    todo!()
}

pub(crate) fn matches(t0: &Term, t1: &Term) -> Option<Subst> {
    todo!()
}

pub(crate) fn apply(sigma: &Subst, t: &Term) -> Term {
    todo!()
}

pub(crate) fn apply_vec(sigma: &Subst, v: &Vec<Term>) -> Vec<Term> {
    let mut new_v : Vec<Term> = Vec::new();
    for t in v {
        new_v.push(t.clone());
    }
    new_v
}