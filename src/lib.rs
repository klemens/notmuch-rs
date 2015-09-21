extern crate libc;

use std::ffi::{CStr, CString};
use std::{str, ptr};

mod ffi;
pub use ffi::Status;
pub use ffi::OpenMode;

#[derive(Debug)]
pub struct Database {
    database: ffi::database_t
}

impl Database {
    pub fn create(path: &str) -> Result<Database, Status> {
        let cstring = CString::new(path).unwrap();
        let mut database: ffi::database_t = ptr::null_mut();
        unsafe {
            match ffi::notmuch_database_create(cstring.as_ptr(), &mut database) {
                Status::Success => Ok(Database { database: database }),
                result => Err(result),
            }
        }
    }

    pub fn open(path: &str, mode: OpenMode) -> Result<Database, Status> {
        let cstring = CString::new(path).unwrap();
        let mut database: ffi::database_t = ptr::null_mut();
        unsafe {
            match ffi::notmuch_database_open(cstring.as_ptr(), mode, &mut database) {
                Status::Success => Ok(Database { database: database }),
                result => Err(result),
            }
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_database_destroy(self.database);
            self.database = ptr::null_mut();
        }
    }
}

pub fn explain_status(status: Status) -> String {
    unsafe {
        let ptr = ffi::notmuch_status_to_string(status);
        let bytes = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8(bytes).ok().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn status_descriptions() {
        assert_eq!(explain_status(Status::Success), "No error occurred");
        assert_eq!(explain_status(Status::OutOfMemory), "Out of memory");
        assert_eq!(explain_status(Status::FileError), "Something went wrong trying to read or write a file");
    }

    #[test]
    fn create_and_open_database() {
        let path = ".test-notmuch-rs";
        fs::create_dir(path).unwrap();
        {
            Database::create(path).unwrap();
        }
        {
            Database::open(path, OpenMode::ReadOnly).unwrap();
        }
        {
            Database::open(path, OpenMode::ReadWrite).unwrap();
        }
        fs::remove_dir_all(path).unwrap();
    }
}
