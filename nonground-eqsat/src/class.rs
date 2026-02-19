use std::{collections::HashSet, fmt::Display};

use crate::language::Term;

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Class {
    pub terms: Vec<Term>,
    pub constraints: Vec<Term>,
}

impl Clone for Class {
    fn clone(&self) -> Self {
        Self {
            terms: self.terms.clone(),
            constraints: self.constraints.clone(),
        }
    }
}

impl Class {
    pub(crate) fn new() -> Self {
        Self {
            terms: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub(crate) fn sepvars(&self) -> HashSet<usize> {
        //let mut set = HashSet::new();
        if !self.terms.is_empty() {
            let mut set = self.terms[0].get_vars();
            let mut to_delete = Vec::new();
            for t in &self.terms {
                let v = t.get_vars();
                let vector : Vec<usize> = (&set).clone().into_iter().collect();
                for x in vector {
                    if !v.contains(&x) {
                        to_delete.push(x);
                    }
                }
                for x in to_delete {
                    let _ = &mut set.remove(&x);
                }
                to_delete = Vec::new();
            }
            set
        } else {
            HashSet::new()
        }
    }

    pub(crate) fn get_terms_vars(&self) -> HashSet<usize> {
        let mut set = HashSet::new();
        for t in &self.terms {
            let v = t.get_vars();
            for x in v {
                set.insert(x);
            }
        }
        set
    }

    pub(crate) fn get_constraint_vars(&self) -> HashSet<usize> {
        let mut set = HashSet::new();
        for t in &self.constraints {
            let v = t.get_vars();
            for x in v {
                set.insert(x);
            }
        }
        set
    }

    pub(crate) fn get_vars(&self) -> HashSet<usize> {
        let mut set = self.get_terms_vars();
        for x in self.get_constraint_vars() {
            set.insert(x);
        }
        set
    }

    pub(crate) fn share_vars(&self, other: &Class) -> bool {
        let self_vars = self.get_vars();
        let other_vars = other.get_vars();
        for x in other_vars.clone() {
            if !self_vars.contains(&x) {
                return false;
            }
        }
        for x in self_vars {
            if !other_vars.contains(&x) {
                return false;
            }
        }
        true
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "==== Class ====\n");
        let _ = write!(f, "Terms:\n");
        for t in &self.terms {
            let _ = write!(f, "{},\n", t);
        }
        let _ = write!(f, "\nConstraints:\n");
        for c in &self.constraints {
            let _ = write!(f, "{},\n", c);
        }
        write!(f,"\n")
    }
}
