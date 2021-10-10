use std::env::var;

fn main() {
    let target = var("TARGET").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    if target.contains("windows") && !target.contains("msvc") {
        println!("cargo:rerun-if-env-changed=WINAPI_NO_BUNDLED_LIBRARIES");
        println!("cargo:rerun-if-env-changed=WINAPI_STATIC_NOBUNDLE");

        println!("cargo:rustc-link-lib=static=gcc");
        println!("cargo:rustc-link-lib=static=stdc++");
        println!("cargo:rustc-link-lib=static=winpthread");
    }
}
