fn main() {
    println!("cargo:rustc-link-search=./lib");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=Hypervisor");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=/opt/homebrew/lib");
    #[cfg(all(not(feature = "tee"), not(feature = "efi"), not(feature = "mewz")))]
    println!("cargo:rustc-link-lib=krunfw");
    #[cfg(feature = "tee")]
    println!("cargo:rustc-link-lib=krunfw-sev");
    #[cfg(feature = "mewz")]
    println!("cargo:rustc-link-lib=krunfw-mewz");
}
