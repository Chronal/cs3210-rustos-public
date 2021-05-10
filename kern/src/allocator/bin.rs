use core::alloc::Layout;
use core::fmt;
use core::ptr;

use crate::allocator::linked_list::LinkedList;
use crate::allocator::util::*;
use crate::allocator::LocalAlloc;

use crate::console::kprintln;

/// A simple allocator that allocates based on size classes.
///   bin 0 (2^3 bytes)    : handles allocations in (0, 2^3]
///   bin 1 (2^4 bytes)    : handles allocations in (2^3, 2^4]
///   ...
///   bin 29 (2^32 bytes): handles allocations in (2^31, 2^32]
///   
///   map_to_bin(size) -> k
///   

pub struct Allocator {
    bins: [ LinkedList; 30 ],
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {

        let mut bins: [LinkedList; 30] = [ LinkedList::new(); 30];

        let mut cur_addr = start;

        for expo in (3..25) {

            let block_size = 1 << expo;
            cur_addr = align_up(cur_addr, block_size);

            if (cur_addr + block_size > end) {
                break 
            }
            
            let bin_index = expo - 3;
            unsafe { bins[bin_index].push(cur_addr as *mut usize); }
            cur_addr += block_size;
        }

        let block_size = 1 << 24; //16MB
        let bin_index = 21; // 16 MB bin_index
        // cur_addr should be already aligned to 16MB
        while cur_addr + block_size < end {
            unsafe { bins[bin_index].push(cur_addr as *mut usize); }
            cur_addr += block_size;
        }

        /*
        for (index, bin) in bins.iter().enumerate() {
            let bin_size: usize = 1 << (index + 3);
            kprintln!("Bin size: {}", bin_size);
            for node in bin.iter() {
                kprintln!("{:?}", node);
            }
        }
        */

        Allocator {
            bins: bins,
        }
    }
}

impl LocalAlloc for Allocator {
    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning null pointer (`core::ptr::null_mut`)
    /// indicates that either memory is exhausted
    /// or `layout` does not meet this allocator's
    /// size or alignment constraints.
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let new_layout =  transform_layout(layout);
        let bin_index: usize = log2(new_layout.align()) - 3;

        match self.bins[bin_index].pop() {
            Some(addr) => return addr as *mut u8,
            _ => (),
        };

        for index in (bin_index + 1)..self.bins.len() {
            let node = self.bins[index].pop();
            
            if node.is_none() {
                continue;
            }

            let mut cur_addr = node.unwrap() as usize;

            /*
            let cur_block_size = 1 << index;

            for _ in 0..log2(cur_block_size) {
                unsafe { self.bins[bin_index].push(cur_addr as *mut usize); }
                cur_addr += new_layout.size();
            }
            */

            let mut cur_block_size = 1 << index;

            for move_index in ((bin_index + 1)..index).rev() {
                kprintln!("addr: {}, align {}", cur_addr, cur_block_size);
                cur_block_size = cur_block_size >> 1;
                unsafe { self.bins[move_index].push(cur_addr as *mut usize); }
                cur_addr += cur_block_size;
            }

            cur_block_size = cur_block_size >> 1;
            unsafe { self.bins[bin_index].push(cur_addr as *mut usize); }
            cur_addr += cur_block_size;

            return cur_addr as *mut u8;
        }
        
        return core::ptr::null_mut();
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let new_layout =  transform_layout(layout);
        let bin_index: usize = log2(new_layout.align()) - 3;

        unsafe { self.bins[bin_index].push(ptr as *mut usize); }
    }
}

fn transform_layout(layout: Layout) -> Layout {
    let adjusted_size = layout.size().next_power_of_two();
    if adjusted_size > layout.align() {
        Layout::from_size_align(adjusted_size, adjusted_size)
    } else {
        Layout::from_size_align(layout.align(),layout.align())
    }.unwrap()
}

fn log2(n: usize) -> usize {
    let mut cur_val = n;
    let mut log_val = 0;
    while cur_val > 1 {
        cur_val >>= 1;
        log_val+= 1;
    }

    return log_val;
}

// FIXME: Implement `Debug` for `Allocator`.
impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Allocator").finish()
    }
}
