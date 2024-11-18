
# Project Description

Example application that shows LLVM toolchian users how they can extend the capabilities of the toolchain 
to handle disassembling new or proprietary instructions that it does not support.

```
clang  --target=riscv32 -c test.c 
llvm-objdump -d | coprocfilt 
```

Building:
```
cargo run 
```


# License
See the LICENSE file for more details 
