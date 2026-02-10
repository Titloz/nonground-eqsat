mod language;
mod class;
mod merge;
mod deduction;
mod deduction_intern;
mod subsumption;
mod util;
mod smt;


use std::collections::VecDeque;

use crate::language::{Term, Equality};
use crate::class::Class;
use crate::merge::merge;
use crate::deduction::deduct;

fn nongroundcc(e: VecDeque<Equality>, m: &Vec<Term>) {
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
        us.push_back(cnew);
    }

    // deduction classes
    let mut cf : Class = Class::new();
    let mut cg : Class = Class::new();
    let mut ch : Class = Class::new();
    let mut ca : Class = Class::new();
    let mut cb : Class = Class::new();
    let mut cc : Class = Class::new();
    cf.terms.push(Term::F(Box::new(Term::Var(0)))); // indices obviously wrong
    cf.constraints.push(Term::F(Box::new(Term::Var(0))));
    cg.terms.push(Term::G(Box::new(Term::Var(1)))); // indices obviously wrong
    cg.constraints.push(Term::G(Box::new(Term::Var(1))));
    ch.terms.push(Term::H(Box::new(Term::Var(2)))); // indices obviously wrong
    ch.constraints.push(Term::H(Box::new(Term::Var(2))));
    ca.terms.push(Term::A);
    ca.constraints.push(Term::A);
    cb.terms.push(Term::B);
    cb.constraints.push(Term::B);
    cc.terms.push(Term::C);
    cc.constraints.push(Term::C);
    wo.push_back(cf);
    wo.push_back(cg);
    wo.push_back(ch);
    wo.push_back(ca);
    wo.push_back(cb);
    wo.push_back(cc);

    // main loop
    while !us.is_empty() {
        let c : Class = us.pop_front().expect("");
        if merge(m, &mut wo, &mut us, c.clone()) && deduct(m, &mut wo, &mut us, c.clone()){
            wo.push_back(c);
        } 
    }
}


fn main() {
    let equalities : VecDeque<Equality> = VecDeque::new(); // get the equalities
    let m : Vec<Term> = Vec::new();
    nongroundcc(equalities, &m);
}
