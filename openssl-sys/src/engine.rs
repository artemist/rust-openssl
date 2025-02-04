use libc::*;
use *;

extern "C" {
    pub fn ENGINE_load_builtin_engines();
    pub fn ENGINE_by_id(id: *const c_char) -> *mut ENGINE;

    pub fn ENGINE_init(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_finish(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_free(e: *mut ENGINE) -> c_int;

    pub fn ENGINE_ctrl_cmd(
        e: *mut ENGINE,
        cmd_name: *const c_char,
        i: c_long,
        p: *mut c_void,
        f: extern "C" fn() -> (),
        cmd_optional: c_int,
    ) -> c_int;
    pub fn ENGINE_ctrl_cmd_string(
        e: *mut ENGINE,
        cmd_name: *const c_char,
        arg: *const c_char,
        cmd_optional: c_int,
    ) -> c_int;

    pub fn ENGINE_load_private_key(
        e: *mut ENGINE,
        key_id: *const c_char,
        ui_method: *mut UI_METHOD,
        callback_data: *mut c_void,
    ) -> *mut EVP_PKEY;
    pub fn ENGINE_load_public_key(
        e: *mut ENGINE,
        key_id: *const c_char,
        ui_method: *mut UI_METHOD,
        callback_data: *mut c_void,
    ) -> *mut EVP_PKEY;
    pub fn ENGINE_load_ssl_client_cert(
        e: *mut ENGINE,
        ssl: *mut SSL,
        ca_dn: *mut stack_st_X509_NAME,
        pcert: *mut *mut X509,
        ppkey: *mut *mut EVP_PKEY,
        pother: *mut *mut stack_st_X509,
        ui_method: *mut UI_METHOD,
        callback_data: *mut c_void,
    ) -> c_int;

    pub fn UI_set_default_method(meth: *const UI_METHOD);
    pub fn UI_get_default_method() -> *const UI_METHOD;
    pub fn UI_get_method(ui: *mut UI) -> *const UI_METHOD;
    pub fn UI_set_method(ui: *mut UI, meth: *const UI_METHOD) -> *const UI_METHOD;

    pub fn UI_OpenSSL() -> *mut UI_METHOD;
    pub fn UI_null() -> *const UI_METHOD;
}

type UI = c_int;
type UI_STRING = c_int;

pub enum UI_METHOD {}

const UI_FLAG_REDOABLE: c_int = 0x0001;
const UI_FLAG_PRINT_ERRORS: c_int = 0x0100;

#[repr(C)]
pub enum UI_string_types {
    UIT_NONE = 0,
    UIT_PROMPT,
    UIT_VERIFY,
    UIT_BOOLEAN,
    UIT_INFO,
    UIT_ERROR,
}
