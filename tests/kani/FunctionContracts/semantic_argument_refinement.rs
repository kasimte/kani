// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

// Multi-candidate refinement compares the written generic arguments against
// each impl's self type semantically (region-erased types) instead of
// comparing `def_path_str` renderings. Each impl pair below has distinct
// postconditions, so a harness only verifies if resolution picks its impl:
//   * `Nest<Box<dyn Any>>` — a trait object nested inside another argument;
//     its rendering keeps inner parens no string normalization reaches, so
//     this is the case the semantic comparison newly resolves (#4830),
//   * `Cg<'x'>` / `Ng<-7>` — char and negative-int const-generic arguments,
//   * `Rc`-style `Holder<Mu<T>, A>` — the impl's own parameter names written
//     in the turbofish match the generic impl (the std `Rc::assume_init`
//     shape),
//   * `Μtype<u8>` — a multibyte type name (no byte/char pitfalls semantically),
//   * `Al<u8>` / `Al<u16, u32>` — an omitted trailing defaulted argument and
//     that default written explicitly, both matched through the declared default,
//   * `Wc<u128::MAX>` / `Ni<i128::MIN>` — 128-bit const arguments at the
//     range-guard boundaries.
#![allow(uncommon_codepoints)]

use std::any::Any;
use std::boxed::Box;

struct Nest<T>(T);

impl Nest<Box<dyn Any>> {
    #[kani::ensures(|r| *r == 1)]
    fn tag(&self) -> u32 {
        1
    }
}

impl Nest<u32> {
    #[kani::ensures(|r| *r == 2)]
    fn tag(&self) -> u32 {
        2
    }
}

struct Cg<const C: char>;

impl Cg<'x'> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl Cg<'y'> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

struct Ng<const N: i8>;

impl Ng<-7> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl Ng<7> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

struct Mu<T>(core::marker::PhantomData<T>);
struct Holder<T, A>(core::marker::PhantomData<(T, A)>);

impl<T, A> Holder<Mu<T>, A> {
    #[kani::ensures(|r| *r == 1)]
    fn assume(&self) -> u32 {
        1
    }
}

impl<T, A> Holder<Box<T>, A> {
    #[kani::ensures(|r| *r == 2)]
    fn assume(&self) -> u32 {
        2
    }
}

#[allow(non_camel_case_types)]
struct Μtype<T>(T);

impl Μtype<u8> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl Μtype<u16> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

struct Al<T, A = u32>(core::marker::PhantomData<(T, A)>);

impl Al<u8> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl Al<u16> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

struct Wc<const N: u128>;

impl Wc<340282366920938463463374607431768211455> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl Wc<0> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

struct Ni<const N: i128>;

impl Ni<-170141183460469231731687303715884105728> {
    #[kani::ensures(|r| *r == 1)]
    fn get(&self) -> u32 {
        1
    }
}

impl Ni<0> {
    #[kani::ensures(|r| *r == 2)]
    fn get(&self) -> u32 {
        2
    }
}

mod verify {
    use super::*;

    #[kani::proof_for_contract(Nest::<Box<dyn Any>>::tag)]
    fn check_nested_dyn() {
        let n = Nest(Box::new(0u8) as Box<dyn Any>);
        let _ = n.tag();
    }

    #[kani::proof_for_contract(Nest::<u32>::tag)]
    fn check_nested_sibling() {
        let n = Nest(0u32);
        let _ = n.tag();
    }

    #[kani::proof_for_contract(Cg::<'x'>::get)]
    fn check_char_const() {
        let c = Cg::<'x'>;
        let _ = c.get();
    }

    #[kani::proof_for_contract(Ng::<-7>::get)]
    fn check_negative_const() {
        let n = Ng::<-7>;
        let _ = n.get();
    }

    #[kani::proof_for_contract(Holder::<Mu<T>, A>::assume)]
    fn check_param_name_spelling() {
        let h = Holder::<Mu<u8>, u8>(core::marker::PhantomData);
        let _ = h.assume();
    }

    #[kani::proof_for_contract(Μtype::<u8>::get)]
    fn check_multibyte_type() {
        let m = Μtype(0u8);
        let _ = m.get();
    }

    // Omitted trailing defaulted argument: `Al::<u8>` must match `impl Al<u8>`
    // (= `Al<u8, u32>`) through the declared default.
    #[kani::proof_for_contract(Al::<u8>::get)]
    fn check_omitted_default() {
        let a: Al<u8> = Al(core::marker::PhantomData);
        let _ = a.get();
    }

    // The default written explicitly must also match.
    #[kani::proof_for_contract(Al::<u16, u32>::get)]
    fn check_explicit_default() {
        let a: Al<u16> = Al(core::marker::PhantomData);
        let _ = a.get();
    }

    // u128::MAX as a literal const argument (widest unsigned cell; exercises the
    // unsigned range guard at its boundary).
    #[kani::proof_for_contract(Wc::<340282366920938463463374607431768211455>::get)]
    fn check_u128_max_const() {
        let w = Wc::<340282366920938463463374607431768211455>;
        let _ = w.get();
    }

    // i128::MIN as a literal (negated-magnitude == 2^127 boundary of the signed
    // range guard; the two's-complement wrap path).
    #[kani::proof_for_contract(Ni::<-170141183460469231731687303715884105728>::get)]
    fn check_i128_min_const() {
        let n = Ni::<-170141183460469231731687303715884105728>;
        let _ = n.get();
    }
}
