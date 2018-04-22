// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-pretty pretty-printing is unhygienic

#![feature(decl_macro, proc_macro_path_invoc)]

mod bar {
    mod baz {
        pub fn f() {}
    }

    pub macro m($f:ident) {
        baz::f();
        let _: i32 = $f();
        {
            fn $f() -> u32 { 0 }
            let _: u32 = $f();
        }
    }
}

fn main() {
    fn f() -> i32 { 0 }
    bar::m!(f);
}