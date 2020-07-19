# lab2

## Notes

### main
- `alloc::{boxed, ...}` is the same as `std::{boxed, ...}`
- Why it can alloc the data in the heap which we has defined?
  - some reasons

### memory

#### address

##### Structures
- `PhysicalAddress`
  - `pub usize`: physical address
- `PhysicalPageNumber`
  - `pub usize`: physical page number
##### Support operations
- transformations between address and page_number
- arithmetic operations(+, +=, -, -=) for address and page number, transfromation between usize and them, and display.

#### config
page size, start and end address of memory, the end if the kernel space and heap size.

#### heap
- reserve the space for the whole heap.
- Since `buddy_system_allocator` has implemented [`alloc::alloc::GlobalAlloc`] trait, we can allocate data in it.
- `#[global_allocator]` This attribute allows configuring the choice of global allocator. Global allocator to route all default allocation requests to a custom object.

#### range
Continuous pages $[start, end)$, attributes: `pub start: T, pub end: T,`


Supported operations
- create `Range<T>` from `core::ops::Range<U>`
- check overlap between 2 Ranges.
- iterate all pages in Range.
- transformations between physical address and virtual address.
- get one value with an index.
- contains a specific value.

#### frame


---
