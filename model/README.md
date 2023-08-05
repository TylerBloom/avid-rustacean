## Overview
This crate contains all common code shared between both ends of the app.
Since this app is a simple "Hello World", there is nothing in this model.
However, this is where you can fully leverage the power of having an app be fullstack in one language.

## API Definitions
Let's say we want a `GET` endpoint at `api/v1/foo` and the data that is returned should look something like:
```rust
pub const FOO_ROUTE: &str = "api/v1/foo";

pub struct Foo {
    name: String
    count: String
}
```

This allows us to tightly couple this route and type via a trait like so:
```rust
pub trait ApiDefinition: Serialize + Deserialize {
    const ROUTE: &'static str;
}

impl ApiDefinition for Foo {
    const ROUTE = FOO_ROUTE;
}
```

From here, the world is your oyster.

## Tips & Considerations
When working with WASM , you need to be mindful of your dependencies. Doubly so when you're sharing code between native and WASM targets. Many things will not compile to WASM or, worse, will silently panic if used incorrectly (or at all). It is wise to gate these problematic dependencies behind feature flags, such as `client` and `server`, and/or behind `cfg` directives. For example, imagine you need the `uuid` crate's `v4` feature (i.e. the "generate random ids" feature). Your `Cargo.toml` for the `model` might looks something like this:
```toml
[dependencies]
uuid = { version = "1.0", features = ["v4", "serde"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
# Needed for the randomness used by `uuid`
getrandom = { version = "0.2", features = ["js"] }
```

Major things to aware of are anything "random", anything dealing with the network or filesystem, and anything dealing with time.
