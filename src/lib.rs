#![no_std]

mod sppcs;
mod win32;

use core::{ffi::c_void, ptr::null_mut};
use win32::{
    Guid, LocalFree, SLGetLicensingStatusInformation, SLGetProductSkuInformation, StrStrNIW,
    SL_LICENSING_STATUS,
};

const GRACE: [u16; 6] = [0x47, 0x72, 0x61, 0x63, 0x65, 0x00];
const NAME: [u16; 5] = [0x4e, 0x61, 0x6d, 0x65, 0x00];

unsafe fn is_grace_period_product(hslc: *const c_void, pproductskuid: *const Guid) -> bool {
    let mut p_buffer = null_mut();
    let mut cb_size = 0;

    if SLGetProductSkuInformation(
        hslc,
        pproductskuid,
        NAME.as_ptr(),
        null_mut(),
        &mut cb_size,
        &mut p_buffer,
    ) != 0
    {
        LocalFree(p_buffer.cast());
        return false;
    }

    if !StrStrNIW(p_buffer.cast(), GRACE.as_ptr(), cb_size).is_null() {
        LocalFree(p_buffer.cast());
        return true;
    }

    LocalFree(p_buffer.cast());
    false
}

#[no_mangle]
unsafe extern "system" fn SLGetLicensingStatusInformationHook(
    hslc: *const c_void,
    pappid: *const Guid,
    pproductskuid: *const Guid,
    pwszrightname: *const u16,
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
