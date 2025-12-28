use std::str::FromStr;

pub fn parse_array<T, const N: usize, F>(
    text: &str,
    len_err: F,
    parse_err: F,
) -> anyhow::Result<[T; N]>
where
    T: FromStr + Default + Copy,
    F: Fn() -> anyhow::Error,
{
    let mut array = [T::default(); N];

    let mut text_split = text.split_whitespace();

    for i in 0..N {
        let str = text_split.next().ok_or_else(|| len_err())?;

        array[i] = str.parse::<T>().map_err(|_| parse_err())?;
    }

    match text_split.next() {
        Some(_) => return Err(len_err()),
        None => {}
    };

    Ok(array)
}
