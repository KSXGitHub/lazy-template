# lazy-template

## Description

This is a string template crate. Instead of requiring a complete set of inputs (such as via a `struct`, a `HashMap`, or a JSON object) to be available, the templates from this crate would send queries (which would usually be the names of the variables) to a function (called "responder") to get the value of each query.

## Documentation

Go to [docs.rs][docs].

## Basic Usage

This is the most basic usage. There are more in the [documentation][docs].

```rust
let system = lazy_template::simple_curly_braces();
let template = system.lazy_parse("{name} is a {age} years old {gender}");
let output = template
    .to_string(|query| match query {
        "name" => Ok("Alice"),
        "age" => Ok("20"),
        "gender" => Ok("girl"),
        _ => Err(format!("Can't answer {query}")),
    })
    .unwrap();
assert_eq!(output, "Alice is a 20 years old girl");
```

## License

[MIT][license] © [Hoàng Văn Khải][author]

<!-- LINKS -->
[docs]: https://docs.rs/lazy-template
[license]: https://github.com/KSXGitHub/lazy-template/blob/master/LICENSE.md
[author]: https://github.com/KSXGitHub/
