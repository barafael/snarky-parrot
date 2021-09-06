use crate::{error::Error, MarkovChainRule};
use std::{
    collections::{BTreeMap, HashMap},
    iter::once,
    slice,
};

#[cfg(test)]
mod test;

/// Slightly faster, but using unsafe.
pub fn generate_rule_from_data_unsafe(
    content: &str,
    key_size: usize,
) -> Result<MarkovChainRule, Error> {
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

/// Slightly slower but no unsafe.
pub fn generate_rule_from_data(content: &str, key_size: usize) -> Result<MarkovChainRule, Error> {
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

/// Original function
pub fn generate_rule_from_data_vec(
    content: &str,
    key_size: usize,
) -> Result<HashMap<Vec<&str>, Vec<&str>>, Error> {
    if key_size < 1 {
        return Err(Error::InvalidKeySize);
    }

    let words: Vec<&str> = content.split_whitespace().collect();

    let mut dict: HashMap<Vec<&str>, Vec<&str>> = HashMap::new();

    for slice in words.windows(key_size + 1) {
        let (key, value) = slice.split_at(key_size);
        let value = value[0];
        dict.entry(key.to_vec()).or_default().push(value);
    }

    Ok(dict)
}

/// Safe version using BTreeMap
pub fn generate_rule_from_data_btreemap(
    content: &str,
    key_size: usize,
) -> Result<BTreeMap<&str, Vec<&str>>, Error> {
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

    let mut dict: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

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

/// Safe version using hashbrown
pub fn generate_rule_from_data_hashbrown_safe(
    content: &str,
    key_size: usize,
) -> Result<hashbrown::HashMap<&str, Vec<&str>>, Error> {
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

    let mut dict: hashbrown::HashMap<&str, Vec<&str>> = hashbrown::HashMap::new();

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

/// Safe version using unsafe hashbrown
pub fn generate_rule_from_data_hashbrown_unsafe(
    content: &str,
    key_size: usize,
) -> Result<hashbrown::HashMap<&str, Vec<&str>>, Error> {
    if key_size < 1 {
        return Err(Error::InvalidKeySize);
    }

    let words: Vec<&str> = content.split_whitespace().collect();

    let mut dict: hashbrown::HashMap<&str, Vec<&str>> = hashbrown::HashMap::new();

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

/// Original function using hashbrown
pub fn generate_rule_from_data_vec_hashbrown(
    content: &str,
    key_size: usize,
) -> Result<hashbrown::HashMap<Vec<&str>, Vec<&str>>, Error> {
    if key_size < 1 {
        return Err(Error::InvalidKeySize);
    }

    let words: Vec<&str> = content.split_whitespace().collect();

    let mut dict: hashbrown::HashMap<Vec<&str>, Vec<&str>> = hashbrown::HashMap::new();

    for slice in words.windows(key_size + 1) {
        let (key, value) = slice.split_at(key_size);
        let value = value[0];
        dict.entry(key.to_vec()).or_default().push(value);
    }

    Ok(dict)
}
