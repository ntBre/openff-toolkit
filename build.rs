fn main() {
    let prefix = std::env::var("CONDA_PREFIX").unwrap();
    println!("cargo:rustc-env=LD_LIBRARY_PATH={prefix}/lib");
}
