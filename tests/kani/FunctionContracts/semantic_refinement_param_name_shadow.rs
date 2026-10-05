// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

// A crate type whose name collides with an impl's generic parameter must NOT
// shadow the param-name wildcard. The written bare `T` in `Pair::<u8, T>::tag`
// is the generic impl's own parameter, not a resolution of the crate-local
// `struct T`. Before the param-name check was hoisted above type resolution,
// the fast path resolved the written `T` to `struct T`, compared it against the
// impl's parameter, mismatched, and wrongly rejected the generic impl.
use std::marker::PhantomData;

struct Pair<A, B>(PhantomData<(A, B)>);

#[allow(dead_code)]
struct T; // crate-local type sharing the generic parameter's name — the shadow

impl<T> Pair<u8, T> {
    #[kani::ensures(|r| *r == 1)]
    fn tag(&self) -> u32 {
        1
    }
}

// Non-overlapping sibling (distinct first argument) so refinement runs (len > 1).
impl Pair<u16, u16> {
    #[kani::ensures(|r| *r == 2)]
    fn tag(&self) -> u32 {
        2
    }
}

#[kani::proof_for_contract(Pair::<u8, T>::tag)]
fn check_param_name_not_shadowed() {
    let p: Pair<u8, u32> = Pair(PhantomData);
    let _ = p.tag();
}
