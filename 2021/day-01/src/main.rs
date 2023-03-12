use std::fs;

fn day1_part1() {
    let data = match std::fs::read_to_string("sample.txt") {
        Ok(data) => data
            .trim()
            .split("\n")
            .map(|x| {
                println!("raw: {}", x);
                let a = match x.parse::<usize>() {
                    Ok(a) => a,
                    Err(err) => panic!("Error: {}", err),
                };
                println!("parsed: {}", a);
                return a;
            })
            .collect::<Vec<usize>>(),
        Err(err) => panic!("Error: {}", err),
    };

    dbg!(&data);

    // day 1

    let mut previous = usize::MAX;
    let mut increases = 0;

    for i in data {
        if i > previous {
            println!("increase! {} > {}", i, previous);
            increases += 1;
        }
        previous = i;
        println!("{}", i);
    }

    println!("Increases: {}", increases);
}

////////////////////////////////////////////////////////////////////

fn day1_part2() {
    let data = match std::fs::read_to_string("datainput.txt") {
        Ok(data) => data
            .trim()
            .split("\n")
            .map(|x| {
                println!("raw: {}", x);
                let a = match x.parse::<usize>() {
                    Ok(a) => a,
                    Err(err) => panic!("Error: {}", err),
                };
                println!("parsed: {}", a);
                return a;
            })
            .collect::<Vec<usize>>(),
        Err(err) => panic!("Error: {}", err),
    };

    println!("-----------------");
    let mut increases = 0;
    for i in 0..data.len() - 3 {
        let a = data[i] + data[i + 1] + data[i + 2];
        let b = data[i + 1] + data[i + 2] + data[i + 3];

        if b > a {
            increases += 1;
        }

        println!("total: {}", a);
        println!("{}", i);
        println!("{}", data[i])
    }

    println!("Increases: {}", increases);
}

fn main() {
    day1_part2();
}
