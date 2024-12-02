use itertools::Itertools;

pub type Report = Vec<i32>;

pub fn is_report_safe(report: &Report) -> bool {
    // is in either ascending or descending order
    let mut report_copy = report.clone();
    let mut report_copy_reverse = report.clone();
    report_copy.sort();
    report_copy_reverse.sort();
    report_copy_reverse.reverse();

    if report != &report_copy && report != &report_copy_reverse {
        return false;
    }

    // Difference in jumps has to be in 1..=3 range
    if report.iter().tuple_windows().any(|(prev, next)| -> bool {
        let diff = prev.abs_diff(*next);

        return !(diff.ge(&1) && diff.le(&3));
    }) {
        return false;
    }

    return true;
}