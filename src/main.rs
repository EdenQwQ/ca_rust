use rand::random;

fn main() {
    let neighbor: usize = 3;
    let length: usize = 100;
    let times: usize = 100;
    let init_cell = random_bin(length);
    let rule: String = String::from("00000101000000000101010100000101000001010000000001010101000001010101010111111111010101011111111101010101111111110101010111111111");
    // let rule: String = dtb(30, 256);

    let mut cells: Vec<std::string::String> = vec![init_cell];

    for _ in 0..times {
        cells.push(next(&cells[cells.len()-1], neighbor, &rule));
    }

    for cell in cells {
        let mut line: String = String::new();
        for n in cell.chars() {
            line += if n.to_string() == "1" { "█" } else { " " }
        }
        println!("{}", line);
    }
}

fn random_bin(length: usize) -> std::string::String {
    let mut rbin = String::new();
    for _ in 0..length {
        let ran: i8 = random();
        rbin.push(if ran%2 == 0 { '1' } else { '0' })
    }
    rbin
}

fn dtb(d: usize, length: usize) -> std::string::String {
    let mut b = String::new();
    let mut d = d;
    for _ in 0..length {
        b.push_str(&(d % 2).to_string());
        d /= 2;
    }
    b
}

fn next(prev_cell: &str, neighbor: usize, rule: &str) -> std::string::String {
    let mut next_cell = String::new();
    let length: usize = prev_cell.len();
    for n in 1..length+1 {
        let mut status = prev_cell.chars().nth(n-1).unwrap().to_string();
        for i in 1..neighbor+1 {
            status = format!("{}{}{}",
                prev_cell.chars().nth((length+n-i-1)%length).unwrap().to_string(),
                status,
                prev_cell.chars().nth((length+n+i-1)%length).unwrap().to_string(),
                );
        }
        next_cell = next_cell + &rule.chars().nth(usize::from_str_radix(&status, 2).unwrap()).unwrap().to_string();
    }
    next_cell
}
