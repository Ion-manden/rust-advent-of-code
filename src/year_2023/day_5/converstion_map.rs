use std::collections::HashMap;

pub fn create_conversion_map_from_str(input: &str) -> HashMap<&str, Vec<(usize, usize, usize)>> {
    let mut parts = input.split("\n\n");
    // Skip seeds header
    let _seeds_part = parts.next();

    let mut conversion_map: HashMap<&str, Vec<(usize, usize, usize)>> = HashMap::new();

    for part in parts {
        let mut part_lines = part.lines();
        let header = part_lines.next().unwrap().split_once(" ").unwrap().0;

        let mut conversions: Vec<(usize, usize, usize)> = vec![];
        for line in part_lines {
            let mut parts = line.splitn(3, ' ');
            let (to, from, range): (usize, usize, usize) = (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );

            conversions.push((from, to, range));
        }

        conversion_map.insert(header, conversions);
    }

    conversion_map
}

pub fn create_conversions_vec_from_str(input: &str) -> Vec<Vec<(usize, usize, usize)>> {
    let mut parts = input.split("\n\n");
    // Skip seeds header
    let _seeds_part = parts.next();

    let mut conversion_vec: Vec<Vec<(usize, usize, usize)>> = vec![];

    for part in parts {
        let mut part_lines = part.lines();
        let _header = part_lines.next().unwrap().split_once(" ").unwrap().0;

        let mut conversions: Vec<(usize, usize, usize)> = vec![];
        for line in part_lines {
            let mut parts = line.splitn(3, ' ');
            let (to, from, range): (usize, usize, usize) = (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );

            conversions.push((from, to, range));
        }

        conversion_vec.push(conversions);
    }

    conversion_vec
}

#[cfg(test)]
mod tests {
    use super::create_conversion_map_from_str;

    #[test]
    fn test_conversion_map_from_string() {
        assert_eq!(
            create_conversion_map_from_str(include_str!("./example.txt"))
                .keys()
                .len(),
            7
        );

        let seed = 60;
        assert_eq!(
            create_conversion_map_from_str(include_str!("./example.txt"))
                .get("seed-to-soil")
                .unwrap()
                .into_iter()
                .find(|(from, _to, range)| &seed >= from && &seed < &(from + range))
                .map_or(seed, |(from, to, _range)| to + (&seed).abs_diff(from.to_owned())),
            62
        );
    }
}
