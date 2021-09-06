use crate::{error::Error, MarkovChainRule};
use std::{collections::HashMap, iter::once, slice};

#[cfg(test)]
mod test;

pub fn generate_rule_from_data(content: &str, key_size: usize) -> Result<MarkovChainRule, Error> {
    if key_size < 1 {
        return Err(Error::InvalidKeySize);
    }

    let words: Vec<&str> = content.split_whitespace().collect();

    let mut dict: MarkovChainRule = HashMap::new();

    for slice in words.windows(key_size + 1) {
        let (key, value) = slice.split_at(key_size);
        let value = value[0];
        dict.entry(key.to_vec()).or_default().push(value);
    }

    Ok(dict)
}

pub fn generate_rule_from_data_in_place(
    content: &str,
    key_size: usize,
) -> Result<HashMap<&str, Vec<&str>>, Error> {
    if key_size < 1 {
        return Err(Error::InvalidKeySize);
    }

    let words: Vec<&str> = content.split_whitespace().collect();

    let mut dict: HashMap<&str, Vec<&str>> = HashMap::new();

    for slice in words.windows(key_size + 1) {
        let (key, value) = slice.split_at(key_size);
        let value = value[0];

        let ptr = key[0].as_ptr();
        let last = key.last().unwrap();
        let len = (last.as_ptr() as usize + last.len()) - ptr as usize;
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        let key = unsafe { std::str::from_utf8_unchecked(slice) };
        dict.entry(key).or_default().push(value);
    }

    Ok(dict)
}

pub fn generate_rule_from_data_in_place_safe(
    content: &str,
    key_size: usize,
) -> Result<HashMap<&str, Vec<&str>>, Error> {
    if key_size < 1 {
        return Err(Error::InvalidKeySize);
    }

    let space_indices = content
        .char_indices()
        .filter_map(|(pos, c)| c.is_whitespace().then(|| pos))
        .chain(once(content.len()));

    let word_boundaries: Vec<(usize, usize)> = space_indices
        .scan(None, |prev_end, end| {
            let start = match prev_end {
                None => 0,
                Some(prev_end) => *prev_end + 1,
            };
            let _ = prev_end.insert(end);

            Some((start, end))
        })
        .filter(|(start, end)| end - start > 0)
        .collect();

    let mut dict: HashMap<&str, Vec<&str>> = HashMap::new();

    for slice in word_boundaries.windows(key_size + 1) {
        let (&(value_start, value_end), keys) = slice.split_last().unwrap();

        let key_start = keys.first().unwrap().0;
        let key_end = keys.last().unwrap().1;

        dict.entry(&content[key_start..key_end])
            .or_default()
            .push(&content[value_start..value_end]);
    }

    Ok(dict)
}
