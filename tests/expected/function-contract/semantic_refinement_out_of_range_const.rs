// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

// A const literal outside the parameter type's range (256 for u8) names no impl
// value; the refinement must reject it (listing the implementations), never
// truncate it to a matching small value (256 as u8 would be 0).

struct Ob<const N: u8>;

impl Ob<0> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl Ob<1> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

#[kani::proof_for_contract(Ob::<256>::get)]
fn check_out_of_range() {
    let o = Ob::<0>;
    let _ = o.get();
}
