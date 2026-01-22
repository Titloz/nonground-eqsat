mod language;
mod class;
mod merge;
mod deduction;
mod deduction_intern;
mod subsumption;
mod util;


use std::collections::VecDeque;

use crate::language::{Term, Equality};
use crate::class::Class;

fn nongroundcc(e: VecDeque<Equality>) {
    // the data structures here are probably to change
    let mut us : VecDeque<Class> = VecDeque::new();
    let mut wo : VecDeque<Class> = VecDeque::new();
    // I also need to keep a PI in memory, this is what I will return
    for eq in e {
        let s : Term = eq.lhs;
        let t: Term = eq.rhs;
        let mut cnew : Class = Class::new();
        cnew.terms.push(s.clone());
        cnew.terms.push(t.clone());
        cnew.constraints.push(s.clone());
        cnew.constraints.push(t.clone());
    }

}


fn main() {
    
}
