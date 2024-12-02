use super::report;

pub fn solve(input: &str) -> usize {
    let mut safe_report_count = 0;
    'line_loop: for line in input.lines() {
        let report: report::Report = line
            .split(" ")
            .map(|split: &str| split.parse().unwrap())
            .collect();

        if report::is_report_safe(&report) {
            safe_report_count += 1;
            continue;
        }

        for sub_report in sub_reports_from_report(&report) {
            if report::is_report_safe(&sub_report) {
                safe_report_count += 1;
                continue 'line_loop;
            }
        }
    }

    safe_report_count
}

fn sub_reports_from_report(report: &report::Report) -> Vec<report::Report> {
    let report_len = report.len();

    let mut sub_reports: Vec<report::Report> = vec![];

    for i in 0..report_len {
        let mut sub_report = report.clone();

        sub_report.remove(i);

        sub_reports.push(sub_report);
    }

    sub_reports
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 604);
    }
}
