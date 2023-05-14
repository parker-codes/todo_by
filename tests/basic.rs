use todo_by::{todo_b4, todo_by};

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
    todo_b4!("<8.1.0");
    fn my_function() {}

    struct User {}
    impl User {
        fn new(name: &str) -> Self {
            Self {}
        }
    }

    todo_b4!("<11.1.0");
    let user = User::new("Jane");
}

#[test]
#[allow(unused_variables)]
fn can_add_b4_comments() {
    todo_b4!("<10.0.0", "Here is my TODO comment!");
    let result = 1 + 1;
}

#[test]
fn failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/*.rs");
}
