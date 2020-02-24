/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    let mut align_value = align;
    loop {
        if align_value < 2 {
            panic!("`align` is not a power of 2.");
        }
        else if align_value == 2 {
            break;
        }
        else {
            align_value /= 2;
        }
    }
    let reminder = addr % align;
    let aligned_addr = addr - reminder;
    return aligned_addr;
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2
/// or aligning up overflows the address.
pub fn align_up(addr: usize, align: usize) -> usize {
    let mut align_value = align;
    loop {
        if align_value < 2 {
            panic!("`align` is not a power of 2.");
        }
        else if align_value == 2 {
            break;
        }
        else {
            align_value /= 2;
        }
    }
    let reminder = addr % align;
    let mut aligned_addr = addr;
    if reminder > 0 {
        aligned_addr = addr + align - reminder;
    }
    if aligned_addr < addr {
        panic!("aligning up overflows the address.");
    }
    return aligned_addr;
}
