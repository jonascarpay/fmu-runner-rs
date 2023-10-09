use std::path::PathBuf;

fn main() {
    cc::Build::new().file("src/logger.c").compile("logger");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("fmi-standard/headers/fmi2Functions.h")
        .header("fmi-standard/headers/fmi2FunctionTypes.h")
        .header("fmi-standard/headers/fmi2TypesPlatform.h")
        .allowlist_var(r#"(\w*fmi\w*)"#)
        .allowlist_type(r#"(\w*fmi\w*)"#)
        .allowlist_function(r#"(\w*fmi\w*)"#)
        .disable_name_namespacing()
        .disable_nested_struct_naming()
        .translate_enum_integer_types(true)
        .wrap_unsafe_ops(true)
        .merge_extern_blocks(true)
        .rustified_enum("fmi2Status")
        .rustified_enum("fmi2StatusKind")
        .rustified_enum("fmi2Type")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("fmi.rs"))
        .expect("Couldn't write bindings!");
}
