use std::slice::Iter;

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Op {
    fn from_int(n: u32) -> Op {
        match n {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Minimum,
            3 => Op::Maximum,
            4 => Op::Literal,
            5 => Op::GreaterThan,
            6 => Op::LessThan,
            7 => Op::EqualTo,
            _ => panic!()
        }
    }
}


#[derive(Debug)]
struct Packet {
    version: u32,
    op: Op,
    bit_length: u32,
    value: Option<u32>,
    // For literals
    sub_packets: Vec<Packet>,  // For operators
}

fn get_num(stream: &mut Iter<u8>, num_bits: usize) -> u32 {
    let mut value = 0u32;
    for _ in 0..num_bits {
        value = (value << 1) + *stream.next().unwrap() as u32;
    }
    value
}

impl Packet {
    fn version_total(&self) -> u32 {
        let mut total = self.version;
        for sp in self.sub_packets.iter() {
            total += sp.version_total();
        }
        total
    }

    fn get_value(&self) -> u128 {
        use Op::*;
        match self.op {
            Sum => {
                self.sub_packets.iter().map(|packet| packet.get_value()).sum()
            }
            Product => {
                self.sub_packets.iter().map(|packet| packet.get_value()).product()
            }
            Minimum => {
                self.sub_packets.iter().map(|packet| packet.get_value()).min().unwrap()
            }
            Maximum => {
                self.sub_packets.iter().map(|packet| packet.get_value()).max().unwrap()
            }
            Literal => {
                self.value.unwrap() as u128
            }
            GreaterThan => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets.first().unwrap().get_value() > self.sub_packets.last().unwrap().get_value() { 1 } else { 0 }
            }
            LessThan => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets.first().unwrap().get_value() < self.sub_packets.last().unwrap().get_value() { 1 } else { 0 }
            }
            EqualTo => {
                assert_eq!(self.sub_packets.len(), 2);
                if self.sub_packets.first().unwrap().get_value() == self.sub_packets.last().unwrap().get_value() { 1 } else { 0 }
            }
        }
    }

    fn from_bitstream(bit_stream: &mut Iter<u8>) -> Packet {
        let version = get_num(bit_stream, 3);
        let op = Op::from_int(get_num(bit_stream, 3));
        let mut bit_length = 6;
        match op {
            Op::Literal => {
                let mut done = false;
                let mut value = 0u32;
                while !done {
                    done = get_num(bit_stream, 1) == 0;
                    let nibble = get_num(bit_stream, 4);
                    bit_length += 5;
                    value <<= 4;
                    value += nibble;
                }
                Packet { version, op, value: Some(value), bit_length, sub_packets: vec![] }
            }
            _ => {
                bit_length += 1;
                match get_num(bit_stream, 1) {
                    0 => {
                        let sub_packet_bit_length = get_num(bit_stream, 15);
                        bit_length += 15 + sub_packet_bit_length;
                        let mut sub_bits_read = 0;
                        let mut sub_packets = vec![];
                        while sub_bits_read < sub_packet_bit_length {
                            let sub_packet = Packet::from_bitstream(bit_stream);
                            sub_bits_read += sub_packet.bit_length;
                            sub_packets.push(sub_packet);
                        }
                        Packet { version, op, bit_length, sub_packets, value: None }
                    }
                    1 => {
                        let num_sub_packets = get_num(bit_stream, 11);
                        bit_length += 11;
                        let mut sub_packets = vec![];
                        for _ in 0..num_sub_packets {
                            let sub_packet = Packet::from_bitstream(bit_stream);
                            bit_length += sub_packet.bit_length;
                            sub_packets.push(sub_packet);
                        }
                        Packet { version, op, bit_length, sub_packets, value: None }
                    }
                    _ => panic!("That is not a bit")
                }
            }
        }
    }
}

fn string_to_bit_stream(s: String) -> Vec<u8> {
    s.trim().chars().map(|c| match c {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => panic!("Not hex: {}", c)
    }).flatten().collect::<Vec<u8>>()
}

pub fn part1(input: String) {
    let bits = string_to_bit_stream(input);
    let packet = Packet::from_bitstream(&mut bits.iter());
    println!("{}", packet.version_total());
}

pub fn part2(input: String) {
    let bits = string_to_bit_stream(input);
    let packet = Packet::from_bitstream(&mut bits.iter());
    println!("{}", packet.get_value());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{Packet, string_to_bit_stream};

    #[test]
    fn test_part1() {
        let cases = HashMap::from([
            (String::from("8A004A801A8002F478"), 16),
            (String::from("620080001611562C8802118E34"), 12),
            (String::from("C0015000016115A2E0802F182340"), 23),
            (String::from("A0016C880162017C3686B18A3D4780"), 31),
        ]);

        for (data, expected) in cases {
            let bits = string_to_bit_stream(data);
            let packet = Packet::from_bitstream(&mut bits.iter());
            assert_eq!(packet.version_total(), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = HashMap::from([
            (String::from("C200B40A82"), 3),
            (String::from("04005AC33890"), 54),
            (String::from("880086C3E88112"), 7),
            (String::from("CE00C43D881120"), 9),
            (String::from("D8005AC2A8F0"), 1),
            (String::from("F600BC2D8F"), 0),
            (String::from("9C005AC2F8F0"), 0),
            (String::from("9C0141080250320F1802104A08"), 1),
        ]);


        for (data, expected) in cases {
            let bits = string_to_bit_stream(data.clone());
            let packet = Packet::from_bitstream(&mut bits.iter());
            assert_eq!(packet.get_value(), expected);
        }
    }
}