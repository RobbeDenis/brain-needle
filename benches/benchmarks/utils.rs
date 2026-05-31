
#[allow(unused)]
#[inline]
#[must_use]
pub fn multiply_contents_into_bytes<P: AsRef<std::path::Path>>(path: P, n: usize) -> Vec<u8> {
    let contents = std::fs::read_to_string(path).unwrap();
    return contents.repeat(n).into_bytes();
}