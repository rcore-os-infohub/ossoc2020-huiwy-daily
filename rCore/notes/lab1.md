
# Lab1

## 简述：在 rust_main 函数中，执行 ebreak 命令后至函数结束前，sp 寄存器的值是怎样变化的？
- First save the context of 32 general use registers and 2 CSRs. Since then size of a register is 8B, sp will decrease by 8*34.
- After the context is saved, the stack may decreases or increase, but the content of the context will be preserved because of the property of stack.
- When we need to restore the registers, it use sp to load the context and load sp as the last one in case of the change of sp, and sp will decrease by 8*34.

## 回答：如果去掉 rust_main 后的 panic 会发生什么，为什么？
- The program still does not exit after the end of rust_main, we should panic or call `shut_down()` explicitly.

## 实验
- 如果程序访问不存在的地址，会得到 Exception::LoadFault。模仿捕获 ebreak 和时钟中断的方法，捕获 LoadFault（之后 panic 即可）
- 在处理异常的过程中，如果程序想要非法访问的地址是 `0x0`，则打印 `SUCCESS!`。
- 添加或修改少量代码，使得运行时触发这个异常，并且打印出 `SUCCESS!`。
-------------------------
# notes

## CSR
- sepc 
  - automatically filled in by hardware.
  - address of instruction where the interrupt happens.
- scause 
  - automatically filled in by hardware.
  - whether it is a hardware interrupt and the cause of interrupt.
- stval 
  - automatically filled in by hardware.
  - additional information about the interrupt.
- stvec
  - guide the way to handle the interrupt
  - BASE[XLEN-1:2] | MODE
  - address 4 Byte aligned
  - MODE(0) => pc <- BASE
  - MODE(1) => pc <- MEM[BASE + 4 * cause]
- sstatus
  - guide the way to handle the interrupt
  - keep track of the processor’s currentoperating state.
- sie/sip
  - guide the way to handle the interrupt
  - identify the interrupt(Software Interrupt, Timer Interrupt, External Interrupt)

## Interrupt instructions
- ecall 
- sret
  - Supervisor Mode to User Mode
  - set `pc` to `sepc`.
- ebreak
  - raise a break
- mret 
  - Machine Mode to Supervisor Mode.
  - set `pc` to `mepc`

## CSR instructions
- `csrrw dst, csr, src` CSR Read Write
  - write `csr` to `dst` and write `src` to `csr`.
  - atomic
- `csrr dst, csr` CSR Read
- `csrw csr, src` CSR Write
- `csrc(i) csr, rs1` CSR Clear
- `csrs(i) csr, rs1` CSR Set
