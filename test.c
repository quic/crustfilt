
// Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause


int main() {
    int a = 0;
    int c = 32;

    asm(".insn 8, 0x2000200940003F"); // 64 custom instruction
    asm(".insn 6, 0x021F | 0x00001000 << 32"); // xqci.e.li rd=8 imm=0x1000
    asm(".insn 4, 0x84F940B"); // xqci.insbi  
    asm(".insn 2, 0xB8F2"); // cm.push
    asm(".insn 4, 0x84F940B"); // xqci.insbi  
    asm(".insn 2, 0xB8F2"); // cm.push
    return a * c;
}
