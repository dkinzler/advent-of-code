use std::fs;

fn main() {
    let bits = read_input("inputs/16");
    let (packet, _) = parse_packet(&bits);
    println!("part1: {}", sum_packet_versions(&packet));
    println!("part2: {}", eval(&packet));
}

fn sum_packet_versions(p: &Packet) -> u64 {
    match p {
        Packet::Literal { version, .. } => *version as u64,
        Packet::Operator {
            version, packets, ..
        } => {
            let mut sum = *version as u64;
            for p in packets.iter() {
                sum += sum_packet_versions(p);
            }
            sum
        }
    }
}

fn eval(p: &Packet) -> u64 {
    match p {
        Packet::Literal { number, .. } => *number,
        Packet::Operator {
            type_id, packets, ..
        } => match *type_id {
            0 => packets.iter().fold(0, |a, x| a + eval(x)),
            1 => packets.iter().fold(1, |a, x| a * eval(x)),
            2 => packets.iter().map(|x| eval(x)).min().unwrap(),
            3 => packets.iter().map(|x| eval(x)).max().unwrap(),
            5 => {
                if eval(&packets[0]) > eval(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                if eval(&packets[0]) < eval(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                if eval(&packets[0]) == eval(&packets[1]) {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        },
    }
}

fn parse_packet(b: &[u8]) -> (Packet, usize) {
    let version = bits_to_num(&b[0..3]);
    let type_id = bits_to_num(&b[3..6]);
    if type_id == 4 {
        let mut i = 6;
        let mut v = 0u64;
        loop {
            v = (v << 4) + bits_to_num(&b[i + 1..i + 5]);
            if b[i] == 0 {
                break;
            }
            i += 5;
        }
        (
            Packet::Literal {
                version: version as u8,
                number: v,
            },
            i + 5,
        )
    } else {
        let mut sub_packets = Vec::new();
        let length_type = b[6];
        let mut i;
        if length_type == 0 {
            let total_length = bits_to_num(&b[7..7 + 15]) as usize;
            i = 7 + 15;
            let mut curr_length = 0;
            while curr_length < total_length {
                let (sp, l) = parse_packet(&b[i..]);
                curr_length += l;
                i += l;
                sub_packets.push(Box::new(sp));
            }
        } else {
            let n_subpackets = bits_to_num(&b[7..7 + 11]);
            i = 7 + 11;
            let mut curr_n = 0;
            while curr_n < n_subpackets {
                let (sp, l) = parse_packet(&b[i..]);
                i += l;
                curr_n += 1;
                sub_packets.push(Box::new(sp));
            }
        }
        (
            Packet::Operator {
                version: version as u8,
                type_id: type_id as u8,
                packets: sub_packets,
            },
            i,
        )
    }
}

fn bits_to_num(b: &[u8]) -> u64 {
    let mut r = 0;
    for i in 0..b.len() {
        r |= (b[b.len() - 1 - i] as u64) << i;
    }
    r
}

enum Packet {
    Literal {
        version: u8,
        number: u64,
    },
    Operator {
        version: u8,
        type_id: u8,
        packets: Vec<Box<Packet>>,
    },
}

fn read_input(file: &str) -> Vec<u8> {
    fs::read_to_string(file)
        .expect("failed to read input file")
        .trim()
        .as_bytes()
        .iter()
        .flat_map(|x| {
            let v = match *x {
                b'0'..=b'9' => *x - b'0',
                b'A'..=b'F' => *x - b'A' + 10,
                _ => panic!(),
            };
            [(v & 8) >> 3, (v & 4) >> 2, (v & 2) >> 1, v & 1]
        })
        .collect()
}
