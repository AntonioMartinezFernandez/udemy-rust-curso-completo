use std::collections::HashSet;

pub fn prelude() {
    /* PRELUDE
       https://doc.rust-lang.org/std/prelude/index.html
    */

    // defined structure
    let _my_struct: MyStruct;

    // imported structure
    let _imported: HashSet<i32>;

    // explicit structure
    let _explicit: std::collections::HashMap<i32, i32>;

    // with Prelude (a directive 'use' is imported implicitly --> std::prelude::v1::*)
    let _prelude: Option<i32>;
    // is equivalent to
    let _prelude2: std::prelude::v1::Option<i32>;
}

struct MyStruct {}
