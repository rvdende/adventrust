// --------------------------------------------------

fn part1(data_in: &Vec<Vec<usize>>) {
    let mut common: Vec<usize> = vec![];
    for l in data_in {
        for (b_id, b) in l.iter().enumerate() {
            if common.len() < b_id + 1 {
                common.push(0);
            }
            common[b_id] += b;
        }
    }

    // dbg!(&common);

    let common_bool: Vec<bool> = common
        .iter()
        .map(|c| {
            if c > &(data_in.len() / 2) {
                return true;
            } else {
                return false;
            }
        })
        .collect();
    // dbg!(a);
    // dbg!(&common_bool);
    let (gamma, epsilon) = vec_bool_binary_to_usize(common_bool);
    let power_consumption = gamma * epsilon;
    println!("----------------------------");
    println!("power_consumption: {}", power_consumption);
}

fn part2_oxygen_generator_rating(data_in: &Vec<Vec<usize>>) {
    let mut common: Vec<usize> = vec![];
    for l in data_in {
        for (b_id, b) in l.iter().enumerate() {
            if common.len() < b_id + 1 {
                common.push(0);
            }
            common[b_id] += b;
        }
    }

    let common_bool: Vec<usize> = common
        .iter()
        .map(|c| {
            if c > &(data_in.len() / 2) {
                return 1;
            } else {
                return 0;
            }
        })
        .collect();

    dbg!(&common_bool);

    data_in.iter().for_each(|l| {
        let a = l.len();
        let b = common_bool.len();
        println!("a; {} b: {}", a, b);
        l.iter().for_each(|n| {
            // todo...
        })
    });
}

fn main() {
    let data = std::fs::read_to_string("sample.txt").unwrap();
    // let data = std::fs::read_to_string("data.txt").unwrap();

    let parsed: Vec<Vec<usize>> = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|b| match b {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Invalid bit"),
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    println!("--------------- part 1 -------------");
    part1(&parsed);
    println!("--------------- part 2 -------------");
    part2_oxygen_generator_rating(&parsed);
    println!("--------------- done -------------");
}

fn vec_bool_binary_to_usize(bits: Vec<bool>) -> (usize, usize) {
    // println!("{}", bits.len());
    let mut bitslength: u32 = bits.len() as u32;
    bitslength -= 1;
    let mut num = 0;
    let mut invertednum = 0;
    for (i, bit) in bits.iter().enumerate() {
        if !(*bit) {
            invertednum += 2usize.pow(bitslength - i as u32);
        }

        if *bit {
            num += 2usize.pow(bitslength - i as u32);
        }
    }
    println!("nor {} => {:b}", num, num);
    println!("inv {} => {:b}", invertednum, invertednum);
    return (num, invertednum);
}
