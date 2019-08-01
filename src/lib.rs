use leveldb_sys as sys;
use std::ffi::CStr;

#[derive(Debug)]
pub struct Error<'a> {
    msg: &'a CStr,
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // Unwrap here is safe because LevelDB
        // shouldn't return error messages with
        // invalid UTF-8
        f.write_str(self.msg.to_str().unwrap())?;
        Ok(())
    }
}

impl<'a> std::error::Error for Error<'a> {}

#[derive(Debug)]
pub struct Database {
    handle: *mut sys::leveldb_t,
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe {
            sys::leveldb_close(self.handle);
        }
    }
}

/// A LevelDB database object is thread-safe.
unsafe impl Send for Database {}
/// A LevelDB database object is thread-safe.
unsafe impl Sync for Database {}

#[derive(Debug)]
pub struct Options {
    handle: *mut sys::leveldb_options_t,
}

impl Options {
    pub fn new() -> Self {
        let handle = unsafe { sys::leveldb_options_create() };
        Self { handle }
    }

    pub fn create_if_missing(self, create_if_missing: bool) -> Self {
        unsafe {
            sys::leveldb_options_set_create_if_missing(self.handle, create_if_missing as u8);
        }

        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Options {
    fn drop(&mut self) {
        unsafe {
            sys::leveldb_options_destroy(self.handle);
        }
    }
}
