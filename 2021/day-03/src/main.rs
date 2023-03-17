fn main() {
    // let data = std::fs::read_to_string("sample.txt").unwrap();
    let data = std::fs::read_to_string("data.txt").unwrap();

    let mut common : Vec<usize> = vec![];

    let a: Vec<Vec<usize>> = data.lines().enumerate().map(|(l_id,l)| {
        // let bits: Vec<usize> = l.chars().map(|b| b.is_ascii_digit()).collect();
        let bits = l.chars().enumerate().map(|(b_id,b)| {

            let val = match b {
                '0' => 0,
                '1' => 1,
                _ => panic!("Invalid bit")
            };

            if common.len() < b_id+1 {
                common.push(0);
            }
            common[b_id] += val;

            println!("line: {} num: {} = {}", l_id, b_id, val);
            return val;
        }).collect::<Vec<usize>>();
        
        return bits;
    }).collect();

    let commonBool: Vec<bool> = common.iter().map(|c| {
        if c > &(a.len()/2) {
            return true;
        } else {
            return false;
        }
    }).collect();
    // dbg!(a);
    dbg!(&commonBool);
    let (gamma, epsilon) = BoolVecToNum(commonBool);
    let power_consumption = gamma * epsilon;

    dbg!(power_consumption);
}


fn BoolVecToNum(bits: Vec<bool>) -> (usize, usize) {
    println!("{}",bits.len());
    let mut bitslength: u32 = bits.len() as u32;
    bitslength -= 1;
    let mut num = 0;
    let mut invertednum = 0;
    for (i, bit) in bits.iter().enumerate() {

        
            if !(*bit) {
                invertednum += 2usize.pow(bitslength-i as u32);
            }
        
            if *bit {
                num += 2usize.pow(bitslength-i as u32);
            }
        

        
    }
    println!("nor {} => {:b}", num, num);
    println!("inv {} => {:b}", invertednum, invertednum);
    return (num, invertednum);
}