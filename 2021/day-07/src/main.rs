fn calcfuel(data: &Vec<isize>, pos: isize) -> isize {
    let output: isize = data.iter().map(|c| (c - pos).abs()).sum();
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

    let fuel = calcfuel(&data, 2);
    println!("fuel {}", fuel);

    let minmax = getminmax(&data);
    println!("range {} - {}", minmax.0, minmax.1);

    let mut minfuel = std::isize::MAX;
    let mut minpos = 0;

    for p in minmax.0..=minmax.1 {
        let fuel = calcfuel(&data, p);
        println!("pos {} fuel {}", p, fuel);

        if fuel < minfuel {
            minfuel = fuel;
            minpos = p;
        }
    }

    println!("minpos {} minfuel {}", minpos, minfuel)
}
