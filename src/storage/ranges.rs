use std::ops::Range;

pub fn clamp(len: usize, index: i64) -> usize {
    match index {
        0 => 0,
        i if i < 0 && -i >= len as i64 => 0,
        i if i < 0 => (i + len as i64) as usize,
        i if i > len as i64 => len,
        i => i as usize,
    }
}

pub fn clamp_range(len: usize, start: i64, end: i64) -> Range<usize> {
    let start = clamp(len, start);
    let end = 1 + clamp(len, end);

    println!("start: {}, end: {}", start, end);

    if len < end {
        start..len
    } else if start < end {
        start..end
    } else {
        start..start
    }
}
