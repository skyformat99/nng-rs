export LIBCLANG_PATH=/usr/lib64/llvm/5/lib64/

bindgen all_in_one.h -o all_in_one.rs
rustfmt all_in_one.rs
