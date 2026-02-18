mod language;
mod class;
mod merge;
mod deduction;
mod deduction_intern;
mod subsumption;
mod util;
mod smt;


use std::collections::VecDeque; // HashSet, 

use crate::language::{Term, Equality};
use crate::class::Class;
use crate::merge::merge;
use crate::deduction::deduct;

fn nongroundcc(e: VecDeque<Equality>, m: &Vec<Term>) {
    // DISTINCT VARIABLES BETWEEN DISTINCTS EQUALITIES!
    let mut us : VecDeque<Class> = VecDeque::new();
    let mut wo : VecDeque<Class> = VecDeque::new();
    // nvars denote the first index for which every higher (>=) index does not denote the index of a variable
    let mut nvars : usize = 1;
    // I also need to keep a PI in memory, this is what I will return
    for eq in e {
        let s : Term = eq.lhs;
        let t : Term = eq.rhs;
        let varss = s.get_vars();
        let mut varst = t.get_vars();
        for x in varss {
            varst.insert(x);
        }
        nvars += varst.len();
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
    //let mut ca : Class = Class::new();
    //let mut cb : Class = Class::new();
    //let mut cc : Class = Class::new();
    cf.terms.push(Term::F(Box::new(Term::Var(nvars)))); 
    cf.constraints.push(Term::F(Box::new(Term::Var(nvars))));
    nvars += 1;
    cg.terms.push(Term::G(Box::new(Term::Var(nvars)))); 
    cg.constraints.push(Term::G(Box::new(Term::Var(nvars))));
    nvars += 1;
    ch.terms.push(Term::H(Box::new(Term::Var(nvars)))); 
    ch.constraints.push(Term::H(Box::new(Term::Var(nvars))));
    nvars += 1;
    //ca.terms.push(Term::A);
    //ca.constraints.push(Term::A);
    //cb.terms.push(Term::B);
    //cb.constraints.push(Term::B);
    //cc.terms.push(Term::C);
    //cc.constraints.push(Term::C);
    wo.push_back(cf);
    wo.push_back(cg);
    wo.push_back(ch);
    //wo.push_back(ca);
    //wo.push_back(cb);
    //wo.push_back(cc);

    // until main loop : experimental
    /*let mut wo_set : HashSet<Class> = HashSet::new();
    for c in wo.clone() {
        wo_set.insert(c);
    }*/
    // main loop
    while !us.is_empty() {
        let c : Class = us.pop_front().expect("");
        if merge(m, &mut wo, &mut us, c.clone(), &mut nvars){
            if deduct(m, &mut wo, &mut us, c.clone(), &mut nvars){
                if !wo.contains(&c) {
                    wo.push_back(c);
                }
                // OR
                /*if !wo_set.contains(&c){
                    wo_set.insert(c.clone());
                    wo.push_back(c);
                }*/
            }
        } 
    }
    // to avoid doublons
    /* 
    let mut wo_final = Vec::new();
    let mut wo_hash = HashSet::new();
    for c in wo {
        if !wo_hash.contains(&c){
            let _ = wo_hash.insert(c.clone());
            wo_final.push(c);
        }
    }
    */
    for c in wo { //_final
        print!("{}", c);
    }
}


fn main() {
    let mut equalities : VecDeque<Equality> = VecDeque::new(); // get the equalities
    //EXAMPLE 1 : OK
    /* 
    let lhs0 : Term = Term::G(Box::new(Term::Var(0)));
    let rhs0 : Term = Term::H(Box::new(Term::Var(0)));
    let eq0 : Equality = Equality {lhs: lhs0, rhs: rhs0};
    let lhs1 : Term = Term::H(Box::new(Term::H(Box::new(Term::Var(1)))));
    let rhs1 : Term = Term::F(Box::new(Term::H(Box::new(Term::Var(1)))));
    let eq1 = Equality {lhs: lhs1, rhs: rhs1};
    equalities.push_back(eq0);
    equalities.push_back(eq1);
    */
    // EXAMPLE 2 : NOT OK because of overflow...
    
    let lhs0 = Term::G(Box::new(Term::Var(0)));
    let rhs0 = Term::A;
    let eq0 = Equality {lhs: lhs0, rhs: rhs0};
    let lhs1 = Term::H(Box::new(Term::Var(1)));
    let rhs1 = Term::A;
    let eq1 = Equality {lhs: lhs1, rhs: rhs1};
    let lhs2 = Term::G(Box::new(Term::H(Box::new(Term::Var(2)))));
    let rhs2 = Term::H(Box::new(Term::H(Box::new(Term::Var(2)))));
    let eq2 = Equality {lhs: lhs2, rhs: rhs2};

    equalities.push_back(eq0);
    equalities.push_back(eq1);
    equalities.push_back(eq2);
    
    //EXAMPLE 3 : OK
    /*let lhs0 = Term::F(Box::new(Term::Var(0)));
    let rhs0 = Term::G(Box::new(Term::Var(0)));
    let eq0 = Equality {lhs: lhs0, rhs:rhs0};
    equalities.push_back(eq0);
    */
    //EXAMPLE 4 : OK
    /* 
    let lhs0 = Term::F(Box::new(Term::A));
    let rhs0 = Term::H(Box::new(Term::A));
    let eq0 = Equality {lhs: lhs0, rhs: rhs0};
    let lhs1 = Term::G(Box::new(Term::A));
    let rhs1 = Term::H(Box::new(Term::A));
    let eq1 = Equality {lhs: lhs1, rhs: rhs1};
    let lhs2 = Term::F(Box::new(Term::B));
    let rhs2 = Term::H(Box::new(Term::B));
    let eq2 = Equality {lhs: lhs2, rhs: rhs2};
    let lhs3 = Term::G(Box::new(Term::B));
    let rhs3 = Term::H(Box::new(Term::B));
    let eq3 = Equality {lhs: lhs3, rhs:rhs3};
    let lhs4 = Term::F(Box::new(Term::Var(0)));
    let rhs4 = Term::G(Box::new(Term::Var(0)));
    let eq4 = Equality {lhs: lhs4, rhs:rhs4};
    let lhs5 = Term::A;
    let rhs5 = Term::F(Box::new(Term::A));
    let eq5 = Equality {lhs: lhs5, rhs: rhs5};
    let lhs6 = Term::B;
    let rhs6 = Term::F(Box::new(Term::B));
    let eq6 = Equality {lhs: lhs6, rhs: rhs6};
    equalities.push_back(eq0);
    equalities.push_back(eq1);
    equalities.push_back(eq2);
    equalities.push_back(eq3);
    equalities.push_back(eq4);
    equalities.push_back(eq5);
    equalities.push_back(eq6);
    */
    let mut m : Vec<Term> = Vec::new(); // get the ground terms

    //EXAMPLE 1
    /* 
    let ga = Term::G(Box::new(Term::A));
    let ha = Term::H(Box::new(Term::A));
    let fa = Term::F(Box::new(Term::A));
    let fha = Term::F(Box::new(ha.clone()));
    let gha = Term::G(Box::new(ha.clone()));
    let hha = Term::H(Box::new(ha.clone()));
    let fga = Term::F(Box::new(ga.clone()));
    let gga = Term::G(Box::new(ga.clone()));
    let hga = Term::H(Box::new(ga.clone()));

    let gb = Term::G(Box::new(Term::B));
    let hb = Term::H(Box::new(Term::B));
    let fb = Term::F(Box::new(Term::B));
    let fhb = Term::F(Box::new(hb.clone()));
    let ghb = Term::G(Box::new(hb.clone()));
    let hhb = Term::H(Box::new(hb.clone()));
    let fgb = Term::F(Box::new(gb.clone()));
    let ggb = Term::G(Box::new(gb.clone()));
    let hgb = Term::H(Box::new(gb.clone()));

    let gc = Term::G(Box::new(Term::C));
    let hc = Term::H(Box::new(Term::C));
    let fc = Term::F(Box::new(Term::C));
    let fhc = Term::F(Box::new(hc.clone()));
    let ghc = Term::G(Box::new(hc.clone()));
    let hhc = Term::H(Box::new(hc.clone()));
    let fgc = Term::F(Box::new(gc.clone()));
    let ggc = Term::G(Box::new(gc.clone()));
    let hgc = Term::H(Box::new(gc.clone()));
    
    m.push(ga);
    m.push(ha);
    m.push(fa);
    m.push(fha);
    m.push(gha);
    m.push(hha);
    m.push(fga);
    m.push(gga);
    m.push(hga);

    m.push(gb);
    m.push(hb);
    m.push(fb);
    m.push(fhb);
    m.push(ghb);
    m.push(hhb);
    m.push(fgb);
    m.push(ggb);
    m.push(hgb);

    m.push(gc);
    m.push(hc);
    m.push(fc);
    m.push(fhc);
    m.push(ghc);
    m.push(hhc);
    m.push(fgc);
    m.push(ggc);
    m.push(hgc);
    */
    // EXAMPLE 2
    
    let ga = Term::G(Box::new(Term::A));
    let ha = Term::H(Box::new(Term::A));
    let fa = Term::F(Box::new(Term::A));
    let fha = Term::F(Box::new(ha.clone()));
    let gha = Term::G(Box::new(ha.clone()));
    let hha = Term::H(Box::new(ha.clone()));
    let fga = Term::F(Box::new(ga.clone()));
    let gga = Term::G(Box::new(ga.clone()));
    let hga = Term::H(Box::new(ga.clone()));

    let gb = Term::G(Box::new(Term::B));
    let hb = Term::H(Box::new(Term::B));
    let fb = Term::F(Box::new(Term::B));
    let fhb = Term::F(Box::new(hb.clone()));
    let ghb = Term::G(Box::new(hb.clone()));
    let hhb = Term::H(Box::new(hb.clone()));
    let fgb = Term::F(Box::new(gb.clone()));
    let ggb = Term::G(Box::new(gb.clone()));
    let hgb = Term::H(Box::new(gb.clone()));

    let gc = Term::G(Box::new(Term::C));
    let hc = Term::H(Box::new(Term::C));
    let fc = Term::F(Box::new(Term::C));
    let fhc = Term::F(Box::new(hc.clone()));
    let ghc = Term::G(Box::new(hc.clone()));
    let hhc = Term::H(Box::new(hc.clone()));
    let fgc = Term::F(Box::new(gc.clone()));
    let ggc = Term::G(Box::new(gc.clone()));
    let hgc = Term::H(Box::new(gc.clone()));

    let ffa = Term::F(Box::new(fa.clone()));
    let gfa = Term::G(Box::new(fa.clone()));
    let hfa = Term::H(Box::new(fa.clone()));
    let ffb = Term::F(Box::new(fb.clone()));
    let gfb = Term::G(Box::new(fb.clone()));
    let hfb = Term::H(Box::new(fb.clone()));
    let ffc = Term::F(Box::new(fc.clone()));
    let gfc = Term::G(Box::new(fc.clone()));
    let hfc = Term::H(Box::new(fc.clone()));

    let a = Term::A;
    let b = Term::B;
    let c = Term::C;

    m.push(ga);
    m.push(ha);
    m.push(fa);
    m.push(fha);
    m.push(gha);
    m.push(hha);
    m.push(fga);
    m.push(gga);
    m.push(hga);

    m.push(gb);
    m.push(hb);
    m.push(fb);
    m.push(fhb);
    m.push(ghb);
    m.push(hhb);
    m.push(fgb);
    m.push(ggb);
    m.push(hgb);

    m.push(gc);
    m.push(hc);
    m.push(fc);
    m.push(fhc);
    m.push(ghc);
    m.push(hhc);
    m.push(fgc);
    m.push(ggc);
    m.push(hgc);

    m.push(ffa);
    m.push(gfa);
    m.push(hfa);
    m.push(ffb);
    m.push(gfb);
    m.push(hfb);
    m.push(ffc);
    m.push(gfc);
    m.push(hfc);
    
    m.push(a);
    m.push(b);
    m.push(c);
    
    //EXAMPLE 3
    /* 
    let ga = Term::G(Box::new(Term::A));
    let ha = Term::H(Box::new(Term::A));
    let fa = Term::F(Box::new(Term::A));
    let fha = Term::F(Box::new(ha.clone()));
    let gha = Term::G(Box::new(ha.clone()));
    let hha = Term::H(Box::new(ha.clone()));
    let fga = Term::F(Box::new(ga.clone()));
    let gga = Term::G(Box::new(ga.clone()));
    let hga = Term::H(Box::new(ga.clone()));

    let gb = Term::G(Box::new(Term::B));
    let hb = Term::H(Box::new(Term::B));
    let fb = Term::F(Box::new(Term::B));
    let fhb = Term::F(Box::new(hb.clone()));
    let ghb = Term::G(Box::new(hb.clone()));
    let hhb = Term::H(Box::new(hb.clone()));
    let fgb = Term::F(Box::new(gb.clone()));
    let ggb = Term::G(Box::new(gb.clone()));
    let hgb = Term::H(Box::new(gb.clone()));

    let gc = Term::G(Box::new(Term::C));
    let hc = Term::H(Box::new(Term::C));
    let fc = Term::F(Box::new(Term::C));
    let fhc = Term::F(Box::new(hc.clone()));
    let ghc = Term::G(Box::new(hc.clone()));
    let hhc = Term::H(Box::new(hc.clone()));
    let fgc = Term::F(Box::new(gc.clone()));
    let ggc = Term::G(Box::new(gc.clone()));
    let hgc = Term::H(Box::new(gc.clone()));

    let ffa = Term::F(Box::new(fa.clone()));
    let gfa = Term::G(Box::new(fa.clone()));
    let hfa = Term::H(Box::new(fa.clone()));
    let ffb = Term::F(Box::new(fb.clone()));
    let gfb = Term::G(Box::new(fb.clone()));
    let hfb = Term::H(Box::new(fb.clone()));
    let ffc = Term::F(Box::new(fc.clone()));
    let gfc = Term::G(Box::new(fc.clone()));
    let hfc = Term::H(Box::new(fc.clone()));

    m.push(ga);
    m.push(ha);
    m.push(fa);
    m.push(fha);
    m.push(gha);
    m.push(hha);
    m.push(fga);
    m.push(gga);
    m.push(hga);

    m.push(gb);
    m.push(hb);
    m.push(fb);
    m.push(fhb);
    m.push(ghb);
    m.push(hhb);
    m.push(fgb);
    m.push(ggb);
    m.push(hgb);

    m.push(gc);
    m.push(hc);
    m.push(fc);
    m.push(fhc);
    m.push(ghc);
    m.push(hhc);
    m.push(fgc);
    m.push(ggc);
    m.push(hgc);

    m.push(ffa);
    m.push(gfa);
    m.push(hfa);
    m.push(ffb);
    m.push(gfb);
    m.push(hfb);
    m.push(ffc);
    m.push(gfc);
    m.push(hfc);

    m.push(Term::A);
    m.push(Term::B);
    m.push(Term::C);
    */
    //EXAMPLE 4
    /* 
    let a = Term::A;
    let b = Term::B;
    let fa = Term::F(Box::new(a.clone()));
    let fb = Term::F(Box::new(b.clone()));
    let ha = Term::H(Box::new(a.clone()));
    let hb = Term::H(Box::new(b.clone()));
    let ga = Term::G(Box::new(a.clone()));
    let gb = Term::G(Box::new(b.clone()));

    m.push(a);
    m.push(b);
    m.push(fa);
    m.push(fb);
    m.push(ha);
    m.push(hb);
    m.push(ga);
    m.push(gb);
    */
    nongroundcc(equalities, &m);
}
