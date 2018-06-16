extern crate nng_sys as raw;

use std::os::raw::c_void;

mod aio;
mod conn_mgmt;
mod error;
mod msg_hdr;
mod proto;
mod socket;

pub struct NngBuf {
    buf: *mut c_void,
    size: usize,
}

impl Drop for NngBuf {
    fn drop(&mut self) {
        unsafe {
            raw::nng_free(self.buf, self.size);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
