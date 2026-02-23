use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Term {
    F(Box<Term>),
    G(Box<Term>),
    H(Box<Term>),
    Var(usize),
    A,
    B,
    C,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::F(t) => write!(f, "f({})", *t),
            Term::G(t) => write!(f, "g({})", *t),
            Term::H(t) => write!(f, "h({})", *t),
            Term::Var(x) => write!(f, "x_{}", x),
            Term::A => write!(f, "a"),
            Term::B => write!(f, "b"),
            Term::C => write!(f, "c"),
        }
    }
}

impl Term {
    pub(crate) fn get_vars(&self) -> HashSet<usize> {
        let mut set = HashSet::new();
        fn traverse(t: &Term, set: &mut HashSet<usize>) {
            match t {
                Term::F(s) => {
                    traverse(&*s, set);
                },
                Term::G(s) => {
                    traverse(&*s, set);
                },
                Term::H(s) => {
                    traverse(&*s, set);
                },
                Term::Var(x) => {
                    if !set.contains(&x){
                       set.insert(*x); 
                    }
                },
                _ => (),
            }
        }
        traverse(self, &mut set);
        set
    }

    pub(crate) fn is_ground(&self) -> bool {
        self.get_vars().is_empty()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Equality {
    pub lhs: Term,
    pub rhs: Term,
}