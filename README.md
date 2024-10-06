# lazy-template

## Description

Imagine that you need to interpolate strings, but the template variables are not quite cheap to load (such as system resources, network request, etc). And since the template are user-provided, you can't know ahead of time which variables you would need to load. You would need a flexible string template library that doesn't require calculating all these variables upfront.

Instead of requiring a complete set of inputs (such as via a `struct`, a `HashMap`, or a JSON object) to be available, the templates from this crate would send queries (which would usually be the names of the variables) to a function (called "responder") to get the value of each query.

While this library provides a preset parser for a preset template syntax (called `curly_braces`) that works out-of-the-box, it is also flexible enough to the advanced users to provide their own component parsers or even a whole new template parser.

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

The `query` in the example above is merely a variable name, but not necessarily. In more advanced use cases, `query` could be an expression, a command, a network request, or request for some system resource that would take time to load.

## License

[MIT][license] © [Hoàng Văn Khải][author]

<!-- LINKS -->
[docs]: https://docs.rs/lazy-template
[license]: https://github.com/KSXGitHub/lazy-template/blob/master/LICENSE.md
[author]: https://github.com/KSXGitHub/
