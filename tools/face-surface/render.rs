// Offline entrypoint: the production showcase owns every server-rendered byte.
#![allow(dead_code)]
include!("../../src/bands/shell/test.rs");

fn main() {
    print!("{}", render_component_showcase());
}
