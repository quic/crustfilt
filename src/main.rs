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

// Frames
fn parse_insn(line: &String) -> Result<(usize, Vec<u16>), ()> {
    // lines are  addr: bytes  <unknown>
    let bstart: usize = line.find(":").ok_or(())?;
    let bend = line.find("<unknown>").ok_or(())?;

    let mut sz = 0;
    let mut bytes = vec![];
    for data in line[bstart + 1..bend].trim().split(" ") {
        if data.len() == 8 {
            sz += 4;
            bytes.push(u16::from_str_radix(&data[0..4], 16).map_err(|_| ())?);
            bytes.push(u16::from_str_radix(&data[4..8], 16).map_err(|_| ())?);
        } else {
            sz += 2;
            bytes.push(u16::from_str_radix(&data[0..4], 16).map_err(|_| ())?);
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
    let cli = Cli::parse();

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
