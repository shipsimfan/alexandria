use std::{arch::x86_64::__cpuid, borrow::Cow};

/// Get the model name of the CPU
pub fn cpu_model() -> String {
    // Get the highest extended result
    let highest_extended_result = unsafe { __cpuid(0x80000000) }.eax;

    // If doesn't support extended brand, fallback to legacy
    if highest_extended_result < 0x8000004 {
        let legacy_result = unsafe { __cpuid(0) };
        let ebx = legacy_result.ebx.to_le_bytes();
        let ecx = legacy_result.ecx.to_le_bytes();
        let edx = legacy_result.edx.to_le_bytes();
        let bytes = [
            ebx[0], ebx[1], ebx[2], ebx[3], edx[0], edx[1], edx[2], edx[3], ecx[0], ecx[1], ecx[2],
            ecx[3],
        ];
        let mut length = 0;
        for byte in bytes {
            if byte == 0 {
                break;
            }

            length += 1;
        }
        return String::from_utf8_lossy(&bytes[..length]).to_string();
    }

    // Pull extended brand name
    let brand_name1 = unsafe { __cpuid(0x80000002) };
    let eax1 = brand_name1.eax.to_le_bytes();
    let ebx1 = brand_name1.ebx.to_le_bytes();
    let ecx1 = brand_name1.ecx.to_le_bytes();
    let edx1 = brand_name1.edx.to_le_bytes();

    let brand_name2 = unsafe { __cpuid(0x80000003) };
    let eax2 = brand_name2.eax.to_le_bytes();
    let ebx2 = brand_name2.ebx.to_le_bytes();
    let ecx2 = brand_name2.ecx.to_le_bytes();
    let edx2 = brand_name2.edx.to_le_bytes();

    let brand_name3 = unsafe { __cpuid(0x80000004) };
    let eax3 = brand_name3.eax.to_le_bytes();
    let ebx3 = brand_name3.ebx.to_le_bytes();
    let ecx3 = brand_name3.ecx.to_le_bytes();
    let edx3 = brand_name3.edx.to_le_bytes();

    let bytes = [
        eax1[0], eax1[1], eax1[2], eax1[3], ebx1[0], ebx1[1], ebx1[2], ebx1[3], ecx1[0], ecx1[1],
        ecx1[2], ecx1[3], edx1[0], edx1[1], edx1[2], edx1[3], eax2[0], eax2[1], eax2[2], eax2[3],
        ebx2[0], ebx2[1], ebx2[2], ebx2[3], ecx2[0], ecx2[1], ecx2[2], ecx2[3], edx2[0], edx2[1],
        edx2[2], edx2[3], eax3[0], eax3[1], eax3[2], eax3[3], ebx3[0], ebx3[1], ebx3[2], ebx3[3],
        ecx3[0], ecx3[1], ecx3[2], ecx3[3], edx3[0], edx3[1], edx3[2], edx3[3],
    ];

    // Calculate the length of the model name
    let mut length = 0;
    for byte in bytes {
        if byte == 0 {
            break;
        }

        length += 1;
    }

    // Convert the model name to an owned string
    match String::from_utf8_lossy(&bytes[..length]) {
        Cow::Owned(owned) => owned,
        Cow::Borrowed(borrowed) => borrowed.to_owned(),
    }
}
