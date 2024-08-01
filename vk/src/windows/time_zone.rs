use win32::{GetTimeZoneInformation, TIME_ZONE_ID_INVALID, TIME_ZONE_INFORMATION};

/// Gets the current system time zone, represented as minutes away from UTC
pub fn get_time_zone() -> i16 {
    let mut time_zone_info = TIME_ZONE_INFORMATION::default();
    if (unsafe { GetTimeZoneInformation(&mut time_zone_info) } == TIME_ZONE_ID_INVALID) {
        return 0;
    }

    time_zone_info.bias as i16
}
