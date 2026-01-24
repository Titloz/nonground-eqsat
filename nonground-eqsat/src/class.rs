use std::collections::HashSet;

use crate::language::Term;

#[derive(PartialEq)]
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
}
