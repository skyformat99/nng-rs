#![allow(bad_style)]

include!("c_headers/all_in_one.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
