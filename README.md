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

The `todo_while` macro allows you to ensure todos are done based on a semver requirement of what is in your Cargo.toml:

```rs
use todo_by::todo_while;

todo_while!("<1.0.0", "This has to be in the first major release")
```

This also allows you to make blockers:

```rs
todo_while!(">123.4", "Must fix this or bump the version")
```

### Important note for library authors

Currently, if you publish/offer a crate and a todo_by expires in the lib code, then it would be an unfixable compilation error for anyone importing the crate. This problem is being investigated - please offer solutions if you have any!

### You might find useful

Here are some alternative implementations for other languages and tools. These are not directly affiliated and software should always be vetted for trustworthiness, including this Rust `todo_by` crate! That said, great work by these authors.

- [no-expired-todo-comments](https://github.com/maxprilutskiy/eslint-plugin-no-expired-todo-comments) - ESLint plugin by [@MaxPrilutskiy](https://twitter.com/MaxPrilutskiy)
- [staabm/phpstan-todo-by](https://github.com/staabm/phpstan-todo-by) - PHPStan plugin by [@staabm](https://twitter.com/markusstaab)
- [barnumbirr/todo_by](https://github.com/barnumbirr/todo_by) - `todo_by` in Python by [@barnumbirr](https://twitter.com/barnumbirr)
