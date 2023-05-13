# todo_by

Checks if a “todo by” date has passed _at compile time_.

To use this macro, you can add a date and a comment (such as above a function or statement) like so:

```rs
todo_by!("2023-06-01");
fn my_function() {
    // TODO: Implement this function by June 1st, 2023
}
```

If the current date is after June 1st, 2023, the macro will generate a compile error with the message “TODO by 2023-06-01 has already passed”. If the current date is on or before June 1st, 2023, the macro will do nothing and the code will compile normally.

### TODOs

- [ ] Make tests pass by asserting build failures (use trybuild?).
- [ ] Make message clearer by pointing to source code location, etc.
- [ ] Allow for a custom message in the macro definition, like `#[todo_by("2023-08-01", "Change this to async")]`
- [ ] Allow for dynamic dates (still at compile time)?
