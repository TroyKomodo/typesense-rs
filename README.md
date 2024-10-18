# Typesense Rust Library

![Crates.io Version](https://img.shields.io/crates/v/typesense-rs)
![docs.rs](https://img.shields.io/docsrs/typesense-rs)

You can generate the bindings using the generate script

```bash
./generate.sh
```

Currently we are using a custom patch of the openapi-generator which adds support for `reqwest-trait` and we are then ontop of that using my own patch to allow for single arguments and `bon` builders.

The docker image is published to `ghcr.io/troykomodo/openapi-generator:rust-gen`

The following PRs:

- https://github.com/ranger-ross/openapi-generator/pull/1
- https://github.com/OpenAPITools/openapi-generator/pull/19788
