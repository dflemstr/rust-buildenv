// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_attrs)]

#[rustc_dump_program_clauses] //~ ERROR Implemented(Self: Foo<F>) :-
                              //~| ERROR FromEnv
                              //~| ERROR FromEnv
                              //~| ERROR FromEnv
trait Foo<F> where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8
{
    fn s(F) -> F;
}

fn main() {
    println!("hello");
}
