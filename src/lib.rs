#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EverCrypt_Hash_state_s_s {
    _unused: [u8; 0],
}
pub type EverCrypt_Hash_state_s = EverCrypt_Hash_state_s_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EverCrypt_Hash_Incremental_hash_state_s {
    pub block_state: *mut EverCrypt_Hash_state_s,
    pub buf: *mut u8,
    pub total_len: u64,
}
pub type EverCrypt_Hash_Incremental_hash_state = EverCrypt_Hash_Incremental_hash_state_s;

pub type EverCrypt_Error_error_code = u8;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
extern "C" {
    pub fn EverCrypt_AutoConfig2_init();

    pub fn EverCrypt_Hash_Incremental_create_in(
        a: u8,
    ) -> *mut EverCrypt_Hash_Incremental_hash_state;

    pub fn EverCrypt_Hash_Incremental_update(
        s: *mut EverCrypt_Hash_Incremental_hash_state,
        data: *const u8,
        len: u32,
    ) -> EverCrypt_Error_error_code;

    pub fn EverCrypt_Hash_Incremental_finish(
        s: *mut EverCrypt_Hash_Incremental_hash_state,
        dst: *mut u8,
    );

    pub fn EverCrypt_Hash_Incremental_free(s: *mut EverCrypt_Hash_Incremental_hash_state);

    pub fn EverCrypt_Hash_Incremental_hash(a: u8, dst: *mut u8, input: *const u8, len: u32);
}

const SHA256_ALG: u8 = 1;
const ERROR_SUCCESS: u8 = 0;

pub struct Context {
    state: *mut EverCrypt_Hash_Incremental_hash_state,
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { EverCrypt_Hash_Incremental_free(self.state) }
    }
}

impl Context {
    pub fn new() -> Self {
        let state = unsafe { EverCrypt_Hash_Incremental_create_in(SHA256_ALG) };
        Self { state }
    }

    pub fn update_safe(&mut self, bytes: &[u8]) -> Result<(), EverCrypt_Error_error_code> {
        let result = unsafe {
            EverCrypt_Hash_Incremental_update(self.state, bytes.as_ptr(), bytes.len() as u32)
        };
        if result == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.update_safe(bytes).expect("incremental hashing failed")
    }

    pub fn finalize(self) -> [u8; 32] {
        let mut result = [0; 32];
        unsafe { EverCrypt_Hash_Incremental_finish(self.state, result.as_mut_ptr()) }
        drop(self);
        result
    }
}

pub fn init() {
    unsafe {
        EverCrypt_AutoConfig2_init();
    }
}

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut result = [0; 32];
    unsafe {
        EverCrypt_Hash_Incremental_hash(
            SHA256_ALG,
            result.as_mut_ptr(),
            bytes.as_ptr(),
            bytes.len() as u32,
        )
    }
    result
}
