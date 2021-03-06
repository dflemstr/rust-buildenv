// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(dead_code)]

// Checks that mutable static items can have mutable slices


static mut TEST: &'static mut [isize] = &mut [1];
static mut EMPTY: &'static mut [isize] = &mut [];

pub fn main() {
    unsafe {
        TEST[0] += 1;
        assert_eq!(TEST[0], 2);
    }
}
