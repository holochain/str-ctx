<!-- cargo-rdme start -->

Simple string context map.

```rust
let ctx = StrCtx::from_iter([("hello", "zombies")]);
assert_eq!("[hello:zombies]", &format!("{ctx}"));

let ctx = ctx.derive([("friend", "apple")]);
assert_eq!("[friend:apple,hello:zombies]", &format!("{ctx}"));

let ctx = ctx.derive([("hello", "world")]);
assert_eq!("[friend:apple,hello:world]", &format!("{ctx}"));

let ctx: Vec<(&'static str, String)> = ctx.into_iter().collect();
assert_eq!(
    &[("friend", "apple".into()), ("hello", "world".into())],
    ctx.as_slice(),
);
```

<!-- cargo-rdme end -->
