mod into_u32;
mod ip2region;
mod ipregion;

pub use self::ip2region::Ip2Region;
pub use self::ipregion::IpRegion;

use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::net::Ipv4Addr;
use std::ptr;

#[no_mangle]
pub extern fn ip2region_new(db_file:*const c_char) -> *mut Ip2Region{
    unsafe{
        match Ip2Region::new(CStr::from_ptr(db_file).to_str().unwrap()){
                    Ok(ip2region)=>{
                        Box::into_raw(Box::new(ip2region))
                    },
                    Err(_)=>ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern fn ip2region_free(ptr:*mut Ip2Region){
    if !ptr.is_null(){
        unsafe{
            Box::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern fn ip2region_get_region(ip2region:*mut Ip2Region,ip:*const c_char)->*mut c_char{
    if ip2region.is_null() || ip.is_null(){
       return ptr::null_mut();
    }
    unsafe{
        let ip = CStr::from_ptr(ip).to_string_lossy().into_owned();
        match ip.parse::<Ipv4Addr>(){
            Ok(ip)=>{
                let ip2region = &mut *ip2region;
                match ip2region.get_region(&ip){
                     Ok(result)=>{
                        let region = result.region();
                        CString::new(region).unwrap().into_raw()
                    },
                    Err(_)=>ptr::null_mut()
                }
            },
            Err(_)=>ptr::null_mut()
        }
    }    
}

#[no_mangle]
pub extern fn free_region(region:*mut c_char){
    if !region.is_null(){
        unsafe{
        CString::from_raw(region);
        }
    }
}