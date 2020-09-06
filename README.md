# Reproduce the error

Run `cargo test`, you should see following error:

```
error[E0308]: mismatched types
  --> client/src/lib.rs:19:3
   |
19 |           assert_eq!(
   |  _________^
20 | |             full_name(String::from("hello"), String::from("world")),
21 | |             test_utils::helper()
22 | |         )
   | |_________^ expected struct `Name`, found struct `client::Name`
```
