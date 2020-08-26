use ::std::ffi::CStr;
use ::std::marker::PhantomData;


use ::r_sys::{DL_FUNC, R_CallMethodDef};

use crate::types::{RRes, RArg, RDll};

pub unsafe trait RawR {
    fn as_dlfunc(&self) -> DL_FUNC;

    fn arg_count(&self) -> i32;
}


macro_rules! rawr {
    ($type_name:ident, $arg_count:tt) => {
	unsafe impl RawR for $type_name {
	    fn as_dlfunc(&self) -> DL_FUNC {
		Some(unsafe{std::mem::transmute(*self)})
	    }

	    fn arg_count(&self) -> i32 {
		$arg_count
	    }
	}	
    };
}

pub type R0=extern "C" fn() -> RRes;
pub type R1=extern "C" fn(RArg) -> RRes;
pub type R2=extern "C" fn(RArg, RArg) -> RRes;
pub type R3=extern "C" fn(RArg, RArg, RArg) -> RRes;
pub type R4=extern "C" fn(RArg, RArg, RArg, RArg) -> RRes;
pub type R5=extern "C" fn(RArg, RArg, RArg, RArg, RArg) -> RRes;
pub type R6=extern "C" fn(RArg, RArg, RArg, RArg, RArg, RArg) -> RRes;

rawr!(R0, 0);
rawr!(R1, 1);
rawr!(R2, 2);
rawr!(R3, 3);
rawr!(R4, 4);
rawr!(R5, 5);
rawr!(R6, 6);


#[repr(transparent)]
pub struct RBinding<'a>(r_sys::R_CallMethodDef, PhantomData<&'a CStr>);

impl<'a> RBinding<'a> {
    pub fn new<T: RawR>(src: T, name:&'a CStr) -> Self {
	RBinding(R_CallMethodDef {
	    name: name.as_ptr(),
	    fun: src.as_dlfunc(),
	    numArgs: src.arg_count()
	}, PhantomData)
    }
  
    pub fn is_null(&self) -> bool {
	self.0.name.is_null() &&
	    self.0.fun.is_none() &&
	    (self.0.numArgs == 0)
    }

    pub fn null() -> Self {
	RBinding(R_CallMethodDef {
	    name: std::ptr::null(),
	    fun: None,
	    numArgs: 0},
	PhantomData)
    }
}

pub fn register(dll: RDll, linkage: &[RBinding]) {
    if !linkage.last().expect("No bindings supplied").is_null() {
	panic!("linkage marker not present");
    }
    
    unsafe {
	r_sys::R_registerRoutines(dll.to_ptr(),
				  std::ptr::null(),
				  linkage.as_ptr() as *const r_sys::R_CallMethodDef,
				  std::ptr::null(),
				  std::ptr::null());
	r_sys::R_useDynamicSymbols(dll.to_ptr(),
				   r_sys::Rboolean::TRUE);
	r_sys::R_forceSymbols(dll.to_ptr(),
			      r_sys::Rboolean::FALSE);
    }
}
