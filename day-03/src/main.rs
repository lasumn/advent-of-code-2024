use regex::Regex;

fn is_active(m_index: &usize, do_indexes: &Vec<usize>, dont_indexes: &Vec<usize>) -> bool {
    let mut last_do: usize = 0;
    let mut last_dont: usize = 0;
    do_indexes.iter().for_each(|i| {
        if i < m_index {
            last_do = *i;
        }
    });
    dont_indexes.iter().for_each(|i| {
        if i < m_index {
            last_dont = *i;
        }
    });

    if last_do < last_dont {
        false
    } else {
        true
    }
}
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

fn part_2(input: &str) {
    let re_mul = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mults: Vec<regex::Match> = re_mul.find_iter(input).collect();
    let mut do_indexes: Vec<usize> = re_do.find_iter(input).map(|m| m.start()).collect();
    let mut dont_indexes: Vec<usize> = re_dont.find_iter(input).map(|m| m.start()).collect();
    do_indexes.sort();
    dont_indexes.sort();

    let mut sum = 0;
    mults.iter().for_each(|m| {
        let m_index = m.start();
        if is_active(&m_index, &do_indexes, &dont_indexes) {
            let x = &m.as_str()[4..m.as_str().len() - 1];
            let (l, r) = x.split_at(x.find(",").unwrap());
            let l: i32 = l.parse().unwrap();
            let r: i32 = r[1..r.len()].parse().unwrap();
            sum += l * r;
        }
    });
    println!("{sum}");
}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");

    part_1(&input);
    part_2(&input);
}
