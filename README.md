
# Project Description

Example application that shows LLVM toolchain users how they can extend the capabilities of the toolchain 
to handle disassembling new or proprietary instructions that it does not support.

In the code snippet below, llvm-objdump outputs ```<unknown>``` for instructions
it does not recognize.

```
clang  --target=riscv32 -march=rv32imac_zcmp_xqci -menable-experimental-extensions -c test.c -o test.out
llvm-objdump -d test.out

00000000 <main>:
       0: 1101          addi    sp, sp, -0x20
       2: ce06          sw      ra, 0x1c(sp)
       4: cc22          sw      s0, 0x18(sp)
       6: 1000          addi    s0, sp, 0x20
       8: 4501          li      a0, 0x0
       a: fea42a23      sw      a0, -0xc(s0)
       e: fea42823      sw      a0, -0x10(s0)
      12: 02000513      li      a0, 0x20
      16: fea42623      sw      a0, -0x14(s0)
      1a: 0940003f 00200020     <unknown>
      22: 021f 0000 1000        qc.e.li tp, 0x10000000
      28: 084f940b      qc.insbi        s0, -0x1, 0x5, 0x4
      2c: b8f2          cm.push {ra, s0-s11}, -64
      2e: 084f940b      qc.insbi        s0, -0x1, 0x5, 0x4
      32: b8f2          cm.push {ra, s0-s11}, -64
      34: ff042503      lw      a0, -0x10(s0)
      38: fec42583      lw      a1, -0x14(s0)
      3c: 02b50533      mul     a0, a0, a1
      40: 40f2          lw      ra, 0x1c(sp)
      42: 4462          lw      s0, 0x18(sp)
      44: 6105          addi    sp, sp, 0x20
      46: 8082          ret
```
The custom instruction filter processes llvm-objdump output and correctly
disassembles the custom instruction ```Fake64```, defined in ```test.c```,
which is not supported in the toolchain.

```
llvm-objdump -d test.out | crustfilt

00000000 <main>:
       0: 1101          addi    sp, sp, -0x20
       2: ce06          sw      ra, 0x1c(sp)
       4: cc22          sw      s0, 0x18(sp)
       6: 1000          addi    s0, sp, 0x20
       8: 4501          li      a0, 0x0
       a: fea42a23      sw      a0, -0xc(s0)
       e: fea42823      sw      a0, -0x10(s0)
      12: 02000513      li      a0, 0x20
      16: fea42623      sw      a0, -0x14(s0)
      1a: 0940003f 00200020     Fake64
      22: 021f 0000 1000        qc.e.li tp, 0x10000000
      28: 084f940b      qc.insbi        s0, -0x1, 0x5, 0x4
      2c: b8f2          cm.push {ra, s0-s11}, -64
      2e: 084f940b      qc.insbi        s0, -0x1, 0x5, 0x4
      32: b8f2          cm.push {ra, s0-s11}, -64
      34: ff042503      lw      a0, -0x10(s0)
      38: fec42583      lw      a1, -0x14(s0)
      3c: 02b50533      mul     a0, a0, a1
      40: 40f2          lw      ra, 0x1c(sp)
      42: 4462          lw      s0, 0x18(sp)
      44: 6105          addi    sp, sp, 0x20
      46: 8082          ret
```

This method can also be used with lldb to generate assembly for custom
instructions not supported upstream.

The next example shows how to use LLDB to debug a RISC-V binary
on QEMU using the custom instruction filter.

Launch QEMU directly through LLDB's command ```platform select qemu-user``` or
start QEMU with GDB remote debugging enabled on a local port,
e.g., ```qemu -g 10000 test.out```.

Run LLDB, load the test executable, and set a breakpoint at test.c main
function:

```
lldb
(lldb) file test.out
lldb) b main
(lldb) c
```

Load ```filter_disasm.py```, the sample python script implementing the
new command ```fdis``` to invoke crustfilt, and run the command.

```
(lldb) command script import filter_disasm.py
Disassembly filter command (fdis) loaded
Filter program set to crustfilt
(lldb) fdis
test.out`main:
->  0x1047c <+0>:  1101                     addi   sp, sp, -0x20
    0x1047e <+2>:  ce06                     sw     ra, 0x1c(sp)
    0x10480 <+4>:  cc22                     sw     s0, 0x18(sp)
    0x10482 <+6>:  1000                     addi   s0, sp, 0x20
    0x10484 <+8>:  4501                     li     a0, 0x0
    0x10486 <+10>: fea42a23                 sw     a0, -0xc(s0)
    0x1048a <+14>: fea42823                 sw     a0, -0x10(s0)
    0x1048e <+18>: 02000513                 li     a0, 0x20
    0x10492 <+22>: fea42623                 sw     a0, -0x14(s0)
    0x10496 <+26>: 0940003f 00200020        Fake64
    0x1049e <+34>: 021f 0000 1000           qc.e.li tp, 0x10000000
    0x104a4 <+40>: 084f940b                 qc.insbi s0, -0x1, 0x5, 0x4
    0x104a8 <+44>: b8f2                     CmPush {ra, s0-s11},-0
    0x104aa <+46>: 084f940b                 qc.insbi s0, -0x1, 0x5, 0x4
    0x104ae <+50>: b8f2                     CmPush {ra, s0-s11},-0
    0x104b0 <+52>: ff042503                 lw     a0, -0x10(s0)
    0x104b4 <+56>: fec42583                 lw     a1, -0x14(s0)
    0x104b8 <+60>: 02b50533                 mul    a0, a0, a1
    0x104bc <+64>: 40f2                     lw     ra, 0x1c(sp)
    0x104be <+66>: 4462                     lw     s0, 0x18(sp)
    0x104c0 <+68>: 6105                     addi   sp, sp, 0x20
    0x104c2 <+70>: 8082                     ret
(lldb)
```

This will invoke the custom instruction disassembler and display the decoded
custom instructions such as:

```
0x10496 <+26>: 0940003f 00200020 Fake64
```

The disassembly output from LLDB now aligns more closely with llvm-objdump,
showing decoded custom instructions when the filter is active.

The upstream patch for RISC-V LLDB that includes this support is currently
available at [#145793](https://github.com/llvm/llvm-project/pull/145793).


Building:
```
cargo run 
```


# License
See the LICENSE file for more details 
