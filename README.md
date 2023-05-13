# todo_by

Compile-time lifetimes for comments.

---

To use this macro, add it to your dependencies via Cargo:

```bash
cargo add todo_by
```

Then, import and invoke the macro and add a date (such as above a function or statement) like so:

```rs
use todo_by::todo_by;

todo_by!("2023-06-01");
fn my_function() {
    // TODO: Implement this function by June 1st, 2023
}
```

If the current date is after June 1st, 2023, the macro will generate a compile error with the message “TODO by Jun 1, 2023 has passed”. If the current date is on or before June 1st, 2023, the macro will expand to nothing and the code will compile normally.

You can also add specific TODO comments:

```rs
todo_by!("2023-06-01", "Clean up implementation");
```

### Important note for library authors

Currently, if you publish/offer a crate and a todo_by expires in the lib code, then it would be an unfixable compilation error for anyone importing the crate. This problem is being investigated - please offer solutions if you have any!

### TODOs

- [ ] Add docblocks.
