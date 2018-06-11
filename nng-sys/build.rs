extern crate pkg_config;
extern crate cmake;

fn main() {
    if let Ok(lib) = pkg_config::find_library("nng") {
        for path in &lib.include_paths {
            println!("cargo:include={}", path.display());
        }
        return
    }

    let mut cfg = cmake::Config::new("nng");
    let dst = cfg.define("BUILD_SHARED_LIBS", "OFF")
                 .define("CMAKE_INSTALL_LIBDIR", "lib")
                 .define("NNG_TESTS", "OFF")
                 .define("NNG_TOOLS", "OFF")
                 .define("NNG_ENABLE_TLS", "OFF")
                 .define("NNG_TRANSPORT_ZEROTIER", "OFF")
                 .define("NNG_ENABLE_HTTP", "OFF")
                 .define("NNG_TRANSPORT_WS", "OFF")
                 .build();

    println!("cargo:rustc-link-lib=static=nng");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:include={}/include", dst.display());
}
