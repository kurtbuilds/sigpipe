<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/pipefix/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/pipefix.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/pipefix/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/pipefix.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/pipefix/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/pipefix/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/pipefix">
    <img src="https://img.shields.io/crates/d/pipefix?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/pipefix">
    <img src="https://img.shields.io/crates/v/pipefix?style=flat-square" alt="Crates.io" />
</a>

# pipefix

The default Rust runtime panics when `println!` and family write to a closed 
pipe. `pipefix` fixes it with a single function call invoked at the start of your program.

# Usage

```rust

fn main() {
    sigpipe::reset();
    // The rest of your program goes here.
}
```
  
# Installation

    # Using cargo-edit
    cargo add sigpipe

    # In your Cargo.toml
    [dependencies]
    sigpipe = "0.1"

# Discussion

There have been several discussions about this issue. See:

- https://github.com/rust-lang/rust/issues/24821
- https://github.com/rust-lang/rust/issues/46016
- https://github.com/rust-lang/rust/issues/62569

# Acknowledgments

This library is directly copied from @burntsushi's 
[StackOverflow answer](https://stackoverflow.com/questions/65755853/simple-word-count-rust-program-outputs-valid-stdout-but-panicks-when-piped-to-he/65760807#65760807).

I made this library to package the solution, so users don't have to hunt for it online.