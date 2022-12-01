#[derive(Debug, Clone)]
pub enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl Operator {
    pub fn from_i64(n: i64) -> Self {
        match n {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Minimum,
            3 => Operator::Maximum,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::Equal,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
enum PacketContents {
    Operator {
        operator: Operator,
        sub_packets: Vec<Packet>,
    },
    Data(i64),
}

#[derive(Debug, Clone)]
pub struct Packet {
    type_id: i64,
    version: i64,
    contents: PacketContents,
}

impl Packet {
    pub fn version_numbers(&self) -> Vec<i64> {
        let mut versions = vec![self.version];
        if let PacketContents::Operator { sub_packets, .. } = &self.contents {
            let sub_versions = sub_packets
                .iter()
                .flat_map(|packet| packet.version_numbers())
                .collect::<Vec<_>>();
            versions.extend(sub_versions);
        }
        versions
    }
    pub fn evaluate(&self) -> i64 {
        match &self.contents {
            PacketContents::Data(n) => *n,
            PacketContents::Operator {
                operator,
                sub_packets,
            } => match operator {
                Operator::Sum => sub_packets.iter().map(|packet| packet.evaluate()).sum(),
                Operator::Product => sub_packets.iter().map(|packet| packet.evaluate()).product(),
                Operator::Minimum => sub_packets
                    .iter()
                    .map(|packet| packet.evaluate())
                    .min()
                    .unwrap(),
                Operator::Maximum => sub_packets
                    .iter()
                    .map(|packet| packet.evaluate())
                    .max()
                    .unwrap(),
                Operator::GreaterThan => {
                    if sub_packets[0].evaluate() > sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                Operator::LessThan => {
                    if sub_packets[0].evaluate() < sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                Operator::Equal => {
                    if sub_packets[0].evaluate() == sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

fn bit_char_arr_to_num(code: &[char]) -> i64 {
    let base: i64 = 2;
    code.iter().rev().enumerate().fold(0, |acc, (index, char)| {
        acc + (i64::from_str_radix(&char.to_string(), 2).unwrap()) * base.pow(index as u32)
    })
}

#[derive(Debug)]
pub struct Computer {
    code: Vec<char>,
    index: usize,
}

impl Computer {
    pub fn load_input(input: &str) -> Self {
        let mut code = vec![];
        for line in input.lines() {
            for char in line.chars() {
                code.extend(
                    format!(
                        "{:04b}",
                        i64::from_str_radix(&char.to_string(), 16).unwrap()
                    )
                    .chars(),
                );
            }
        }
        Computer { code, index: 0 }
    }
    pub fn parse_packet(&mut self) -> Packet {
        let version = self.parse_version();
        let type_id = self.parse_type_id();
        match type_id {
            4 => {
                let lit = self.parse_literal();
                Packet {
                    type_id,
                    version,
                    contents: PacketContents::Data(lit),
                }
            }
            _ => {
                let sub_packets = self.parse_operator();
                Packet {
                    type_id,
                    version,
                    contents: PacketContents::Operator {
                        operator: Operator::from_i64(type_id),
                        sub_packets,
                    },
                }
            }
        }
    }
    fn parse_literal(&mut self) -> i64 {
        let mut num = vec![];
        loop {
            let prefix = self.read_bits(1);
            num.extend(
                self.code[self.index..self.index + 4]
                    .iter()
                    .map(|c| c.to_owned()) // LOL
                    .collect::<Vec<char>>(),
            );
            self.index += 4;
            if prefix == 0 {
                break;
            }
        }
        bit_char_arr_to_num(&num)
    }
    fn parse_operator(&mut self) -> Vec<Packet> {
        let length_type_id = self.read_bits(1);
        let mut length = None;
        let mut num_packets = None;
        match length_type_id {
            0 => {
                length = Some(self.read_bits(15));
            }
            1 => {
                num_packets = Some(self.read_bits(11));
            }
            _ => unreachable!(),
        }
        if let Some(length) = length {
            let start = self.index;
            let mut packets = vec![];
            while self.index < start + length as usize {
                packets.push(self.parse_packet());
            }
            packets
        } else if let Some(num_packets) = num_packets {
            let mut num_packets = num_packets;
            let mut packets = vec![];
            while num_packets > 0 {
                packets.push(self.parse_packet());
                num_packets -= 1;
            }
            packets
        } else {
            unreachable!();
        }
    }

    fn read_bits(&mut self, bits: usize) -> i64 {
        let val = bit_char_arr_to_num(&self.code[self.index..self.index + bits]);
        self.index += bits;
        val
    }
    fn parse_version(&mut self) -> i64 {
        self.read_bits(3)
    }
    fn parse_type_id(&mut self) -> i64 {
        self.read_bits(3)
    }
}
