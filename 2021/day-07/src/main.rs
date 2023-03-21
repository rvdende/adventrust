fn calcfuel(data: &Vec<isize>, pos: isize) -> usize {
    let output: usize = data
        .iter()
        .map(|c| calcexpfuel((c - pos).abs() as usize))
        .sum();
    return output;
}

fn getminmax(data: &Vec<isize>) -> (isize, isize) {
    let min = data.iter().min().unwrap();
    let max = data.iter().max().unwrap();
    return (*min, *max);
}

fn main() {
    let filename = "input.txt";
    // let filename = "sample.txt";

    let data = std::fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|f| f.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    // let fuel: isize = calcfuel(&data, 2) as isize;
    // println!("fuel {}", fuel);

    let minmax = getminmax(&data);
    println!("range {} - {}", minmax.0, minmax.1);

    let mut minfuel = std::isize::MAX;
    let mut minpos = 0;

    for p in minmax.0..=minmax.1 {
        let fuelt = calcfuel(&data, p) as isize;
        println!("pos {} fuel {}", p, fuelt);

        if fuelt < minfuel {
            minfuel = fuelt;
            minpos = p;
        }
    }

    println!("minpos {} minfuel {}", minpos, minfuel)
}

fn calcexpfuel(distance: usize) -> usize {
    let mut total = 0;

    for i in 0..=distance {
        total += i;
    }

    return total;
}

#[test]
fn test() {
    assert_eq!(calcexpfuel(1), 1);
    assert_eq!(calcexpfuel(2), 3);
    assert_eq!(calcexpfuel(3), 6);
    assert_eq!(calcexpfuel(4), 10);
    assert_eq!(calcexpfuel(5), 15);
}
