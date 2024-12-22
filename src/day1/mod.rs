pub mod part1 {
    use std::fs;

    pub fn run() -> u64 {
        let file_path = "src/day1/input.txt";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let lines = contents.lines();

        let mut a: Vec<i64> = vec![];
        let mut b: Vec<i64> = vec![];

        for line in lines {
            let temp: Vec<&str> = line.split("   ").collect();
            a.push(temp[0].parse().unwrap());
            b.push(temp[1].parse().unwrap());
        }

        a.sort();
        b.sort();

        let mut total = 0;

        for i in 0..a.len() {
            let diff: i64 = a[i] - b[i];
            total += diff.abs();
        }
        println!("{}", total);

        return 0;
    }
}

pub mod part2 {
    use std::fs;

    pub fn run() -> u64 {
        let file_path = "src/day1/input.txt";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let lines = contents.lines();

        let mut a: Vec<u64> = vec![];
        let mut b: Vec<u64> = vec![];

        for line in lines {
            let temp: Vec<&str> = line.split("   ").collect();
            a.push(temp[0].parse().unwrap());
            b.push(temp[1].parse().unwrap());
        }

        let mut total = 0;

        for i in 0..a.len() {
            let similarity_score = u64::try_from(b.iter().filter(|&n| *n == a[i]).count())
                .ok()
                .unwrap()
                * a[i];
            total += similarity_score;
        }
        println!("{}", total);

        return 0;
    }
}
