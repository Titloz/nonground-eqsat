use crate::class::Class;
use crate::language::Term;

pub(crate) fn sat(c: &Class) -> bool {
    todo!("sat solver for c.constraints")
}

type LA = u8; // to change obviously

pub(crate) fn build_lAC(v: &Vec<Term>) -> LA {
    todo!("buildLAC")
}

pub(crate) fn la_implication_test(lac0 : LA, lac1 : LA) -> bool {
    todo!("laImplicationTest")
}