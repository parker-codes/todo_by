use todo_by::{todo_by, todo_by_version};

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
    todo_by_version!(">8.1.0");
    fn my_function() {}

    struct User {}
    impl User {
        fn new(name: &str) -> Self {
            Self {}
        }
    }

    todo_by_version!(">11.1.0");
    let user = User::new("Jane");
}

#[test]
#[allow(unused_variables)]
fn can_add_version_comments() {
    todo_by_version!(">10.0.0", "Here is my TODO comment!");
    let result = 1 + 1;
}

#[test]
fn failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/*.rs");
}
