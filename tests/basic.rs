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

// #[test]
// #[should_panic(expected = "Not a valid TODO date")]
// fn past_date_errors() {
//     todo_by!("2022/01/01");
// }

// #[test]
// #[should_panic(expected = "TODO by Jan 1, 2022 has passed")]
// fn past_date_errors() {
//     // TODO: use the trybuild crate?
//     // let _ = try_compile_error!(todo_by!("2022-01-01"));

//     todo_by!("2022-01-01");
//     fn my_function() {}
// }
