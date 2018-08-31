// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]  // genus is always capitalized

pub(crate) struct Snail;
//~^ NOTE `Snail` declared as crate-visible

mod sea {
    pub(super) struct Turtle;
    //~^ NOTE `sea::Turtle` declared as restricted
}

struct Tortoise;
//~^ NOTE `Tortoise` declared as private

pub struct Shell<T> {
    pub(crate) creature: T,
}

pub type Helix_pomatia = Shell<Snail>;
//~^ ERROR crate-visible type `Snail` in public interface
//~| NOTE can't leak crate-visible type
pub type Dermochelys_coriacea = Shell<sea::Turtle>;
//~^ ERROR restricted type `sea::Turtle` in public interface
//~| NOTE can't leak restricted type
pub type Testudo_graeca = Shell<Tortoise>;
//~^ ERROR private type `Tortoise` in public interface
//~| NOTE can't leak private type

fn main() {}
