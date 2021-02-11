extern crate bindgen;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use bindgen::Builder;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings = Builder::default()
        .header("lib/client.h")
        .header("lib/render.h")
        .header("lib/render_gl.h")
        .header("lib/stream_cb.h")
        .impl_debug(true)
        .opaque_type("mpv_handle")
        .opaque_type("mpv_render_context")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(&out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    if cfg!(windows) {
        let dll_src: String = String::from("./lib/x64/mpv-1.dll");
        let dll_dest_path = Path::new(&out_dir).join("mpv-1.dll");
        fs::copy(dll_src, dll_dest_path).unwrap();

        let lib_src: String = String::from("./lib/x64/mpv.lib");
        let lib_dest_path = Path::new(&out_dir).join("mpv.lib");
        fs::copy(lib_src, lib_dest_path).unwrap();

        println!("cargo:rustc-link-search={}", &out_dir);
    }

    println!("cargo:rustc-link-lib=mpv");
}
