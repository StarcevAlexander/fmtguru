# fmtguru

**Runtime string templates for Rust** — `{name}` and `{0}` placeholders rendered from key-value pairs at runtime, with `{{` escaping. Complements the compile-time `format!` when the template arrives from a config file, database or user input.

Zero dependencies.

## Usage

```rust
use fmtguru::Template;

fn main() {
    let template = Template::new("Hello {name}, you have {count} messages");
    let out = template.render(&[
        ("name", "Ivan"),
        ("count", "7"),
    ]);
    assert_eq!(out, "Hello Ivan, you have 7 messages");

    let positional = Template::new("{0} vs {1}");
    assert_eq!(positional.render_positional(&["rock", "paper"]), "rock vs paper");
}
```

- `{{` and `}}` escape literal braces
- missing keys stay verbatim (`{unknown}` is left untouched)
- `keys()` lists every placeholder in order

## License

Licensed under either of MIT or Apache-2.0, at your option.

Name is a tribute to C++'s [fmt](https://github.com/fmtlib/fmt); this crate is an independent implementation providing runtime (non-macro) template rendering for Rust.
