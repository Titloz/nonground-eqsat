use crate::class::Class;

pub(crate) fn check_subsumption(c0: Class, c1: Class) -> bool {
    if (&c0).sepvars().is_empty() {
        check_subsumption_fv(c0, c1) //, empty_subst
    } else {
        for t0 in c0.terms {
            for t1 in &c1.terms {
                todo!("I need substitutions as well as matching_terms functions");
            }
        }
        false
    }
}

fn check_subsumption_fv(c0: Class, c1: Class) -> bool { // sigma: &mut Subst
    for t1 in c1.terms {
        let result: bool = false;
        for t0 in &c0.terms {
            todo!("I need substitutions as well as matching_terms functions");
            todo!("I also need BuildLAC and LAImplicationTEST");
        }
        if !result {
            return false;
        }
    }
    return true;
}