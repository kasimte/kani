// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

// A written type argument that neither resolves nor names one of the impl's
// generic parameters (here: a typo'd type name) refines nothing; the
// refinement reports the invalid arguments and lists the implementations.

struct W<T>(T);

impl W<u32> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl W<u64> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

#[kani::proof_for_contract(W::<u332>::get)]
fn check_typo_type() {
    let w = W(0u32);
    let _ = w.get();
}
