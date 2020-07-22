# lab3
Virtual memory, page table, kernel remap.
## Notes

### Segment
- Sections of the program(text, rodata, data, bss)
- Different segments require different 
- page aligned

Attributes
- `pub map_type: MapType`, the type of map for this segment, linear for os, frame for applications 
- `pub range: Range<VirtualAddress>`, the start and end address of this segment in virtual address.
- `pub flags: Flags`, the of the segment.

Methods

### mapping

`page_tables`, `root_ppn`, `mapped_pairs`

#### linear map
- traverse the segment and get its transformed ppn to the page table.

#### frame map
- traverse the segment, for each vpn, allocate a ppn and add them to the page table.

why the alignment is not taken into consideration in linear case?

why `mapping.rs::find_entry` does not need the big page case?

### memory set

The data structure describing memory of the whole program, includes the map, and all the segments.

### page table entry
A `usize` Sv39 bitfield, containing the MODE, VPA and flags. 

`has_next_level` whether this page table entry has next level. Used for big page.

### page table
a page table contains `PAGE_SIZE / 8` page table entries. 

`get_next_table` get the next page table of this entry.

