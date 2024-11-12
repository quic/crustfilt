
Application to show decoding inline string applications with custom unshared asm utilties. 

demo:
```
clang  --target=riscv32 -c test.c 
llvm-objdump -d | coprocfilt 
```

Building:
cargo run 


