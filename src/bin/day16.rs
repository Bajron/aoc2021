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

        let packets = read_from_iter(bits.chars());

        for p in &packets {
            println!("Packet of type {}", p.kind);
        }

        let sv: usize = packets.iter().map(|p| p.sum_versions()).sum();
        println!("Versions sum: {sv}");
    }
}

fn read_from_iter<T>(mut it: T) -> Vec<Packet>
where
    T: Iterator<Item = char>,
{
    let mut packets: Vec<Packet> = vec![];
    let itr = &mut it;
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
            let number = u32::from_str_radix(&data_bits.as_str(), 2).unwrap();
            Packet {
                version: version,
                kind: kind,
                contents: PacketContent::Number(number),
            }
        }
        _ => {
            if let Some(len_type) = itr.next() {
                if len_type == '0' {
                    let len_bits = itr.take(15).collect::<String>();
                    if len_bits.len() != 15 {
                        return None;
                    }
                    let inside_bits = usize::from_str_radix(len_bits.as_str(), 2).unwrap();
                    Packet {
                        version: version,
                        kind: kind,
                        contents: PacketContent::Packets(read_from_iter(itr.take(inside_bits))),
                    }
                } else {
                    let len_bits = itr.take(11).collect::<String>();
                    if len_bits.len() != 11 {
                        return None;
                    }
                    let inside_packets = u32::from_str_radix(len_bits.as_str(), 2).unwrap();
                    Packet {
                        version: version,
                        kind: kind,
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

struct Packet {
    version: u32,
    kind: u32,
    contents: PacketContent,
}

enum PacketContent {
    Number(u32),
    Packets(Vec<Packet>),
}

impl Packet {
    fn sum_versions(&self) -> usize {
        self.version as usize
            + match &self.contents {
                PacketContent::Packets(p) => p.iter().map(|p| p.version as usize).sum(),
                _ => 0,
            }
    }
}
