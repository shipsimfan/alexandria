use win32::{
    GetTimeZoneInformation, TIME_ZONE_ID_DAYLIGHT, TIME_ZONE_ID_INVALID, TIME_ZONE_INFORMATION,
};

/// Gets the current system time zone, represented as minutes away from UTC
pub fn get_time_zone() -> i16 {
    let mut time_zone_info = TIME_ZONE_INFORMATION::default();
    let result = unsafe { GetTimeZoneInformation(&mut time_zone_info) };
    if result == TIME_ZONE_ID_INVALID {
        return 0;
    }

    -(time_zone_info.bias
        + if result == TIME_ZONE_ID_DAYLIGHT {
            time_zone_info.daylight_bias
        } else {
            time_zone_info.standard_bias
        }) as i16
}
