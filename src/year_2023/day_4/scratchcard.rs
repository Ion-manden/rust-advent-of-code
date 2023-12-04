#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Scratchcard {
    pub id: i32,
    pub winning_numbers: Vec<i32>,
    pub card_numbers: Vec<i32>,
}

impl From<&str> for Scratchcard {
    fn from(line: &str) -> Self {
        let (card_id_part, number_part) = line.split_once(": ").unwrap();
        let id: i32 = card_id_part.split_once(" ").unwrap().1.trim().parse().unwrap();

        let (winning_numbers_part, card_numbers_pard) = number_part.split_once(" | ").unwrap();

        Self {
            id,
            winning_numbers: winning_numbers_part
                .split(" ")
                .filter(|part| part.ne(&""))
                .map(|part| part.parse::<i32>().unwrap())
                .collect(),
            card_numbers: card_numbers_pard
                .split(" ")
                .filter(|part| part.ne(&""))
                .map(|part| part.parse::<i32>().unwrap())
                .collect(),
        }
    }
}

impl Scratchcard {
    pub fn get_matching_numbers(self: &Self) -> Vec<&i32> {
        self.card_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Scratchcard;

    #[test]
    fn test_scratchcard_from_str() {
        assert_eq!(
            Scratchcard::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            Scratchcard {
                id: 4,
                winning_numbers: vec![41, 92, 73, 84, 69],
                card_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83]
            }
        );
    }

    #[test]
    fn test_scratchcard_from_long_str() {
        assert_eq!(
            Scratchcard::from("Card  15: 40 99  7 94 75 66 24 71 17 33 | 62 87 68 82 79 54 95 69 26  7 20 18 64 84 63 52 53 35 50 86 34  9 14 27 73"),
            Scratchcard {
                id: 15,
                winning_numbers: vec![40, 99 , 7, 94, 75, 66, 24, 71, 17, 33],
                card_numbers: vec![62, 87, 68, 82, 79, 54, 95, 69, 26,  7, 20, 18, 64, 84, 63, 52, 53, 35, 50, 86, 34,9, 14, 27, 73]
            }
        );
    }
}
