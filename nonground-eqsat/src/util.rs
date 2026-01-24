use std::collections::{HashMap, VecDeque};

// I need substitutions, mgu, unifiable, match?
use crate::language::Term;
use crate::class::Class;

pub(crate) type Subst = HashMap<Term,Term>; // invariant to maintain : I only add variables ...

/*pub(crate) fn unifiable(t0: &Term, t1: &Term) -> bool {
    match mgu(&t0, &t1) {
        None => false,
        Some(_) => true,
    }
}*/

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
        Term::Var(_) => {
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
    match t {
        Term::Var(_) => {
            match sigma.get(t).cloned() {
                None => t.clone(),
                Some(s) => s,
            }
        }, 
        Term::A => Term::A,
        Term::B => Term::B,
        Term::C => Term::C,
        Term::F(s) => {
            let term = apply(sigma, &*s);
            Term::F(Box::new(term))
        },
        Term::G(s) => {
            let term = apply(sigma, &*s);
            Term::G(Box::new(term))
        },
        Term::H(s) => {
            let term = apply(sigma, &*s);
            Term::H(Box::new(term))
        },
    }
}

pub(crate) fn pop_value(vdq: &mut VecDeque<Class>, c: &Class) { // -> VecDeque<Class>
    // direclty modifies vdq
    let mut vd : VecDeque<Class> = VecDeque::new();
    let mut b: bool = false;
    for d in vdq.clone() {
        if d == *c && !b {
            b = true;
        } else {
            vd.push_back(d);
        }
    }
    while !vdq.is_empty() {
        vdq.pop_front();
    }
    for d in vd {
        vdq.push_back(d);
    }
    //vdq
}

pub(crate) fn pop_allval(v: &Vec<Term>, t: Term) -> Vec<Term> {
    let mut new_v : Vec<Term> = Vec::new();
    for x in v {
        if *x != t {
            new_v.push(x.clone());
        }
    }
    new_v
}

pub(crate) fn symdiff(v1: Vec<Term>, v2: Vec<Term>) -> Vec<Term> {
    let mut new_v : Vec<Term> = v1.clone();
    for el in v2 {
        new_v = pop_allval(&new_v, el);
    }
    new_v
}