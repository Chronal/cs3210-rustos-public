/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    is_power_of_two(align);
    let shift_amount = addr % align;
    return addr - shift_amount;
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
    is_power_of_two(align);
    let shift_amount = align - (addr % align);
    if addr % align == 0 {
        addr
    } else {
        addr + shift_amount
    }
}


/// Checks `align` is a power of two
/// Uses technique listed here
/// https://graphics.stanford.edu/~seander/bithacks.html#DetermineIfPowerOf2
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
fn is_power_of_two(align: usize) -> bool {
    if align == 0 || !((align & (align - 1)) == 0) {
        panic!("Alignment {} not a power of 2", align); 
    }
    return true;
}
