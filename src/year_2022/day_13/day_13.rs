use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::reader::reader::create_reader_from_file;

type Packet = Vec<PacketValue>;
impl TryFrom<PacketValue> for Packet {
    type Error = String;

    fn try_from(value: PacketValue) -> Result<Self, Self::Error> {
        match value {
            PacketValue::List(list) => Ok(list as Self),
            PacketValue::Val(val) => Err(format!("Unable to convert value {} to Packet", val)),
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PacketValue {
    Val(u8),
    List(Vec<PacketValue>),
}

impl From<String> for PacketValue {
    fn from(mut line: String) -> Self {
        if line == "[]" {
            return Self::List(vec![]);
        }

        if let Ok(val) = line.parse::<u8>() {
            return Self::Val(val);
        }

        let left_most_char = line.remove(0);
        let right_most_char = line.remove(line.len() - 1);
        if left_most_char != '[' || right_most_char != ']' {
            panic!("Invalid string sent to string_to_packet")
        }

        if line.len() == 1 {
            let val = line.parse::<u8>().unwrap();
            return Self::List(vec![Self::Val(val)]);
        }

        let mut packet = vec![];
        let mut buffer = String::from("");
        let mut indent = 0;
        for c in line.chars() {
            if c == '[' {
                indent += 1;
            }
            if c == ']' {
                indent -= 1;
            }
            if c == ',' && indent == 0 {
                let sub_packet = Self::from(buffer);

                packet.push(sub_packet);

                buffer = String::from("");
                indent = 0;
                continue;
            }

            buffer.push(c);
        }

        if buffer.len() > 0 {
            let sub_packet = Self::from(buffer);

            packet.push(sub_packet);
        }

        return Self::List(packet);
    }
}

#[test]
fn test_packet_from_string() {
    {
        let expect: PacketValue = PacketValue::List(vec![
            PacketValue::Val(1),
            PacketValue::Val(1),
            PacketValue::Val(3),
            PacketValue::Val(1),
            PacketValue::Val(1),
        ]);
        let got = PacketValue::from(String::from("[1,1,3,1,1]"));
        assert_eq!(got, expect);
    }
    {
        let expect: PacketValue = PacketValue::List(vec![
            PacketValue::Val(1),
            PacketValue::List(vec![
                PacketValue::Val(2),
                PacketValue::List(vec![
                    PacketValue::Val(3),
                    PacketValue::List(vec![
                        PacketValue::Val(4),
                        PacketValue::List(vec![
                            PacketValue::Val(5),
                            PacketValue::Val(6),
                            PacketValue::Val(7),
                        ]),
                    ]),
                ]),
            ]),
            PacketValue::Val(8),
            PacketValue::Val(9),
        ]);
        let got = PacketValue::from(String::from("[1,[2,[3,[4,[5,6,7]]]],8,9]"));
        assert_eq!(got, expect);
    }
    {
        let expect: PacketValue = PacketValue::List(vec![
            PacketValue::List(vec![
                PacketValue::List(vec![
                    PacketValue::Val(3),
                    PacketValue::List(vec![
                        PacketValue::Val(1),
                        PacketValue::Val(5),
                        PacketValue::Val(10),
                        PacketValue::Val(1),
                    ]),
                    PacketValue::List(vec![
                        PacketValue::Val(7),
                        PacketValue::Val(0),
                        PacketValue::Val(9),
                        PacketValue::Val(4),
                    ]),
                    PacketValue::Val(2),
                    PacketValue::Val(4),
                ]),
                PacketValue::List(vec![PacketValue::Val(0)]),
                PacketValue::Val(8),
                PacketValue::List(vec![
                    PacketValue::Val(4),
                    PacketValue::Val(1),
                    PacketValue::List(vec![]),
                ]),
            ]),
            PacketValue::List(vec![
                PacketValue::Val(7),
                PacketValue::List(vec![
                    PacketValue::List(vec![
                        PacketValue::Val(8),
                        PacketValue::Val(9),
                        PacketValue::Val(6),
                        PacketValue::Val(9),
                    ]),
                    PacketValue::List(vec![PacketValue::Val(9), PacketValue::Val(0)]),
                    PacketValue::List(vec![
                        PacketValue::Val(5),
                        PacketValue::Val(3),
                        PacketValue::Val(9),
                    ]),
                    PacketValue::List(vec![PacketValue::Val(0), PacketValue::Val(8)]),
                ]),
                PacketValue::Val(2),
                PacketValue::List(vec![
                    PacketValue::Val(9),
                    PacketValue::Val(6),
                    PacketValue::List(vec![
                        PacketValue::Val(2),
                        PacketValue::Val(4),
                        PacketValue::Val(1),
                    ]),
                ]),
                PacketValue::List(vec![
                    PacketValue::List(vec![
                        PacketValue::Val(8),
                        PacketValue::Val(8),
                        PacketValue::Val(3),
                    ]),
                    PacketValue::Val(4),
                    PacketValue::Val(9),
                ]),
            ]),
        ]);
        let got = PacketValue::from(String::from("[[[3,[1,5,10,1],[7,0,9,4],2,4],[0],8,[4,1,[]]],[7,[[8,9,6,9],[9,0],[5,3,9],[0,8]],2,[9,6,[2,4,1]],[[8,8,3],4,9]]]"));
        assert_eq!(got, expect);
    }
}

fn is_in_order(left: &Packet, right: &Packet) -> Option<bool> {
    for i in 0..left.len().max(right.len()) {
        let left_part = left.get(i);
        let right_part = right.get(i);
        let in_order: Option<bool> = match (left_part, right_part) {
            (None, None) => None,
            (None, Some(_)) => Some(true),
            (Some(_), None) => Some(false),
            (Some(l), Some(r)) => match (l, r) {
                (PacketValue::Val(l), PacketValue::Val(r)) if l < r => Some(true),
                (PacketValue::Val(l), PacketValue::Val(r)) if l > r => Some(false),
                (PacketValue::Val(_), PacketValue::Val(_)) => None,
                (PacketValue::Val(l), PacketValue::List(r)) => {
                    is_in_order(&vec![PacketValue::Val(l.to_owned())], r)
                }
                (PacketValue::List(l), PacketValue::Val(r)) => {
                    is_in_order(l, &vec![PacketValue::Val(r.to_owned())])
                }
                (PacketValue::List(l), PacketValue::List(r)) => is_in_order(l, r),
            },
        };

        if in_order.is_some() {
            return in_order;
        }
    }
    None
}

#[test]
fn test_is_in_order() {
    {
        let expect = true;
        let got = is_in_order(
            &Packet::try_from(PacketValue::from(String::from("[1,1,3,1,1]"))).unwrap(),
            &Packet::try_from(PacketValue::from(String::from("[1,1,5,1,1]"))).unwrap(),
        );
        assert_eq!(got.unwrap(), expect);
    }
    {
        let expect = true;
        let got = is_in_order(
            &Packet::try_from(PacketValue::from(String::from("[[1],[2,3,4]]"))).unwrap(),
            &Packet::try_from(PacketValue::from(String::from("[[1],4]"))).unwrap(),
        );
        assert_eq!(got.unwrap(), expect);
    }
    {
        let expect = false;
        let got = is_in_order(
            &Packet::try_from(PacketValue::from(String::from(
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            )))
            .unwrap(),
            &Packet::try_from(PacketValue::from(String::from(
                "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            )))
            .unwrap(),
        );
        assert_eq!(got.unwrap(), expect);
    }
}

pub fn day_13_part_1(reader: BufReader<File>) -> usize {
    let mut lines = reader.lines();
    let mut i = 0;
    let mut result = 0;
    loop {
        i += 1;
        let left_packet =
            Packet::try_from(PacketValue::from(lines.next().unwrap().unwrap())).unwrap();
        let right_packet =
            Packet::try_from(PacketValue::from(lines.next().unwrap().unwrap())).unwrap();
        if is_in_order(&left_packet, &right_packet).unwrap() {
            result += i;
        }

        let empty_line = lines.next();
        if empty_line.is_none() {
            break;
        }
    }

    return result;
}

#[test]
fn test_day_13_part_1_example() {
    let expect = 13;
    let got = day_13_part_1(create_reader_from_file(
        "./src/solutions/day_13/example.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

#[test]
fn test_day_13_part_1_input() {
    let expect = 4821;
    let got = day_13_part_1(create_reader_from_file(
        "./src/solutions/day_13/input.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

pub fn day_13_part_2(reader: BufReader<File>) -> usize {
    let mut packets: Vec<Packet> = reader
        .lines()
        .into_iter()
        .filter(|l| l.is_ok())
        .map(|l| l.unwrap())
        .filter(|l| l != "")
        .map(|l| Packet::try_from(PacketValue::from(l)).unwrap())
        .collect();

    let first_divider_packet = Packet::try_from(PacketValue::from(String::from("[[2]]"))).unwrap();
    let second_divider_packet = Packet::try_from(PacketValue::from(String::from("[[6]]"))).unwrap();

    packets.push(first_divider_packet.clone());
    packets.push(second_divider_packet.clone());

    packets.sort_by(|l, r| match is_in_order(l, r).unwrap() {
        true => std::cmp::Ordering::Less,
        false => std::cmp::Ordering::Greater,
    });

    let first_pos = packets
        .iter()
        .position(|p| p.eq(&first_divider_packet))
        .unwrap()
        + 1;
    let second_pos = packets
        .iter()
        .position(|p| p.eq(&second_divider_packet))
        .unwrap()
        + 1;

    return first_pos * second_pos;
}

#[test]
fn test_day_13_part_2_example() {
    let expect = 140;

    let got = day_13_part_2(create_reader_from_file(
        "./src/solutions/day_13/example.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}

#[test]
fn test_day_13_part_2_input() {
    let expect = 21890;

    let got = day_13_part_2(create_reader_from_file(
        "./src/solutions/day_13/input.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}
