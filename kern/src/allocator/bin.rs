use core::alloc::Layout;
use core::alloc::LayoutErr;

use core::fmt;
use core::ptr;

use crate::allocator::linked_list::LinkedList;
use crate::allocator::util::*;
use crate::allocator::LocalAlloc;


/// A simple allocator that allocates based on size classes.
///   bin 0 (2^3 bytes)    : handles allocations in (0, 2^3]
///   bin 1 (2^4 bytes)    : handles allocations in (2^3, 2^4]
///   ...
///   bin 29 (2^22 bytes): handles allocations in (2^31, 2^32]
///   
///   map_to_bin(size) -> k
///   

pub struct Allocator {
    bins: [LinkedList; 30],
    start: usize,
    end: usize,
    unallocated_addr: usize,
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        let bins = [LinkedList::new(); 30];

        let start = align_up(start, 1 << 3); 

        Allocator {
            bins: bins,
            start: start,
            end: end,
            unallocated_addr: start,
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
        let layout = transform_layout(layout);

        if layout.is_err() {
            return core::ptr::null_mut();
        }

        let layout = layout.unwrap();

        let bin_index = log2(layout.size()) - 3;
        let bin = &mut self.bins[bin_index];

        // memory already in bin
        if (bin.peek().is_some()) {
            return bin.pop().unwrap() as *mut u8;
        }

        /*
        // Get memory for larger bins
        'outer: for (index, bin) in self.bins[(bin_index + 1)..].iter_mut().enumerate() {
            if bin.peek().is_none() {
                continue
            }

            let index = index + (bin_index + 1);

            let required_bin_size = 1 << bin_index;
            let cur_bin_size = 1 << index;

            let mem_block = bin.pop().unwrap() as usize;

            let nsplit_blocks = cur_bin_size/required_bin_size;
            for n in (1..nsplit_blocks) {
                let new_block = (mem_block as usize) + n*required_bin_size;
                bin.push(new_block as *mut usize); 
            }

            return mem_block as *mut u8
        }
        */

        let alloc_addr = align_up(self.unallocated_addr, layout.align());

        if (alloc_addr + layout.size() > self.end) {
            return core::ptr::null_mut();
        }

        let prev_unalloc = self.unallocated_addr;
        self.unallocated_addr = alloc_addr + layout.size();

        //Put memory between prev_unalloc and the alloc_addr into the smallest bin
        let mut cur_addr = prev_unalloc;
        while (cur_addr + 8 < alloc_addr) {
            self.bins[0].push(cur_addr as *mut usize);
            cur_addr += 8;
        }

        return alloc_addr as *mut u8;
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
        if (ptr.is_null()) {
            return;
        }

        let layout = transform_layout(layout).unwrap();

        // Can use layout.size() or layout.align())
        let bin_index = log2(layout.size()) - 3;
        let mut bin = &mut self.bins[bin_index];

        bin.push(ptr as *mut usize);

    }
}

fn transform_layout(layout: Layout) -> Result<Layout, ()> {
    if (layout.size() == 0 || !layout.align().is_power_of_two()) {
        return Err(());
    }

    let mut adjusted_size = layout.size().next_power_of_two();
    let mut adjusted_align = layout.align(); 

    if (adjusted_size < 8) {
        adjusted_size = 8;
    }

    if (adjusted_align < 8) {
        adjusted_align = 8;
    }

    let new_layout = if adjusted_size > adjusted_align {
        Layout::from_size_align(adjusted_size, adjusted_size)
    } else {
        Layout::from_size_align(adjusted_align, adjusted_align)
    };

    return Ok(new_layout.unwrap())
}

// n has to be power of two
fn log2(n: usize) -> usize {
    n.trailing_zeros() as usize
}

fn previous_power_of_two(n: usize) -> usize {
    if (n.is_power_of_two()) {
        n
    } else {
        n.next_power_of_two() << 1
    }
}

// FIXME: Implement `Debug` for `Allocator`.
impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Bin Alloc")
            .field("bins", &self.bins)
            .field("start", &self.start)
            .field("unallocated_addr", &self.unallocated_addr)
            .field("end", &self.end)
            .finish()
    }
}
