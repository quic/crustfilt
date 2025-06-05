// Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

mod decoder;

use clap::Parser;

#[derive(Parser)]
#[command(version, author, about, long_about=None)]
struct Cli {
    #[arg(short, default_value_t = 1)]
    ver: u32,
}

mod tests {

    use super::*;
    #[test]
    fn test_lldb() {
        let text = "0x3a6 <+22>: fea42623                 sw     a0, -0x14(s0)
    0x3aa <+26>: 0940003f 00200020               
    0x3b2 <+34>: 021f 0000 1000                  
    0x3b8 <+40>: 084f940b                        
    0x3bc <+44>: b8f2                            
    0x3be <+46>: 084f940b                        
    0x3c2 <+50>: b8f2                            
    0x3c4 <+52>: ff042503                 lw     a0, -0x10(s0) ";

        for (i, line) in text.lines().enumerate() {
            if let Ok(data) = parse_insn(&line.to_owned()) {
                println!("{i}: {}", decoder::decode(data.0, data.1).unwrap());
                assert!(if [1, 2, 3, 4, 5, 6].iter().find(|x| **x == i).is_some() {
                    true
                } else {
                    false
                });
            } else {
                println!("{line}");
            }
        }
    }

    #[test]
    fn test_llvm() {
        let text = "
     3a6: fea42623     	sw	a0, -0x14(s0)
     3aa: 0940003f 00200020    	<unknown>
     3b2: 021f 0000 1000       	<unknown>
     3b8: 084f940b     	<unknown>
     3bc: b8f2         	fsd	ft8, 0x70(sp)
     3be: 084f940b     	<unknown>
     3c2: b8f2         	fsd	ft8, 0x70(sp)
     3c4: ff042503     	lw	a0, -0x10(s0)";

        for (i, line) in text.lines().enumerate() {
            if let Ok(data) = parse_insn(&line.to_owned()) {
                println!("{i}:  {}", decoder::decode(data.0, data.1).unwrap());
                assert!(if [2, 3, 4, 6].iter().find(|x| **x == i).is_some() {
                    true
                } else {
                    false
                });
            } else {
                println!("{line}");
            }
        }
    }
}

// Frames
fn parse_insn(line: &String) -> Result<(usize, Vec<u16>), ()> {
    // lines are  addr: bytes  <unknown>
    let bstart: usize = line.find(":").ok_or(())?;
    let mut sz = 0;
    let mut bytes = vec![];

    // llvm objdump style
    if let Ok(bend) = line.find("<unknown>").ok_or(()) {
        for data in line[bstart + 1..bend].trim().split(" ") {
            let data = data.trim();
            if data.len() == 8 {
                sz += 4;
                bytes.push(u16::from_str_radix(&data[0..4], 16).map_err(|_| ())?);
                bytes.push(u16::from_str_radix(&data[4..8], 16).map_err(|_| ())?);
            } else {
                sz += 2;
                bytes.push(u16::from_str_radix(&data[0..4], 16).map_err(|_| ())?);
            }
        }
    } else {
        for data in line[bstart + 1..].trim().split(" ") {
            if data.len() == 8 {
                sz += 4;
                bytes.push(u16::from_str_radix(&data[0..4], 16).map_err(|_| ())?);
                bytes.push(u16::from_str_radix(&data[4..8], 16).map_err(|_| ())?);
            } else if data.len() == 4 {
                sz += 2;
                bytes.push(u16::from_str_radix(&data[0..4], 16).map_err(|_| ())?);
            } else {
                return Err(());
            }
        }
    }
    Ok((sz, bytes))
}

fn parse_line(s: &String) -> Result<String, ()> {
    let s = s;
    if let Some(idx) = s.find("<unknown>") {
        let insn = parse_insn(&s)?;

        Ok(format!("{}{}", &s[..idx], decoder::decode(insn.0, insn.1)?))
    } else {
        Ok(s.to_owned())
    }
}

fn main() {
    let _cli = Cli::parse();
    for line in std::io::stdin().lines() {
        if let Ok(data) = line {
            if let Ok(dline) = parse_line(&data) {
                println!("{}", dline)
            } else {
                println!("{}", &data)
            }
        }
    }
}
