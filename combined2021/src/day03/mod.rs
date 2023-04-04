#[allow(dead_code)]
// --------------------------------------------------

fn vec_usize_to_vec_bool(bits: Vec<usize>) -> Vec<bool> {
    let mut out: Vec<bool> = vec![];
    for b in bits {
        if b == 0 {
            out.push(false);
        } else {
            out.push(true);
        }
    }
    return out;
}

fn part1(data_in: &Vec<Vec<usize>>) -> usize {
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

    let common_bool: Vec<bool> = common.iter().map(|c| c > &(data_in.len() / 2)).collect();
    // dbg!(a);
    // dbg!(&common_bool);
    let (gamma, epsilon) = vec_bool_binary_to_usize(common_bool);
    let power_consumption = gamma * epsilon;
    println!("----------------------------");
    println!("power_consumption: {}", power_consumption);

    power_consumption
}

fn iterate_find_next_oxygen(data_in_raw: &Vec<Vec<usize>>, indexpos: usize) -> Vec<Vec<usize>> {
    let data_in = &data_in_raw.clone();

    let mut bit0count = 0;
    let mut bit1count = 0;

    for l in data_in {
        if l[indexpos] == 0 {
            bit0count += 1;
        } else {
            bit1count += 1;
        }
    }

    let morebit = if bit1count >= bit0count { 1 } else { 0 };

    let localcopy: Vec<Vec<usize>> = data_in
        .iter()
        .cloned()
        .filter(|l| l[indexpos] == morebit)
        .collect();

    localcopy
}

fn iterate_find_next_co2(data_in_raw: &Vec<Vec<usize>>, indexpos: usize) -> Vec<Vec<usize>> {
    let data_in = &data_in_raw.clone();

    let mut bit0count = 0;
    let mut bit1count = 0;

    for l in data_in {
        if l[indexpos] == 0 {
            bit0count += 1;
        } else {
            bit1count += 1;
        }
    }

    let fewerbit = if bit0count <= bit1count { 0 } else { 1 };

    let localcopy: Vec<Vec<usize>> = data_in
        .iter()
        .cloned()
        .filter(|l| l[indexpos] == fewerbit)
        .collect();

    localcopy
}

fn part2_oxygen_generator_rating(data_in: &Vec<Vec<usize>>) -> usize {
    let mut indexposition = 0;
    let mut calcdata = data_in.clone();
    loop {
        calcdata = iterate_find_next_oxygen(&calcdata, indexposition);
        indexposition += 1;
        if calcdata.len() < 2 {
            break;
        }
        if indexposition == calcdata[0].len() {
            break;
        }
    }
    let data = vec_usize_to_vec_bool(calcdata[0].clone());
    let (oxygen, _) = vec_bool_binary_to_usize(data);
    println!("oxygen = {}", oxygen);

    //////////// -------------------

    let mut indexposition = 0;
    let mut calcdata = data_in.clone();
    loop {
        calcdata = iterate_find_next_co2(&calcdata, indexposition);
        indexposition += 1;
        if calcdata.len() < 2 {
            break;
        }
        if indexposition == calcdata[0].len() {
            break;
        }
    }
    let data = vec_usize_to_vec_bool(calcdata[0].clone());

    let (co2, _) = vec_bool_binary_to_usize(data);
    println!("co2 = {}", co2);

    println!("oxygen * co2 = {}", oxygen * co2);

    oxygen * co2
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
    // println!("nor {} => {:b}", num, num);
    // println!("inv {} => {:b}", invertednum, invertednum);
    (num, invertednum)
}

#[allow(dead_code)]
pub fn process(filename: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(filename).unwrap();

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
    let part1_answer = part1(&parsed);
    println!("--------------- part 2 -------------");
    let part2_answer = part2_oxygen_generator_rating(&parsed);
    println!("--------------- done -------------");

    (part1_answer, part2_answer)
}

#[test]
fn test() {
    let sample = process("src/day03/sample.txt");
    assert_eq!(sample, (198, 230));

    let input = process("src/day03/input.txt");
    assert_eq!(input, (3901196, 4412188));
}
