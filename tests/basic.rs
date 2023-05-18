use todo_by::{todo_by, todo_while};

#[test]
#[allow(dead_code, unused_variables)]
fn future_date_succeeds() {
    todo_by!("2030-01-01");
    fn my_function() {}

    struct User {}
    impl User {
        fn new(name: &str) -> Self {
            Self {}
        }
    }

    todo_by!("2041-02-14");
    let user = User::new("Jane");
}

#[test]
#[allow(dead_code, unused_variables)]
fn future_version_succeeds() {
    todo_while!("<8.1.0");
    fn my_function() {}

    struct User {}
    impl User {
        fn new(name: &str) -> Self {
            Self {}
        }
    }

    todo_while!("<11.1.0");
    let user = User::new("Jane");
}

#[test]
#[allow(unused_variables)]
fn can_add_version_comments() {
    todo_while!("<10.0.0", "Here is my TODO comment!");
    let result = 1 + 1;
}

#[test]
fn todo_by_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/todo_by/*.rs");
}

#[test]
fn todo_while_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/todo_while/invalid_version.rs");

    // Here we test them individually so we can set a stub version for each.

    std::env::set_var("TODO_WHILE_STUB", "2.0.0");
    t.compile_fail("tests/failures/todo_while/before_version.rs");

    std::env::set_var("TODO_WHILE_STUB", "0.3.0");
    t.compile_fail("tests/failures/todo_while/past_version.rs");
}
