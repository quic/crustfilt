
int main() {
    int a = 0;
    int c = 32;

    asm(".word 0x0940003F"); asm(".word 0x200020"); // 64 custom instruction
    asm(".hword 0x21F"); asm(".word 0x00001000"); // xqci.e.li rd=8 imm=0x1000
    asm(".word 0x84F940B"); // xqci.insbi  
    asm(".hword 0xB8F2"); // cm.push
    asm(".hword 0x21F"); asm(".word 0x00001000"); // xqci.e.li rd=8 imm=0x1000
    asm(".word 0x84F940B"); // xqci.insbi  
    asm(".hword 0xB8F2"); // cm.push
    return a * c;
}
