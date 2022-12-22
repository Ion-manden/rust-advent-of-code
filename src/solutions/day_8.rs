pub mod day_8 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::reader::reader::create_reader_from_file;

    struct Tree {
        x_index: usize,
        y_index: usize,
        height: usize,
    }

    fn is_hidden_from_view(tree: &Tree, forest: &Vec<Tree>) -> bool {
        let relevant_trees: Vec<&Tree> = forest
            .into_iter()
            .filter(|t| t.x_index == tree.x_index || t.y_index == tree.y_index)
            .collect();

        let is_hidden_from_top = relevant_trees
            .iter()
            .filter(|t| t.y_index < tree.y_index)
            .filter(|t| t.height >= tree.height)
            .count()
            > 0;
        let is_hidden_from_bottom = relevant_trees
            .iter()
            .filter(|t| t.y_index > tree.y_index)
            .filter(|t| t.height >= tree.height)
            .count()
            > 0;
        let is_hidden_from_left = relevant_trees
            .iter()
            .filter(|t| t.x_index < tree.x_index)
            .filter(|t| t.height >= tree.height)
            .count()
            > 0;
        let is_hidden_from_right = relevant_trees
            .iter()
            .filter(|t| t.x_index > tree.x_index)
            .filter(|t| t.height >= tree.height)
            .count()
            > 0;

        is_hidden_from_top && is_hidden_from_bottom && is_hidden_from_left && is_hidden_from_right
    }

    fn get_view_score(tree: &Tree, forest: &Vec<Tree>) -> usize {
        let max_x = forest.iter().map(|t| t.x_index).max().unwrap();
        let max_y = forest.iter().map(|t| t.y_index).max().unwrap();
        if tree.x_index == 0 || tree.x_index == max_x || tree.y_index == 0 || tree.y_index == max_y
        {
            return 0;
        }

        let relevant_trees: Vec<&Tree> = forest
            .into_iter()
            .filter(|t| t.x_index == tree.x_index || t.y_index == tree.y_index)
            // Remove itself
            .filter(|t| {!(t.x_index == tree.x_index && t.y_index == tree.y_index)})
            .collect();

        let mut viewable_top = 0;
        for y in (0..tree.y_index).rev() {
            let rth = relevant_trees
                .iter()
                .find(|t| t.y_index == y.to_owned())
                .unwrap()
                .height;

            if rth >= tree.height {
                viewable_top += 1;
                break;
            }
            viewable_top += 1;
        }

        let mut viewable_bottom = 0;
        for y in tree.y_index+1..=max_y {
            let rth = relevant_trees
                .iter()
                .find(|t| t.y_index == y.to_owned())
                .unwrap()
                .height;

            if rth >= tree.height {
                viewable_bottom += 1;
                break;
            }
            viewable_bottom += 1;
        }

        let mut viewable_left = 0;
        for x in (0..tree.x_index).rev() {
            let rth = relevant_trees
                .iter()
                .find(|t| t.x_index == x.to_owned())
                .unwrap()
                .height;
            if rth >= tree.height {
                viewable_left += 1;
                break;
            }
            viewable_left += 1;
        }

        let mut viewable_right = 0;
        for x in tree.x_index+1..=max_x {
            let rth = relevant_trees
                .iter()
                .find(|t| t.x_index == x.to_owned())
                .unwrap()
                .height;

            if rth >= tree.height {
                viewable_right += 1;
                break;
            }
            viewable_right += 1;
        }

        viewable_top * viewable_bottom * viewable_left * viewable_right
    }

    pub fn day_8_part_1(reader: BufReader<File>) -> usize {
        let forest: Vec<Tree> = reader
            .lines()
            .map(|l| l.unwrap())
            .enumerate()
            .flat_map(|(y, forest_line)| -> Vec<Tree> {
                forest_line
                    .chars()
                    .enumerate()
                    .map(|(x, height)| Tree {
                        x_index: x,
                        y_index: y,
                        height: height.to_string().parse::<usize>().unwrap(),
                    })
                    .collect::<Vec<Tree>>()
            })
            .collect::<Vec<Tree>>();

        forest
            .iter()
            .filter(|t| !is_hidden_from_view(t, &forest))
            .count()
    }

    #[test]
    fn test_day_8_part_1_example() {
        let expect = 21;
        let got = day_8_part_1(create_reader_from_file(
            "./src/solutions/day_8/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_8_part_1_input() {
        let expect = 1825;
        let got = day_8_part_1(create_reader_from_file("./src/solutions/day_8/input.txt".to_owned()));
        assert_eq!(got, expect);
    }

    pub fn day_8_part_2(reader: BufReader<File>) -> usize {
        let forest: Vec<Tree> = reader
            .lines()
            .map(|l| l.unwrap())
            .enumerate()
            .flat_map(|(y, forest_line)| -> Vec<Tree> {
                forest_line
                    .chars()
                    .enumerate()
                    .map(|(x, height)| Tree {
                        x_index: x,
                        y_index: y,
                        height: height.to_string().parse::<usize>().unwrap(),
                    })
                    .collect::<Vec<Tree>>()
            })
            .collect::<Vec<Tree>>();

        forest
            .iter()
            .map(|t| get_view_score(t, &forest))
            .max()
            .unwrap()
    }

    #[test]
    fn test_day_8_part_2_example() {
        let expect = 8;

        let got = day_8_part_2(create_reader_from_file(
            "./src/solutions/day_8/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_8_part_2_input() {
        let expect = 235200;

        let got = day_8_part_2(create_reader_from_file("./src/solutions/day_8/input.txt".to_owned()));

        assert_eq!(got, expect);
    }
}
