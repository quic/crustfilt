# Supporting External Instructions On Top of LLVM

There are two broad approaches to supporting custom instructions in the RISC-V
backend for LLVM.

The first is to directly modify and rebuild LLVM, providing definitions of the
custom instructions via TableGen (and maybe C++). This brings a maintenance
burden to support releasing and updating a toolchain, as well as updating any
definitions of custom instructions as they are developed.

The other approach, which `crustfilt` provides part of the strategy for, is to
support custom extensions without modifying the toolchain. This can have
significant advantages, but also only adds assembly/disassembly support for the
instructions.

In many cases, such as proprietary extensions, or very early experimental phases
of an extension's development, assembly and disassembly support is enough and
code generation support is not required.

## Advantages

The main advantages of this approach are:

* Faster turnaround time for changes when experimenting with ISA extension
  design
* No need for separate toolchains if the extension is proprietary
* Easier to control the distribution of proprietary information
* Users can use up-to-date toolchains without the pain of rebasing lots of LLVM
  changes.

## Disadvantages

There are of course trade-offs in this approach:

* You can only use the (conventional for RISC-V) `<mnemonic> <operands>` syntax,
  this approach does not support assembly syntaxes like [the Hexagon
  ISA][hexagon]'s operator-based syntax.
* You need to remember to use `crustfilt` when disassembling, and in the
  debugger.
* Parts of this approach do not work with Link-Time Optimization (LTO).

[hexagon]: https://docs.qualcomm.com/bundle/publicresource/80-N2040-53_REV_AB_Qualcomm_Hexagon_V73_Programmers_Reference_Manual.pdf

## Assembling

There are two approaches here, one that is nicer, but only works for existing
RISC-V instruction formats, and another which has to be used if your
instructions do not fit into existing instruction types.

We have made significant improvements to support for the following assembly
features in LLVM 20. Earlier versions of LLVM may not support the features
described. Binutils has supported all these features for longer than LLVM.

### RISC-V Instruction Formats

You can use the `.insn <type> <operands>` format when using inline assembly.
Any register operands use the register name (i.e. `a0`). Immediates are written
directly.

