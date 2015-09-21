use libc::{c_char, c_void};

#[repr(C)]
#[derive(Debug)]
pub enum Status {
    Success = 0,
    OutOfMemory,
    ReadOnlyDatabase,
    XapianException,
    FileError,
    FileNotEmail,
    DuplicateMessageId,
    NullPointer,
    TagTooLong,
    UnbalancedFreezeThaw,
    UnbalancedAtomic,
    UnsupportedOperation,
    UpgradeRequired,
    PathError,
    LastStatus,
}

#[repr(C)]
#[derive(Debug)]
pub enum OpenMode {
    ReadOnly = 0,
    ReadWrite,
}

#[allow(non_camel_case_types)]
pub type database_t = *mut c_void;

#[link(name = "notmuch")]
extern {
    pub fn notmuch_database_create(path: *const c_char, database: *mut database_t) -> Status;
    pub fn notmuch_database_open(path: *const c_char, mode: OpenMode, database: *mut database_t) -> Status;
    pub fn notmuch_database_destroy(database: database_t) -> Status;

    pub fn notmuch_status_to_string(status: Status) -> *const c_char;
}
