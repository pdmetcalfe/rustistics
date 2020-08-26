use r_sys::{SEXP, DllInfo};

#[repr(transparent)]
pub struct RArg(SEXP);

#[repr(transparent)]
pub struct RRes(SEXP);

pub struct RVar(SEXP);


impl From<RArg> for RRes {
    fn from(x: RArg) -> Self {
	RRes(x.0)
    }
}

impl From<RVar> for RRes {
    fn from(x: RVar) -> Self {
	RRes(x.0)
    }
}

#[repr(transparent)]
pub struct RDll(*mut DllInfo);

impl RDll {
    pub(crate) unsafe fn to_ptr(&self) -> *mut DllInfo {
	self.0
    }
}


impl RVar {
    pub(crate) fn new(x: SEXP) -> Self {
	RVar(unsafe { r_sys::Rf_protect(x) })
    }
}

impl Drop for RVar {
    fn drop(&mut self) {
	unsafe { r_sys::Rf_unprotect(1) };
    }
}
