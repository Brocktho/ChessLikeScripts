use godot::prelude::*;

mod general;
mod godot_rust_engine;
mod pieces;
struct Main;

#[gdextension]
unsafe impl ExtensionLibrary for Main {}
