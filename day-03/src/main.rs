use regex::Regex;

fn part_1(input: &str) {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mults: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut sum = 0;
    mults.iter().for_each(|m| {
        let x = &m[4..m.len() - 1];
        let (l, r) = x.split_at(x.find(",").unwrap());
        let l: i32 = l.parse().unwrap();
        let r: i32 = r[1..r.len()].parse().unwrap();
        sum += l * r;
    });
    println!("{sum}");
}

//fn part_2(input: &str) {
//    todo!();
//}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");

    part_1(&input);
    //part_2(&input);
}
