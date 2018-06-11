LIBCLANG_PATH=/usr/lib64/llvm/5/lib64/ cargo build -v
cargo fmt -- --force bindgen.rs
rm bindgen.rs.bk
