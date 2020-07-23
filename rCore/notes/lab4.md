# lab4

## note

### Thread

`ThreadID`, `stack`, pointer to the process owning it, `inner`.

`inner` contains the important information about the state of thread.

create a thread
- allocate the thread stack according to the process.
- create the context of the thread with `sp`, `ep` ready.
- pack it into a thread.

Save the context of the thread whose address is saved in register `spec` during a timer interrupt.

When restoring we also need to restore the page table.

### Process

`is_user`, `memory_set`.
`memory_set` is used to hold the pulic page table of the threads created by the process.

`alloc_page_range` allocate a countinuous space (why) for the process, used to generate the stack for its threads.

### Processor

`current_thread`, `scheduler`, `sleeping_threads` 

Operations on arranging the threads. Panic if no active or sleeping threads left.

### Kernel stack

The thread cannot necesarily find its thread stack, so we need a kernel stack to store and restore the context.

### the whole process

- call `_start` in `entry.asm` to start the kernel. 
  - set the pointer to `boot_page_table` and allocate the `boot_stack`.
  - jump to `rust_main`
- init the services, `memory` and `interrupt`.
- Create the kernel process and its threads, set the `ra` register to the function that exit this process.


