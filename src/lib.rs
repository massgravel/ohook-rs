#![no_std]

mod sppcs;
mod win32;

use core::{ffi::c_void, ptr::null_mut};
use win32::{LocalFree, StrStrNIW, SL_LICENSING_STATUS};
use win32::{SLGetLicensingStatusInformation, SLGetProductSkuInformation};

use windows_sys::{
    core::{GUID, PCWSTR},
    w,
};

unsafe fn is_grace_period_product(hslc: *const c_void, pproductskuid: *const GUID) -> bool {
    let mut p_buffer = null_mut();
    let mut cb_size = 0;

    if SLGetProductSkuInformation(
        hslc,
        pproductskuid,
        w!("Name"),
        null_mut(),
        &mut cb_size,
        &mut p_buffer,
    ) != 0
    {
        LocalFree(p_buffer as *mut c_void);
        return false;
    }

    if !StrStrNIW(p_buffer as *const u16, w!("Grace"), cb_size).is_null() {
        LocalFree(p_buffer as *mut c_void);
        return true;
    }

    LocalFree(p_buffer as *mut c_void);
    false
}

#[no_mangle]
unsafe extern "system" fn SLGetLicensingStatusInformationHook(
    hslc: *const c_void,
    pappid: *const GUID,
    pproductskuid: *const GUID,
    pwszrightname: PCWSTR,
    pnstatuscount: *mut u32,
    pplicensingstatus: *mut *mut SL_LICENSING_STATUS,
) -> i32 {
    let result = SLGetLicensingStatusInformation(
        hslc,
        pappid,
        pproductskuid,
        pwszrightname,
        pnstatuscount,
        pplicensingstatus,
    );
    if result != 0 {
        return result;
    }
    for i in 0..(*pnstatuscount as usize) {
        let status = (*pplicensingstatus).add(i);
        if (*status).eStatus == 0 {
            continue;
        }
        if is_grace_period_product(hslc, &(*status).SkuId) {
            continue;
        }
        (*status).eStatus = 1;
        (*status).dwGraceTime = 0;
        (*status).dwTotalGraceDays = 0;
        (*status).hrReason = 0;
        (*status).qwValidityExpiration = 0;
    }
    result
}

#[no_mangle]
extern "system" fn _DllMainCRTStartup(_: *const u8, _: u32, _: *const u8) -> u32 {
    1
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    panic!()
}
