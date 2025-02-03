fn main() {
    println!("cargo:rustc-link-search=dll");
    println!("cargo:rustc-link-lib=static=sqlite3");
}
