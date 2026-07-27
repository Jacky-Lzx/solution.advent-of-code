use anyhow::Result;
use std::fs;

enum Instructions {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc,
    Out(u64),
    Bdv(u64),
    Cdv(u64),
}

impl Instructions {
    pub fn new(opcode: u64, operand: u64) -> Option<Instructions> {
        match opcode {
            0 => Some(Instructions::Adv(operand)),
            1 => Some(Instructions::Bxl(operand)),
            2 => Some(Instructions::Bst(operand)),
            3 => Some(Instructions::Jnz(operand)),
            4 => Some(Instructions::Bxc),
            5 => Some(Instructions::Out(operand)),
            6 => Some(Instructions::Bdv(operand)),
            7 => Some(Instructions::Cdv(operand)),
            _ => None,
        }
    }
}

fn get_operand_val(operand: u64, mem: &[u64; 3]) -> Result<u64> {
    match operand {
        0..=3 => Ok(operand),
        4 => Ok(mem[0]), // A
        5 => Ok(mem[1]), // B
        6 => Ok(mem[2]), // C
        7 => {
            anyhow::bail!("Operand 7 is reserved");
        }
        _ => {
            anyhow::bail!("Operand except 0..=7 is not supported");
        }
    }
}

fn execute(ins: Instructions, mem: &mut [u64; 3], isp: &mut u64, stdout: &mut Vec<u64>) {
    match ins {
        Instructions::Adv(combo) => mem[0] /= 2u64.pow(get_operand_val(combo, mem).unwrap() as u32),
        Instructions::Bxl(literal) => mem[1] ^= literal,
        Instructions::Bst(combo) => mem[1] = get_operand_val(combo, mem).unwrap() % 8,
        Instructions::Jnz(literal) => {
            if mem[0] != 0 {
                *isp = literal;
            }
        }
        Instructions::Bxc => mem[1] ^= mem[2],
        Instructions::Out(combo) => {
            stdout.push(get_operand_val(combo, mem).unwrap() % 8);
        }
        Instructions::Bdv(combo) => {
            mem[1] = mem[0] / 2u64.pow(get_operand_val(combo, mem).unwrap() as u32)
        }
        Instructions::Cdv(combo) => {
            mem[2] = mem[0] / 2u64.pow(get_operand_val(combo, mem).unwrap() as u32)
        }
    }

    match ins {
        Instructions::Jnz(_) => {
            if mem[0] == 0 {
                *isp += 2
            }
        }
        _ => *isp += 2,
    }
}

fn run(mem: &[u64; 3], ram: &[u64]) -> Result<Vec<u64>> {
    let mut mem = *mem;
    let mut isp: u64 = 0;
    let mut stdout: Vec<u64> = Vec::new();

    while (isp as usize) < ram.len() {
        // println!("isp: {}", isp);

        let opcode = ram[isp as usize];
        let operand = ram[(isp + 1) as usize];

        if let Some(ins) = Instructions::new(opcode, operand) {
            execute(ins, &mut mem, &mut isp, &mut stdout);
        } else {
            anyhow::bail!("Invalid opcode: {}", opcode);
        }
    }

    Ok(stdout)
}

fn part_1(mem: &[u64; 3], ram: &[u64]) -> Result<()> {
    let stdout = run(mem, ram)?;

    println!(
        "Part 1: {}",
        stdout
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(",")
    );

    Ok(())
}

fn dfs(idx: usize, start: u64, ram: &[u64]) -> Option<u64> {
    for a in start..(start + 8) {
        let mut b;
        #[allow(clippy::needless_late_init)]
        let c;

        b = a % 8; // 2, 4
        b ^= 1; // 1, 1
        c = a / 2u64.pow(b as u32); // 7, 5
        b ^= 5; // 1, 5
        b ^= c; // 4, 0
        b %= 8; // out instruction

        if b != ram[idx] {
            continue;
        }

        if idx == 0 {
            return Some(a);
        }

        if let Some(res) = dfs(idx - 1, a * 8, ram) {
            return Some(res);
        }
    }

    None
}

fn part_2(_mem: &[u64; 3], ram: &[u64]) -> Result<()> {
    if let Some(res) = dfs(ram.len() - 1, 0, ram) {
        println!("Part 2: {}", res);
        Ok(())
    } else {
        anyhow::bail!("No solution found")
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_17.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let mut line_it = contents.lines();

    let [reg_a, reg_b, reg_c] = [(); 3].map(|_| {
        line_it
            .next()
            .unwrap()
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap()
    });

    line_it.next();

    let ram = line_it
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("reg_a: {:?}, reg_b: {:?}, reg_c: {:?}", reg_a, reg_b, reg_c);
    println!("ram: {:?}", ram);
    println!();

    part_1(&[reg_a, reg_b, reg_c], &ram)?;
    part_2(&[reg_a, reg_b, reg_c], &ram)?;

    Ok(())
}
