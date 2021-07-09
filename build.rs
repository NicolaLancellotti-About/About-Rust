use gcc;

fn main() {
    // 1 - Add a search path for compiled library
    println!("cargo:rustc-link-search=./src/clib");

    // 2 - Compile a library
    gcc::Build::new()
        .file("src/clib/clib.c")
        .include("src")
        .debug(true)
        .compile("libclib.a");
}
