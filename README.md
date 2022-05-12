# rtm-examples

Examples for [Rust Type Mapper (RTM)](https://rtm-rs.org).

![GitHub branch checks state](https://img.shields.io/github/checks-status/rtm-rs/rtm-examples/main)
![Crates.io (recent)](https://img.shields.io/crates/dr/rtm-examples)
![docs.rs](https://img.shields.io/docsrs/rtm-examples)
[![codecov](https://codecov.io/gh/rtm-rs/rtm-examples/branch/main/graph/badge.svg)](https://codecov.io/gh/rtm-rs/rtm-examples)
[![DeepSource](https://deepsource.io/gh/rtm-rs/rtm-examples.svg/?label=active+issues&show_trend=true&token=6nwFjH9D5hpzgH94vasqI0kI)](https://deepsource.io/gh/rtm-rs/rtm-examples/?ref=repository-badge)
[![DeepSource](https://deepsource.io/gh/rtm-rs/rtm-examples.svg/?label=resolved+issues&show_trend=true&token=6nwFjH9D5hpzgH94vasqI0kI)](https://deepsource.io/gh/rtm-rs/rtm-examples/?ref=repository-badge)

## Rust Type Mapper (RTM) Homepage

[rtm-rs.org](https://rtm-rs.org)

## Crate Documentation

[https://docs.rs/rtm-examples](https://docs.rs/rtm-examples)

## Idea: derive

For types implementing relations and aggregates (enties+values)

```rust
#[derive(Relation)] // Relations can exists outside the transitory layer - which is a type
#[derive(Relation, Transitory)]
#[derive(Aggregate)]  // i.e. all aggregates exist in the transitory layer - which is a type
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or [http://www\.apache\.org/licenses/LICENSE-2.0](http://www\.apache\.org/licenses/LICENSE-2.0))
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
