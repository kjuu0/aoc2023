use itertools::Itertools;
use std::io::BufRead;

pub fn parts_sum(reader: impl BufRead) -> u32 {
    let mut sum = 0;
    let mut prev_symbols = vec![];
    let mut cur_symbols = vec![];
    let mut cur = "".to_string();
    let mut cur_intervals = vec![];

    let compute_line_sum = |prev_sym: &[usize], cur_sym: &[usize], next_sym: &[usize], c: &str, c_intervals: &[(usize, usize)]| {
        let mut s = 0;
        let mut symbols = vec![prev_sym.iter(), cur_sym.iter(), next_sym.iter()]
            .into_iter()
            .kmerge().peekable();
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
        
        sum += compute_line_sum(&prev_symbols, &cur_symbols, &next_symbols, &cur, &cur_intervals);
        prev_symbols = cur_symbols;
        cur_symbols = next_symbols;
        cur = line;
        cur_intervals = next_intervals;
    }
    sum + compute_line_sum(&prev_symbols, &cur_symbols, &[], &cur, &cur_intervals)
}
