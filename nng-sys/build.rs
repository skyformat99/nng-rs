extern crate cmake;

fn main() {
    let mut cfg = cmake::Config::new("nng");
    let dst = cfg.define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("NNG_TESTS", "OFF")
        .define("NNG_TOOLS", "OFF")
        .define("NNG_ENABLE_TLS", "OFF")
        .define("NNG_TRANSPORT_ZEROTIER", "OFF")
        .define("NNG_ENABLE_HTTP", "OFF")
        .define("NNG_TRANSPORT_WS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    println!("cargo:rustc-link-lib=static=nng");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:include={}/include", dst.display());
}
