use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
    .inspect(|capture|println!("{:?}", capture))
        .map(|capture| {
            (
                capture.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .map(|(x, y)| x * y)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = solve(input);

        assert_eq!(result, 161);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 166357705);
    }
}
