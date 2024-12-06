pub struct PageOrderingRule {
    left: i32,
    right: i32,
}

impl From<&str> for PageOrderingRule {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once("|").unwrap();

        Self {
            left: left.parse().unwrap(),
            right: right.parse().unwrap(),
        }
    }
}

impl PageOrderingRule {
    pub fn is_update_valid(self: &Self, update: &SaftyManualUpdates) -> bool {
        // If either number is not in the rule the rule is passed
        if !update.page_numbers.contains(&self.left) || !update.page_numbers.contains(&self.right) {
            return true;
        }

        let left_pos = update
            .page_numbers
            .iter()
            .position(|page_nr| page_nr == &self.left)
            .unwrap();
        let right_pos = update
            .page_numbers
            .iter()
            .position(|page_nr| page_nr == &self.right)
            .unwrap();

        return left_pos < right_pos;
    }
}

#[derive(Clone)]
pub struct SaftyManualUpdates {
    pub page_numbers: Vec<i32>,
}

impl From<&str> for SaftyManualUpdates {
    fn from(value: &str) -> Self {
        Self {
            page_numbers: value
                .split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect(),
        }
    }
}

impl SaftyManualUpdates {
    pub fn is_valid_update(self: &Self, rules: &Vec<PageOrderingRule>) -> bool {
        rules.iter().all(|rule| rule.is_update_valid(self))
    }
}

pub fn get_middle_number(numbers: &Vec<i32>) -> i32 {
    let len = numbers.len();
    numbers.get(len / 2).unwrap().clone()
}

pub fn create_fixed_update(
    update: &SaftyManualUpdates,
    rules: &Vec<PageOrderingRule>,
) -> SaftyManualUpdates {
    let mut fixed_update = update.clone();
    loop {
        if fixed_update.is_valid_update(rules) {
            break;
        }

        for rule in rules {
            if !rule.is_update_valid(&fixed_update) {
                let left_pos = fixed_update
                    .page_numbers
                    .iter()
                    .position(|page_nr| page_nr == &rule.left)
                    .unwrap();
                let right_pos = fixed_update
                    .page_numbers
                    .iter()
                    .position(|page_nr| page_nr == &rule.right)
                    .unwrap();

                fixed_update.page_numbers.swap(left_pos, right_pos);
            }
        }
    }

    return fixed_update;
}
