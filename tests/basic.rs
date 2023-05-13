use todo_by::todo_by;

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
#[allow(unused_variables)]
fn can_add_comments() {
    todo_by!("2030-11-11", "Here is my TODO comment!");
    let result = 1 + 1;
}

#[test]
fn failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/*.rs");
}
