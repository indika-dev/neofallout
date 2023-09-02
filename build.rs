use cmake;

fn main() {
    use cmake::Config;

    let dst = Config::new("fallout2-ce").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=fallout2-ce");
}
