use todo_by::todo_by;

#[test]
#[allow(dead_code, unused_variables)]
fn test_todo_by_future_date() {
    todo_by!("2030-01-01");
    fn my_function() {
        // This function should compile because the date is in the future
    }

    struct User {}
    impl User {
        fn new(_name: &str) -> Self {
            Self {}
        }
    }

    todo_by!("2041-02-14");
    let user = User::new("Jane");
}

// #[test]
// #[should_panic(expected = "TODO by 2022-01-01 has already passed")]
// fn test_todo_by_past_date() {
// TODO: use the trybuild crate?
// let _ = try_compile_error!(todo_by!("2022-01-01"));

// todo_by!("2022-01-01");
// fn my_function() {
//     // This function should fail to compile because the date is in the past
// }
// }
