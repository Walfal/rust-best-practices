#![allow(dead_code)]
use std::{cell::OnceCell, rc::Rc};

#[derive(Debug, Default)]
struct MyStruct {
    distance: usize,
    root: Option<Rc<OnceCell<MyStruct>>>,
}

fn main() {
    let root = MyStruct::default();
    let root_cell = Rc::new(OnceCell::new());
    if let Err(previous) = root_cell.set(root) {
        eprintln!("Previous Root {previous:?}");
    }
    let child_1 = MyStruct {
        distance: 1,
        root: Some(root_cell.clone()),
    };

    let child_2 = MyStruct {
        distance: 2,
        root: Some(root_cell),
    };

    println!("CHild 1: {child_1:?}");
    println!("CHild 2: {child_2:?}");
}

#[derive(Debug, PartialEq, Clone)]
struct User<'a> {
    name: &'a str,
    email: &'a str,
    updated_at: u32,
}

#[test]
fn should_create_nominal_user() {
    // ...
    // assert_eq!(user, expected_user);

    // or if some fields must not be tested (eg. updated_time)
    let user = User {
        name: "doe",
        email: "john@doe.com",
        updated_at: 1,
    };

    std::assert_matches!(user, User { name, email, .. } if name == "doe" && email == "john@doe.com");
}
