use std::fmt::{Display, Formatter};
use riscv_codec_proc_macros::make_immediate;

// name signed compressed (imm_pos size instr pos)+
make_immediate!(IImmediate true false (0 12 20));
make_immediate!(SImmediate true false (0 5 7) (5 7 25));
make_immediate!(UImmediate true false (0 20 12));
make_immediate!(JImmediate true false (12 8 12) (11 1 20) (1 10 21) (20 1 31));
make_immediate!(BImmediate true false (11 1 7) (1 4 8) (5 6 25) (12 1 31));
make_immediate!(Shamt  false false (0 6 20));
make_immediate!(ShamtW false false (0 5 20));

make_immediate!(CWideImmediate false true (3 1 5) (2 1 6) (6 4 7) (4 2 11));
make_immediate!(CDImmediate false true (6 2 5) (3 3 10));
make_immediate!(CWImmediate false true (6 1 5) (2 1 6) (3 3 10));
make_immediate!(CIImmediate true true (0 5 2) (5 1 12));
make_immediate!(CBImmediate true true (5 1 2) (1 2 3) (6 2 5) (3 2 10) (8 1 12));
make_immediate!(CShamt false true (0 5 2) (5 1 12));
make_immediate!(CJImmediate true true (5 1 2) (1 3 3) (7 1 6) (6 1 7) (10 1 8) (8 2 9) (4 1 11) (11 1 12));
make_immediate!(CDSPImmediate false true (6 3 2) (3 2 5) (5 1 12));
make_immediate!(CWSPImmediate false true (6 2 2) (2 3 4) (5 1 12));
make_immediate!(CSDSPImmediate false true (6 3 7) (3 3 10));
make_immediate!(CSWSPImmediate false true (6 2 7) (2 4 9));
make_immediate!(C16SPImmediate true true (5 1 2) (7 2 3) (6 1 5) (4 1 6) (9 1 12));

make_immediate!(CSR false false (0 12 20));
make_immediate!(CSRImmediate false false (0 5 15));