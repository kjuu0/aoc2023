use itertools::Itertools;
use std::io::BufRead;

// TODO: try symbol first approach like for gear?
pub fn parts_sum(reader: impl BufRead) -> u32 {
    let mut sum = 0;
    let mut prev_symbols = vec![];
    let mut cur_symbols = vec![];
    let mut cur = "".to_string();
    let mut cur_intervals = vec![];

    let compute_line_sum = |prev_sym: &[usize],
                            cur_sym: &[usize],
                            next_sym: &[usize],
                            c: &str,
                            c_intervals: &[(usize, usize)]| {
        let mut s = 0;
        let mut symbols = vec![prev_sym.iter(), cur_sym.iter(), next_sym.iter()]
            .into_iter()
            .kmerge()
            .peekable();
        for &(start, end) in c_intervals {
            let lo = if start == 0 { 0 } else { start - 1 };
            while let Some(&&symbol) = symbols.peek() {
                if symbol > end {
                    break;
                } else if symbol >= lo {
                    s += c[start..end].parse::<u32>().unwrap();
                    break;
                }
                symbols.next();
            }
        }
        s
    };
    for line in reader.lines() {
        let line = line.expect("failed to read string");
        let mut next_intervals = vec![];
        let mut next_symbols = vec![];
        let mut start = None;
        for (i, c) in line.char_indices() {
            if c.is_digit(/*radix=*/ 10) {
                if start.is_none() {
                    start = Some(i);
                }
            } else {
                if start.is_some() {
                    next_intervals.push((start.unwrap(), i));
                    start = None;
                }
                if c != '.' {
                    next_symbols.push(i);
                }
            }
        }
        if start.is_some() {
            next_intervals.push((start.unwrap(), line.len()));
        }

        sum += compute_line_sum(
            &prev_symbols,
            &cur_symbols,
            &next_symbols,
            &cur,
            &cur_intervals,
        );
        prev_symbols = cur_symbols;
        cur_symbols = next_symbols;
        cur = line;
        cur_intervals = next_intervals;
    }
    sum + compute_line_sum(&prev_symbols, &cur_symbols, &[], &cur, &cur_intervals)
}

pub fn gear_ratio_sum(reader: impl BufRead) -> u32 {
    let mut sum = 0;
    let mut prev = "".to_string();
    let mut cur = "".to_string();

    let find_start_bound_incl = |bytes: &[u8], mut start: usize| {
        while start > 0 && bytes[start - 1].is_ascii_digit() {
            start -= 1;
        }
        start
    };
    let find_end_bound_excl = |bytes: &[u8], mut end: usize| {
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        end
    };

    let process_row = |row: &str, gear_idx: usize| {
        let row_b = row.as_bytes();
        let mut gear_ratio = 1;
        let mut num_neighbors = 0;
        if row_b[gear_idx].is_ascii_digit() {
            let (start, end) = (
                find_start_bound_incl(row_b, gear_idx),
                find_end_bound_excl(row_b, gear_idx),
            );
            gear_ratio *= row[start..end].parse::<u32>().unwrap();
            num_neighbors += 1;
        } else {
            if gear_idx > 0 && row_b[gear_idx - 1].is_ascii_digit() {
                let start = find_start_bound_incl(row_b, gear_idx - 1);
                gear_ratio *= row[start..gear_idx].parse::<u32>().unwrap();
                num_neighbors += 1;
            }
            if gear_idx + 1 < row_b.len() && row_b[gear_idx + 1].is_ascii_digit() {
                let end = find_end_bound_excl(row_b, gear_idx + 1);
                gear_ratio *= row[(gear_idx + 1)..end].parse::<u32>().unwrap();
                num_neighbors += 1;
            }
        }
        (gear_ratio, num_neighbors)
    };

    for line in reader.lines() {
        let next = line.expect("failed to read line");
        sum += cur
            .bytes()
            .enumerate()
            .filter_map(|(i, b)| if b == b'*' { Some(i) } else { None })
            .filter_map(|i| {
                let (prev_ratio, prev_neighbors) = process_row(&prev, i);
                let (cur_ratio, cur_neighbors) = process_row(&cur, i);
                if prev_neighbors + cur_neighbors > 2 {
                    return None;
                }
                let (next_ratio, next_neighbors) = process_row(&next, i);
                if prev_neighbors + cur_neighbors + next_neighbors != 2 {
                    return None;
                } else {
                    Some(prev_ratio * cur_ratio * next_ratio)
                }
            })
            .sum::<u32>();
        prev = cur;
        cur = next;
    }
    sum + cur
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| if b == b'*' { Some(i) } else { None })
        .filter_map(|i| {
            let (prev_ratio, prev_neighbors) = process_row(&prev, i);
            let (cur_ratio, cur_neighbors) = process_row(&cur, i);
            let neighbors = prev_neighbors + cur_neighbors;
            if neighbors != 2 {
                return None;
            } else {
                Some(prev_ratio * cur_ratio)
            }
        })
        .sum::<u32>()
}
