// RGB C bindings library (librgb)
// Written in 2019 by
//     Alekos Filini,
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::ffi::{CStr, CString};
use std::hash::{Hash, Hasher};
use std::os::raw::{c_char, c_void};

use rgb::i9n::Runtime;

use crate::error::RequestError;

pub(crate) fn string_to_ptr(other: String) -> *const c_char {
    let cstr = match CString::new(other) {
        Ok(cstr) => cstr,
        Err(_) => CString::new(String::from(
            "Error converting string: contains a null-char",
        ))
        .unwrap(),
    };

    cstr.into_raw()
}

pub(crate) fn ptr_to_string(
    ptr: *const c_char,
) -> Result<String, RequestError> {
    unsafe { Ok(CStr::from_ptr(ptr).to_string_lossy().into_owned()) }
}

pub(crate) trait CReturnType: Sized + 'static {
    fn from_opaque(other: &COpaqueStruct) -> Result<&mut Self, RequestError> {
        let mut hasher = DefaultHasher::new();
        TypeId::of::<Self>().hash(&mut hasher);
        let ty = hasher.finish();

        if other.ty != ty {
            return Err(RequestError::Runtime(
                rgb::error::BootstrapError::ArgParseError(s!("Type mismatch")),
            ));
        }

        let boxed = unsafe { Box::from_raw(other.ptr.clone() as *mut Self) };
        Ok(Box::leak(boxed))
    }
}
impl CReturnType for Runtime {}
impl CReturnType for String {}
impl CReturnType for () {}

#[repr(C)]
pub struct COpaqueStruct {
    ptr: *const c_void,
    ty: u64,
}

impl COpaqueStruct {
    fn new<T: 'static>(other: T) -> Self {
        let mut hasher = DefaultHasher::new();
        TypeId::of::<T>().hash(&mut hasher);
        let ty = hasher.finish();

        COpaqueStruct {
            ptr: Box::into_raw(Box::new(other)) as *const c_void,
            ty,
        }
    }

    fn raw<T>(ptr: *const T) -> Self {
        COpaqueStruct {
            ptr: ptr as *const c_void,
            ty: 0,
        }
    }
}

#[repr(C)]
pub enum CResultValue {
    Ok,
    Err,
}

#[repr(C)]
pub struct CResult {
    result: CResultValue,
    inner: COpaqueStruct,
}

impl<T: 'static, E> From<Result<T, E>> for CResult
where
    E: std::fmt::Debug,
{
    fn from(other: Result<T, E>) -> Self {
        match other {
            Ok(d) => CResult {
                result: CResultValue::Ok,
                inner: COpaqueStruct::new(d),
            },
            Err(e) => CResult {
                result: CResultValue::Err,
                inner: COpaqueStruct::raw(string_to_ptr(format!("{:?}", e))),
            },
        }
    }
}

#[repr(C)]
pub struct CResultString {
    result: CResultValue,
    inner: *const c_char,
}

impl From<Result<String, RequestError>> for CResultString
where
    RequestError: std::fmt::Debug,
{
    fn from(other: Result<String, RequestError>) -> Self {
        match other {
            Ok(d) => CResultString {
                result: CResultValue::Ok,
                inner: string_to_ptr(d),
            },
            Err(e) => CResultString {
                result: CResultValue::Err,
                inner: string_to_ptr(format!("{:?}", e)),
            },
        }
    }
}
