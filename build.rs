fn main() {
    // println!("cargo:rustc-link-search=native=/home/chase/github/cmajor-rs/cmajor/x64");
    // println!("cargo:rustc-link-search=native=.");
    // println!("cargo:rustc-link-lib=dylib=CmajPerformer");
    println!(r"cargo:rustc-link-search=/home/chase/github/cmajor-rs");
    // println!("cargo:rustc-link-lib=CmajPerformer");
    // println!("cargo:rustc-link-arg=-lstdc++");
    // println!("cargo:rustc-link-search=native=/home/shep/rust/dynlink/library");
    // println!("cargo:rustc-link-arg=-Wl,-rpath,/home/chase/github/cmajor-rs/cmajor/x64/libCmajPerformer.so");
}
