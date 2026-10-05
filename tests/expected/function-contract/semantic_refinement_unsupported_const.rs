// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

// A const argument that is not a literal (a path to a named constant) is not
// evaluated by the semantic comparison; the refinement reports the invalid
// arguments and lists the available implementations.

struct Cg<const N: usize>;

const LEN: usize = 3;

impl Cg<3> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl Cg<4> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

#[kani::proof_for_contract(Cg::<{ LEN }>::get)]
fn check_named_const() {
    let c = Cg::<3>;
    let _ = c.get();
}
