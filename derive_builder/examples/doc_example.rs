// NOTE: generate fully expanded version with `cargo expand`.
//
//       cargo expand --example doc_example

#[macro_use]
extern crate derive_builder_fork_arti;
use derive_builder_fork_arti as derive_builder;

#[allow(dead_code)]
#[derive(Builder)]
struct Lorem {
    ipsum: u32,
}

fn main() {}
