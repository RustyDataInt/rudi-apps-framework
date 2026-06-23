# MDI Apps Framework

The [Rusty Data Interface](https://rustydataint.github.io/) (RuDI) 
is a standardized framework for developing and running HPC data 
analysis **pipelines** and interactive visualization **apps**
with a Rust-first mindset.

This is the repository for the **RuDI apps framework**. It carries 
Rust Dioxus components and utilities that help quickly build modular 
RuDI apps.

The apps framework does not encode data analysis apps themselves, 
which are found in other tool suite repositories created from our 
suite repository template:

- tool suite template: <https://github.com/RustyDataInt/rudi-suite-template>

## Usage

Import the single crate in this repository into RuDI Dioxus app
and library crates as:

```toml
# Cargo.toml
[dependencies]
rudi_apps = { git = "https://github.com/RustyDataInt/rudi-apps-framework" }
```

```rust
// src/any.rs
use rudi::prelude::*;
```
