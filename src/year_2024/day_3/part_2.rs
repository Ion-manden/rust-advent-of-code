use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();

    let captures: Vec<regex::Captures> = re.captures_iter(input).collect();

    let mut result: i32 = 0;

    let mut disabled = false;
    for capture in captures {
        let capture_str = capture.get(0).unwrap().as_str();
        if capture_str.eq("don't()") {
            disabled = true;
            continue;
        }

        if capture_str.eq("do()") {
            disabled = false;
            continue;
        }

        if disabled {
            continue;
        }

        let x = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y =capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        result += x * y;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = solve(input);

        assert_eq!(result, 48);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 88811886);
    }
}
