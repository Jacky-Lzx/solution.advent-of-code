use std::fs;

use anyhow::Result;

fn get_none_len(arr: &[Option<u64>], none_idx: usize) -> usize {
    let mut count = 0;
    while let Some(None) = arr.get(none_idx + count) {
        count += 1;
    }

    count
}

fn get_file_start_idx(arr: &[Option<u64>], file_idx: usize) -> Option<usize> {
    assert!(arr[file_idx].is_some(), "file_idx must point to a file");

    let mut idx = file_idx - 1;
    while let Some(Some(v)) = arr.get(idx)
        && v == &arr[file_idx].unwrap()
    {
        if idx == 0 {
            return None;
        }

        idx -= 1;
    }

    Some(idx + 1)
}

fn part_1(line: &[u64]) -> Result<()> {
    let count = line.iter().sum::<u64>() as usize;
    let mut array: Vec<Option<u64>> = vec![None; count];

    let mut array_idx = 0;
    let mut id = 0;
    let mut is_file = true;

    for &value in line.iter() {
        if is_file {
            array[array_idx..(array_idx + value as usize)].fill(Some(id));
            id += 1;
        } else {
            array[array_idx..(array_idx + value as usize)].fill(None);
        }
        array_idx += value as usize;
        is_file = !is_file;
    }

    // array.iter_mut().for_each(|x| match x {
    //     Some(id) => print!("{id}"),
    //     None => print!("."),
    // });
    // println!();

    let mut end_idx = array.len() - 1;
    let mut array_idx = 0;

    loop {
        // Assert the end of the array is not None
        while end_idx > 0 && array[end_idx].is_none() {
            end_idx -= 1;
        }

        while array_idx < end_idx && array[array_idx].is_some() {
            array_idx += 1;
        }

        if array_idx >= end_idx {
            break;
        }

        array[array_idx] = array[end_idx];
        array[end_idx] = None;
    }

    // array.iter_mut().for_each(|x| match x {
    //     Some(id) => print!("{id}"),
    //     None => print!("."),
    // });
    // println!();

    let sum = array
        .iter()
        .enumerate()
        .map(|(i, x)| match x {
            Some(id) => i as u64 * id,
            None => 0,
        })
        .sum::<u64>();

    println!("Part 1: {}", sum);

    Ok(())
}

fn part_2(line: &[u64]) -> Result<()> {
    let count = line.iter().sum::<u64>() as usize;
    let mut array: Vec<Option<u64>> = vec![None; count];

    let mut array_idx = 0;
    let mut id = 0;
    let mut is_file = true;

    for &value in line.iter() {
        if is_file {
            array[array_idx..(array_idx + value as usize)].fill(Some(id));
            id += 1;
        } else {
            array[array_idx..(array_idx + value as usize)].fill(None);
        }
        array_idx += value as usize;
        is_file = !is_file;
    }

    // array.iter_mut().for_each(|x| match x {
    //     Some(id) => print!("{id}"),
    //     None => print!("."),
    // });
    // println!();

    let mut file_idx = array.len() - 1;

    'outer: loop {
        let mut none_idx = 0;

        let file_size;

        while array[file_idx].is_none() {
            if file_idx == 0 {
                break 'outer;
            }

            file_idx -= 1;
        }

        if let Some(idx) = get_file_start_idx(&array, file_idx) {
            file_size = file_idx - idx + 1;
            file_idx = idx;
        } else {
            break;
        }

        while none_idx < file_idx
            && (array[none_idx].is_some()
                || (array[none_idx].is_none() && get_none_len(&array, none_idx) < file_size))
        {
            if array[none_idx].is_some() {
                none_idx += 1;
            } else {
                let none_len = get_none_len(&array, none_idx);
                if none_len < file_size {
                    none_idx += none_len;
                }
            }
        }

        if none_idx >= file_idx {
            if file_idx == 0 {
                break;
            }
            file_idx -= 1;
            continue;
        }

        let (arr_l, arr_r) = array.split_at_mut(file_idx);

        arr_l[none_idx..(none_idx + file_size)].clone_from_slice(&arr_r[..file_size]);
        arr_r[..file_size].fill(None);

        if file_idx == 0 {
            break;
        }
        file_idx -= 1;
    }

    // array.iter_mut().for_each(|x| match x {
    //     Some(id) => print!("{id}"),
    //     None => print!("."),
    // });
    // println!();

    let sum = array
        .iter()
        .enumerate()
        .map(|(i, x)| match x {
            Some(id) => i as u64 * id,
            None => 0,
        })
        .sum::<u64>();

    println!("Part 2: {}", sum);

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_9.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    anyhow::ensure!(lines.len() == 1, "Input file must have one line");

    let line = lines[0];

    let line = line
        .chars()
        .map(|c| c.to_digit(10).map(|x| x as u64))
        .collect::<Option<Vec<u64>>>()
        .ok_or_else(|| anyhow::anyhow!("Invalid character in input"))?;

    part_1(&line)?;
    part_2(&line)?;

    Ok(())
}
