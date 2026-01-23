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
    // dummy implementation as of now as my terms are linear
    match matches(t0, t1) {
        Some(sigma) => Some(sigma),
        None => matches(t1, t0),
    }
}

pub(crate) fn matches(t00: &Term, t11: &Term) -> Option<Subst> {
    // easy because all my terms are linear here so uninteresting
    fn matches_aux(t0: &Term, t1: &Term, mut sigma: Subst) -> Option<Subst> {
        match t0 {
        Term::F(t) => {
            match t1 {
                Term::F(s) => matches(t, s),
                _ => None,
            }
        },
        Term::G(t) => {
            match t1 {
                Term::G(s) => matches(t, s),
                _ => None,
            }
        },
        Term::H(t) => {
            match t1 {
                Term::H(s) => matches(t, s),
                _ => None,
            }
        },
        Term::A => {
            match t1 {
                Term::A => Some(sigma),
                _ => None,
            }
        },
        Term::B => {
            match t1 {
                Term::B => Some(sigma),
                _ => None,
            }
        },
        Term::C => {
            match t1 {
                Term::C => Some(sigma),
                _ => None,
            }
        },
        Term::Var(x) => {
            if sigma.contains_key(t0) {
                let v = sigma.get(t0)?;
                if v == t1 {
                    Some(sigma)
                } else {
                    None
                }
            } else {
                sigma.insert(t0.clone(), t1.clone());
                Some(sigma)
            }
        }
    }
    } 
    matches_aux(t00, t11, HashMap::new())
}

pub(crate) fn apply(sigma: &Subst, t: &Term) -> Term {
    match (*t) {
        Term::Var(x) => todo!(),
        _ => panic!("The application is not possible!")
    }
}

pub(crate) fn apply_vec(sigma: &Subst, v: &Vec<Term>) -> Vec<Term> {
    let mut new_v : Vec<Term> = Vec::new();
    for t in v {
        new_v.push(t.clone());
    }
    new_v
}