This example shows how to assemble `qc.addsat a0, a1, a2` (from Qualcomm's
[`Xqcia` extension](https://github.com/quic/riscv-unified-db/releases/latest))
using instruction types. This extension has recently been added to LLVM, but
versions before 20 and binutils/gcc do not support it.

```asm
  .insn r 0xb, 0x3, 0x0e, a0, a1, a2
```

This could also be wrapped in an assembly macro, for clarity:

```asm
// Definition of `qc.addsat` instruction macro
.macro qc.addsat d:req, s1:req, s2:req
  // the `:req` suffix denotes these parameters are required.

  // Macro parameters are referenced using a backslash:
  .insn r 0xb, 0x3, 0x0e, \d, \s1, \s2
.endm

  // Use of `qc.addsat` macro
  qc.addsat a0, a1, a2
```

Macro definitions can be put into a separate file, and depending on whether you
are using the C preprocessor on assembly files, can either be `#include`d or
`.include`d where they are used.

[Binutils documents the instruction formats they
support](https://sourceware.org/binutils/docs/as/RISC_002dV_002dFormats.html)
when using `.insn`. LLVM matches this support.

This approach of using assembly macros has been taken from the [Hazard3
docs](https://github.com/Wren6991/Hazard3/releases/tag/v1.0.1) and expanded
upon.

### Raw Instruction Encodings

The situation for instructions that do not fit in an existing instruction format
is more difficult, but still possible. Instead of using the `.insn` directive
that takes an instruction type, we can use the variant that takes a length and a
raw value to encode.

There are various details to this approach that close attention needs to be paid
to, which will be highlighted after the examples.

We can use this approach to assemble the 48-bit `qc.e.li a0, 0xff00ff00`
instruction (from Qualcomm's [`Xqcili`
extension](https://github.com/quic/riscv-unified-db/releases/latest)):

```asm
  .insn 0x6, 0x1f | (10 << 7) | (0xff00ff00 << 16)
```

We can make this a lot easier to read, and add error checking, using assembly macros:

```asm
// Error-checking macros
.macro _valid_gpr_nox0 regN:req
  .if (\regN <= 0) || (\regN > 31)
    .error "Invalid GPRNoX0 number"
  .endif
.endm
.macro _valid_imm32 imm:req
  .if (\imm < 0) || (\imm > 0xffffffff)
    .error "Invalid 32-bit Immediate"
  .endif
.endm

// Definition of `qc.e.li` instruction macro
.macro qc.e.li dN:req, imm:req
  // Validate macro arguments
  _valid_gpr_nox0 \dN
  _valid_imm32 \imm

  .insn 0x6, 0x1f | (\dN << 7) | (\imm << 16)
.endm

  // Use of `qc.e.li` macro
  qc.e.li 10, 0xff00ff00
```

There are some limitations of this approach:

* You cannot use named registers, you can only refer to them by their encoding.

  If your extension adds additional registers, you can use `.set <regname>,
  <encoding>` to add a local absolute symbol, which can then be used in macro
  invocations and expressions.

  You cannot do this for existing registers, as that definition will shadow the
  existing identifier in the parser, and prevent you using that register name in
  conventional instructions. For instance, `.set a0, 10` would cause an issue
  with `addi a0, gp, 256` because the first operand is the symbol and therefore
  the instruction is equivalent to `addi 10, gp, 256` which is not a valid
  instruction in RISC-V.

* You must do any error checking yourself. This is done with the `_valid*`
  macros in the example, as well as the `:req` suffixes.

  If you have constraints on which registers can be used with an instruction,
  you should use these `_valid*` operators (like `_valid_gpr_nox0` shows) to
  throw an error when the constraint is not satisfied.

* You should not use `.word` directives, instead using `.insn`. This is because
  `.word` respects data endianness (which can be either big- or little-endian),
  but `.insn` will always match instruction endianness (which is always
  little-endian on RISC-V). There are other advantages to this approach, as the
  assembler should use mapping symbols to mark `.insn`-emitted bytes as
  instructions, not data.

* You should use the `.insn <length>, <encoding>` variant to be explicit about
  length, as LLVM and binutils have different behaviour about how they infer the
  length of an instruction in `.insn <encoding>` directives.

### Named CSRs

The easiest way to create new CSR names for using in CSR-accessing instructions
is with `.set <name>, <id>`, which produces a local absolute symbol that
evaluates to the ID of the CSR. CSR names should rarely conflict with existing
names.

For instance:

```asm
  // Definition of `qc_mclicie0`
  .set qc_mclicie0, 0x7f0

  csrrc a0, qc_mclicie0, a1
```

## Inline Assembly (from C/C++)

This is some specific guidance about trying to use the assembly features
described above from C/C++.

You can use `.include "macros.s"` from a global inline assembly block, as
long as the file is in your include path.

```c
__asm__(".include \"macros.s\"");
```

> Unfortunately, `.include` doesn't work with LTO in LLVM: [LLVM issue
> #112920](https://github.com/llvm/llvm-project/issues/112920).
>
> You can get around this by directly writing the `.insn` in the inline assembly
> block and avoid using macros at all. Assembly macros can only be defined once
> per Compilation Unit, so redefining them at each use is a bad idea.

Then, for example, to use the `qc.e.li` macro:

```c
// Adds 0xff00ff00 and saturates the result
int32_t my_add_then_saturate(int32_t input) {
  int32_t C;
  __asm__ ("qc.e.li %N[rd], 0xff00ff00" : [rd]"=r"(C) :::);

  int32_t res;
  __asm__ ("qc.addsat %[rd], %[rs1], %[rs2]" : [rd]"=r"(res) : [rs1]"r"(input), [rs2]"r"(C));

  return res;
}
```

Here are some relevant constraints that might be useful when hand-assembling:

- `r` gives a General Purpose Register (GPR) `x1`-`x31`. This prints as the
  register name. This will never give `x0` because that often is used for
  different functionality than the core instruction.
- `R` gives an even-odd GPR Pair. This prints as the even register, but both
  registers will be marked live.
- `cr` gives a RVC-compatible GPR, `x8-x15`.

Another useful thing is the `N` modifier (used in the first inline assembly
block in `my_add_then_saturate`) - this prints the raw encoding of the chosen
register (`0`-`31`) rather than the name, so can be used with assembly macros
that use the Raw Instruction Encoding, rather than instruction types.

More constraints and modifiers are described in the [RISC-V toolchain
conventions
document](https://github.com/riscv-non-isa/riscv-toolchain-conventions).

## Disassembling

Use `crustfilt`!

When disassembling, pass the output of `llvm-objdump -d` through `crustfilt`,
and your instructions should appear where previously they were `<unknown>`.

`llvm-objdump` follows Table 1 from the RISC-V unprivileged spec (including the
unratified encoding scheme for instructions over 32 bits long). In order for
`crustfilt` to work really well, it is worth following this encoding scheme, so
that `llvm-objdump`'s output stays in sync with the actual instruction stream.

## Debugging

It is possible to write a LLDB or GDB plugin to apply `crustfilt` to the output
of any disassemble commands.
