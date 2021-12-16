use std::collections::VecDeque;

fn convert_to_binary_from_hex(hex: &str) -> String {
    let to_binary = hex.chars().map(|c| to_binary(c)).collect();
    to_binary
}

fn to_binary(c: char) -> String {
    let b = match c {
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
    };
    b.to_string()
}

fn read(n: i32, bin: &mut String) -> String {
    let mut out = "".to_string();
    let mut char_vec: VecDeque<char> = bin.chars().collect();
    for i in 0..n {
        out += &char_vec.pop_front().unwrap().to_string();
    }
    *bin = char_vec.into_iter().collect();
    out
}

fn main() {
    let hex = "8A004A801A8002F478".to_string();
    //let hex = "D2FE28".to_string();
    let mut binary = convert_to_binary_from_hex(&hex);
    println!("{}", binary);
    while true {
        let version_bin = read(3, &mut binary);
        let version = isize::from_str_radix(&version_bin, 2).unwrap();
        println!("{:?}", version);
        let packet_type_bin = read(3, &mut binary);
        let packet_type = isize::from_str_radix(&packet_type_bin, 2).unwrap();
        println!("{:?}", packet_type);
        if packet_type != 4 {
            // it’s an operator
            let length_type_id = read(1, &mut binary);
            println!("{:?}", length_type_id);
        } else {
            break;
        }
        break;
    }
}
