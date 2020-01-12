extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let dest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("./src/gl.rs")).unwrap();

    Registry::new(Api::Gles2, (3, 2), Profile::Compatibility, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}
