// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    &1 as dyn Send;
//  ^^^^^^^^^^^^^^ERR cast to unsized type
//        ^^^^^^^^HELP(<1.89.0-beta) try casting to
//        |HELP(>=1.89.0-beta) consider casting to
//        ^^^^^^^^HELP(<1.89.0-beta) /Accept Replacement:.*&dyn Send/
//        |HELP(>=1.89.0-beta) /Accept Replacement:.*&/
    Box::new(1) as dyn Send;
//  ^^^^^^^^^^^^^^^^^^^^^^^ERR cast to unsized type
//                 ^^^^^^^^HELP(<1.46.0-beta) try casting to a `Box` instead
//                 ^^^^^^^^HELP(>=1.46.0-beta,<1.89.0-beta) you can cast to a `Box` instead
//                 |HELP(>=1.89.0-beta) you can cast to a `Box` instead
//                         |HELP(>=1.89.0-beta) you can cast to a `Box` instead
//                 ^^^^^^^^HELP(<1.89.0-beta) /Accept Replacement:.*Box<dyn Send>/
//                 |HELP(>=1.89.0-beta) /Accept Replacement:.*Box</
//                         |HELP(>=1.89.0-beta) /Accept Replacement:.*>/
}
