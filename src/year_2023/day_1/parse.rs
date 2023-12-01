use std::iter;

pub fn get_first_and_last_int(line: &str, accept_word_numbers: bool) -> (i32, i32) {
    let mut numbers: Vec<i32> = vec![];

    let line_len = line.len();
    for i in 0..line_len {
        let part = &line[i..];
        let char = part
            .clone()
            .chars()
            .nth(0)
            .unwrap()
            .to_string()
            .parse::<i32>();

        if char.is_ok() {
            numbers.push(char.unwrap());
            continue;
        }

        if !accept_word_numbers {
            continue;
        }

        if part.starts_with("one") {
            numbers.push(1);
            continue;
        };
        if part.starts_with("two") {
            numbers.push(2);
            continue;
        };
        if part.starts_with("three") {
            numbers.push(3);
            continue;
        };
        if part.starts_with("four") {
            numbers.push(4);
            continue;
        };
        if part.starts_with("five") {
            numbers.push(5);
            continue;
        };
        if part.starts_with("six") {
            numbers.push(6);
            continue;
        };
        if part.starts_with("seven") {
            numbers.push(7);
            continue;
        };
        if part.starts_with("eight") {
            numbers.push(8);
            continue;
        };
        if part.starts_with("nine") {
            numbers.push(9);
            continue;
        };
    }

    let first = numbers.first().unwrap().clone();
    let last = numbers.last().unwrap().clone();

    (first, last)
}

#[cfg(test)]
mod tests {
    use super::get_first_and_last_int;

    #[test]
    fn test_get_first_and_last_int() {
        assert_eq!(get_first_and_last_int("a1b2c3d4e5f", false), (1, 5));
        assert_eq!(get_first_and_last_int("treb7uchet", false), (7, 7));
        assert_eq!(get_first_and_last_int("eightwothree", true), (8, 3));
        assert_eq!(get_first_and_last_int("abcone2threexyz", true), (1, 3));
    }
}
