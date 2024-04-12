// Mostly copied from windows-sys

use core::ffi::c_void;

#[repr(C)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct SL_LICENSING_STATUS {
    pub SkuId: Guid,
    pub eStatus: i32,
    pub dwGraceTime: u32,
    pub dwTotalGraceDays: u32,
    pub hrReason: i32,
    pub qwValidityExpiration: u64,
}

#[cfg_attr(
    target_arch = "x86",
    link(name = "sppcs", kind = "raw-dylib", import_name_type = "undecorated")
)]
#[cfg_attr(not(target_arch = "x86"), link(name = "sppcs", kind = "raw-dylib"))]
extern "system" {
    pub fn SLGetLicensingStatusInformation(
        hslc: *const c_void,
        pappid: *const Guid,
        pproductskuid: *const Guid,
        pwszrightname: *const u16,
        pnstatuscount: *mut u32,
        pplicensingstatus: *mut *mut SL_LICENSING_STATUS,
    ) -> i32;
}

#[cfg_attr(
    target_arch = "x86",
    link(name = "sppcs", kind = "raw-dylib", import_name_type = "undecorated")
)]
#[cfg_attr(not(target_arch = "x86"), link(name = "sppcs", kind = "raw-dylib"))]
extern "system" {
    pub fn SLGetProductSkuInformation(
        hslc: *const c_void,
        pproductskuid: *const Guid,
        pwszvaluename: *const u16,
        pedatatype: *mut u32,
        pcbvalue: *mut u32,
        ppbvalue: *mut *mut u8,
    ) -> i32;
}

#[cfg_attr(
    target_arch = "x86",
    link(
        name = "KERNEL32",
        kind = "raw-dylib",
        import_name_type = "undecorated"
    )
)]
#[cfg_attr(not(target_arch = "x86"), link(name = "KERNEL32", kind = "raw-dylib"))]
extern "system" {
    pub fn LocalFree(hmem: *mut c_void) -> *mut c_void;
}

#[cfg_attr(
    target_arch = "x86",
    link(name = "SHLWAPI", kind = "raw-dylib", import_name_type = "undecorated")
)]
#[cfg_attr(not(target_arch = "x86"), link(name = "SHLWAPI", kind = "raw-dylib"))]
extern "system" {
    pub fn StrStrNIW(pszfirst: *const u16, pszsrch: *const u16, cchmax: u32) -> *mut u16;
}
