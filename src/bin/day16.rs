use core::panic;

use aoc2021::input::*;

fn main() {
    let input = read_lines()
        .ok()
        .and_then(|lines| lines.into_iter().find_map(|l| l.ok()));

    if let Some(hex) = input {
        let bits = hex
            .chars()
            .flat_map(|c| {
                match c {
                    '0' => "0000",
                    '1' => "0001",
                    '2' => "0010",
                    '3' => "0011",
                    '4' => "0100",
                    '5' => "0101",
                    '6' => "0110",
                    '7' => "0111",
                    '8' => "1000",
                    '9' => "1001",
                    'A' => "1010",
                    'B' => "1011",
                    'C' => "1100",
                    'D' => "1101",
                    'E' => "1110",
                    'F' => "1111",
                    _ => "",
                }
                .chars()
            })
            .collect::<String>();

        let packets = read_from_iter(bits.chars().by_ref());

        for p in &packets {
            println!("Packet of type {}", p.kind);
            match &p.contents {
                PacketContent::Number(n) => println!("Contains number {}", n),
                PacketContent::Packets(p) => println!("Contains {} subpackets", p.len()),
            }
        }

        println!("{packets:?}");

        let sv: usize = packets.iter().map(|p| p.sum_versions()).sum();
        println!("Versions sum: {sv}");
        if let Some(first_packet) = packets.first() {
            println!(
                "Evaluation of the first packet is {}",
                first_packet.evaluate()
            );
        }
    }
}

fn read_from_iter<T>(itr: &mut T) -> Vec<Packet>
where
    T: Iterator<Item = char>,
{
    let mut packets: Vec<Packet> = vec![];
    loop {
        if let Some(packet) = extract_packet(itr) {
            packets.push(packet);
        } else {
            break;
        }
    }
    packets
}

fn read_n_from_iter<T>(itr: &mut T, n: u32) -> Vec<Packet>
where
    T: Iterator<Item = char>,
{
    let mut packets: Vec<Packet> = vec![];
    for _ in 0..n {
        if let Some(packet) = extract_packet(itr) {
            packets.push(packet);
        } else {
            break;
        }
    }
    packets
}

fn extract_packet<T>(itr: &mut T) -> Option<Packet>
where
    T: Iterator<Item = char>,
{
    let version_bits = itr.take(3).collect::<String>();
    if version_bits.len() < 3 {
        return None;
    }
    let version = u32::from_str_radix(version_bits.as_str(), 2).unwrap();

    let kind_bits = itr.take(3).collect::<String>();
    if kind_bits.len() < 3 {
        return None;
    }
    let kind = u32::from_str_radix(kind_bits.as_str(), 2).unwrap();

    let packet = match kind {
        4 => {
            let mut data_bits = String::new();
            loop {
                if let Some(control) = itr.next() {
                    data_bits.extend(itr.take(4));
                    if control == '0' {
                        break;
                    }
                } else {
                    return None;
                }
            }
            if data_bits.len() % 4 != 0 {
                return None;
            }
            let number = u128::from_str_radix(&data_bits.as_str(), 2).unwrap();
            Packet {
                version: version,
                kind: kind,
                contents: PacketContent::Number(number),
            }
        }
        _ => {
            if let Some(len_type) = itr.next() {
                if len_type == '0' {
                    //return None;
                    let len_bits = itr.take(15).collect::<String>();
                    if len_bits.len() != 15 {
                        return None;
                    }
                    let inside_bits = usize::from_str_radix(len_bits.as_str(), 2).unwrap();
                    // A dream to pass `itr.take(inside_bits)` as the argument failed with some trait recursion blowing up...
                    let sub_bits = itr.take(inside_bits).collect::<String>();
                    Packet {
                        version,
                        kind,
                        contents: PacketContent::Packets(read_from_iter(sub_bits.chars().by_ref())),
                        // contents: PacketContent::Packets(read_from_iter(itr.take(inside_bits).by_ref())),
                    }
                } else {
                    let len_bits = itr.take(11).collect::<String>();
                    if len_bits.len() != 11 {
                        return None;
                    }
                    let inside_packets = u32::from_str_radix(len_bits.as_str(), 2).unwrap();
                    Packet {
                        version,
                        kind,
                        contents: PacketContent::Packets(read_n_from_iter(itr, inside_packets)),
                    }
                }
            } else {
                return None;
            }
        }
    };
    Some(packet)
}

#[derive(Debug)]
struct Packet {
    version: u32,
    kind: u32,
    contents: PacketContent,
}

#[derive(Debug)]
enum PacketContent {
    Number(u128),
    Packets(Vec<Packet>),
}

impl Packet {
    fn sum_versions(&self) -> usize {
        self.version as usize
            + match &self.contents {
                PacketContent::Packets(p) => p.iter().map(|p| p.sum_versions() as usize).sum(),
                _ => 0,
            }
    }

    fn evaluate(&self) -> u128 {
        if self.kind == 4 {
            if let PacketContent::Number(v) = self.contents {
                return v;
            } else {
                panic!("Malformed value packet");
            }
        } else if let PacketContent::Packets(p) = &self.contents {
            match self.kind {
                0 => p.iter().map(|p| p.evaluate()).sum(),
                1 => p.iter().map(|p| p.evaluate()).product(),
                2 => p.iter().map(|p| p.evaluate()).min().unwrap(),
                3 => p.iter().map(|p| p.evaluate()).max().unwrap(),
                5 => match &p[0..2] {
                    [lhs, rhs] => {
                        if lhs.evaluate() > rhs.evaluate() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Malformed gt packet"),
                },
                6 => match &p[0..2] {
                    [lhs, rhs] => {
                        if lhs.evaluate() < rhs.evaluate() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Malformed lt packet"),
                },
                7 => match &p[0..2] {
                    [lhs, rhs] => {
                        if lhs.evaluate() == rhs.evaluate() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Malformed eq packet"),
                },
                _ => panic!("Invalid packet type in the system {}", self.kind),
            }
        } else {
            panic!("Malformed packet")
        }
    }
}
