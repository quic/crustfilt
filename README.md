
# Project Description

Example application that shows LLVM toolchain users how they can extend the capabilities of the toolchain 
to handle disassembling new or proprietary instructions that it does not support.

In the code snippet below, llvm-objdump outputs ```<unknown>``` for instructions
it does not recognize.

```
clang  --target=riscv32 -march=rv32imac_zcmp_xqci -menable-experimental-extensions -c test.c
llvm-objdump -d

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
llvm-objdump -d | crustfilt

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
instruction not supported upstream.

Building:
```
cargo run 
```


# License
See the LICENSE file for more details 
