use super::{Error, List, ListedTrait, Result, ToArcString};
use std::{cell::RefCell, sync::Arc};

pub fn cast_none_as_index_option(none: ()) -> Option<usize> {
    // Boring but convenient to have as a function.
    None
}

pub fn cast_as_index(option: Option<usize>) -> Option<usize> {
    // Boring but convenient to have as a function.
    option
}

pub fn cast_as_no_index(option: Option<usize>) -> Option<()> {
    // Swaperoo.
    match option {
        Some(_) => None,
        None => Some(()),
    }
}

fn char_boundary_ceil(text: &str, mut index: usize) -> usize {
    // Nightly: https://doc.rust-lang.org/std/string/struct.String.html#method.ceil_char_boundary
    if index > text.len() {
        return text.len();
    }
    while index < text.len() && !text.is_char_boundary(index) {
        index += 1;
    }
    index
}

fn char_boundary_floor(text: &str, mut index: usize) -> usize {
    // Nightly: https://doc.rust-lang.org/std/string/struct.String.html#method.floor_char_boundary
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

// Methods here take &str instead of Arc<String> because &str is enough.
// Doesn't matter too much, but it seems good to require only what's needed
// when functions are manually designed.
pub fn count_between(text: &str, begin: usize, end: usize) -> i32 {
    let begin = char_boundary_ceil(text, begin);
    let end = char_boundary_floor(text, end);
    if begin >= end {
        return 0;
    }
    text[begin..end].chars().count().try_into().unwrap()
}

pub fn for_each(text: &str, action: &dyn Fn(i32)) {
    for code in text.chars() {
        // And valid unicode always fits in i32.
        action(code as i32);
    }
}

// But functions here still produce Arc<String> for convenience.
pub fn from_code_point(code: i32) -> Result<Arc<String>> {
    Ok(char::from_u32(code as u32)
        .ok_or(Error::new())?
        .to_arc_string())
}

pub fn from_code_points(codes: &dyn ListedTrait<i32>) -> Result<Arc<String>> {
    let result: RefCell<Option<String>> = RefCell::new(None);
    codes.with_vec(&|values: &Vec<i32>| {
        result.replace(
            values
                .iter()
                .map(|&code| std::char::from_u32(code as u32))
                .collect(),
        );
    });
    Ok(Arc::new(result.into_inner().ok_or(Error::new())?))
}

pub fn get(text: &str, index: usize) -> Result<i32> {
    if index >= text.len() {
        return Err(Error::new());
    }
    let index = char_boundary_floor(text, index);
    Ok(text.get(index..).unwrap().chars().next().unwrap() as i32)
}

pub fn has_at_least(text: &str, begin: usize, end: usize, min_count: i32) -> bool {
    let begin = char_boundary_ceil(text, begin);
    let end = char_boundary_floor(text, end);
    if begin >= end {
        return min_count == 0;
    }
    let mut count = 0;
    for _ in text[begin..end].chars() {
        count += 1;
        if count >= min_count {
            return true;
        }
    }
    false
}

pub fn has_index(text: &str, index: usize) -> bool {
    index < text.len()
}

pub fn next(text: &str, index: usize) -> usize {
    // TODO Worry about max usize value?
    char_boundary_ceil(text, index + 1)
}

pub fn prev(text: &str, index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    char_boundary_floor(text, index - 1)
}

// TODO Should this one take Arc<String> in case of full string slice?
pub fn slice(text: &str, begin: usize, end: usize) -> Arc<String> {
    let begin = char_boundary_ceil(text, begin);
    let end = char_boundary_floor(text, end);
    if begin >= end {
        return "".to_arc_string();
    }
    text[begin..end].to_arc_string()
}

pub fn split(text: &str, sep: &str) -> List<Arc<String>> {
    let parts = if sep.is_empty() {
        text.chars().map(|x| Arc::new(x.to_string())).collect()
    } else {
        text.split(sep).map(|x| Arc::new(x.to_string())).collect()
    };
    Arc::new(parts)
}

pub fn to_float64(text: &str) -> Result<f64> {
    // See: https://doc.rust-lang.org/std/primitive.f64.html#method.from_str
    // We're more flexible in trimming than Rust.
    let text = text.trim();
    // And we can consider bytes because all valid numbers for us are ascii.
    let bytes = text.as_bytes();
    let Some(last) = bytes.last() else {
        return Err(Error::new());
    };
    // But less flexible in main content.
    let ok = match last {
        b'0'..=b'9' => match bytes.iter().position(|&b| b == b'.') {
            Some(dot) => dot > 0 && (b'0'..=b'9').contains(&bytes[dot - 1]),
            None => true,
        },
        b'N' => text.ends_with("NaN"),
        b'y' => text.ends_with("Infinity"),
        _ => false,
    };
    match ok {
        true => text.parse().map_err(|e| Error::with_source(Arc::new(e))),
        false => Err(Error::new()),
    }
}

pub fn to_int(text: &str, radix: Option<i32>) -> Result<i32> {
    let radix = radix.unwrap_or(10);
    if radix < 2 || radix > 36 {
        // This panics in Rust. TODO Should it panic in Temper?
        return Err(Error::new());
    }
    // See rules here: https://github.com/rust-lang/rust/blob/01e2fff90c7ed19e1d9fb828ebc012e7b9732297/library/core/src/num/mod.rs#L1439
    i32::from_str_radix(text.trim(), radix as u32).map_err(|e| Error::with_source(Arc::new(e)))
}
