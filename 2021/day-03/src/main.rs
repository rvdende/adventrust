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

fn printVecVecUsize(data: &Vec<Vec<usize>>) {
    println!();
    for l in data {
        for b in l {
            print!("{}", b);
        }
        println!("");
    }
    println!();
}

fn printVecUsize(data: &Vec<usize>) {
    println!();
    for b in data {
        print!("{}", b);
    }
    println!();
}

fn iterateFindNextOxygen(data_in_raw: &Vec<Vec<usize>>, indexpos: usize) -> Vec<Vec<usize>> {
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
        .clone()
        .iter()
        .map(|l| l.clone())
        .filter(|l| {
            if (l[indexpos] == morebit) {
                return true;
            }
            return false;
        })
        .collect();

    return localcopy;
}

fn iterateFindNextCO2(data_in_raw: &Vec<Vec<usize>>, indexpos: usize) -> Vec<Vec<usize>> {
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
        .clone()
        .iter()
        .map(|l| l.clone())
        .filter(|l| {
            if (l[indexpos] == fewerbit) {
                return true;
            }
            return false;
        })
        .collect();

    return localcopy;
}

fn part2_oxygen_generator_rating(data_in: &Vec<Vec<usize>>) {
    let mut indexposition = 0;
    let mut calcdata = data_in.clone();
    loop {
        // println!("===== loop {}", indexposition);
        calcdata = iterateFindNextOxygen(&calcdata, indexposition);
        // printVecVecUsize(&calcdata);
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
        // println!("===== loop {}", indexposition);
        calcdata = iterateFindNextCO2(&calcdata, indexposition);
        // printVecVecUsize(&calcdata);
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

    // printVecVecUsize(&calcdata);
    println!("oxygen * co2 = {}", oxygen * co2);
}

fn main() {
    //let data = std::fs::read_to_string("sample.txt").unwrap();
    let data = std::fs::read_to_string("data.txt").unwrap();

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
    // println!("nor {} => {:b}", num, num);
    // println!("inv {} => {:b}", invertednum, invertednum);
    return (num, invertednum);
}
