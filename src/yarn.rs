extern crate notion_core;

use notion_core::tool::{Yarn, Tool};

/// The entry point for the `yarn` shim.
pub fn main() {
    Yarn::launch()
}