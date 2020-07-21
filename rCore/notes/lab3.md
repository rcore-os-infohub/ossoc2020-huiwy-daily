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
- 