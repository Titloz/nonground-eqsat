use std::collections::VecDeque;

use crate::class::Class;
use crate::subsumption::check_subsumption;
use crate::language::Term;

pub(crate) fn deduct_intern(wo: &mut VecDeque<Class>, us: &mut VecDeque<Class>, t0: Term, t1: Term, i: usize, n: usize, c0: &Class, c_new: Class, used: bool) -> bool {
    todo!()
}