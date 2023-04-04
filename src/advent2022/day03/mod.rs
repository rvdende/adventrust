fn main() {
    let data = std::fs::read_to_string("sample.txt").unwrap();
    // let data = std::fs::read_to_string("data.txt").unwrap();

    let total: usize = data
        .trim()
        .split("\n")
        .map(|rs| {
            // char value

            let rsi: String = rs.to_string();
            let mut chars = rs
                .chars()
                .map(|c| {
                    let a = 'a' as u8;
                    let z = 'z' as u8;
                    let A = 'A' as u8;
                    let n = c as u8;

                    // println!("{} {} {} {} {}", a, z, A, Z, n);

                    if (n >= a && n <= z) {
                        return n - a + 1;
                    } else {
                        return n - A + 27;
                    }
                })
                .collect::<Vec<u8>>();

            // println!("{}", chars.len());

            let mut output: usize = 0;

            for x in 0..chars.len() / 2 {
                for y in chars.len() / 2..chars.len() {
                    if (x != y) {
                        let objX = chars[x];
                        let objY = chars[y];
                        if (objX == objY) {
                            let z = rsi.as_bytes()[x] as char;
                            // println!("{}",rsi.to_string());
                            // println!("found match x {} y {} char {}", x, y, z);
                            output = chars[x].into();
                            return output;
                        }
                    }
                }
            }

            return output;
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("part 1 total: {}", total);

    // ---------------------

    let part2: Vec<_> = data.trim().split("\n").collect();

    let mut idx = 0;
    loop {
        let a = part2[idx].to_string();
        let b = part2[idx + 1].to_string();
        let c = part2[idx + 2].to_string();
        idx += 3;

        if (idx >= part2.len()) {
            break;
        }
    }

    dbg!(part2);
}
