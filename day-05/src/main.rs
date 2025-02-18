fn part_1(input: &str) {
    let mut rules: Vec<Vec<bool>> = vec![vec![false; 100]; 100];
    let mut updates: Vec<Vec<i32>> = vec![];
    input.lines().for_each(|l| {
        if l.contains("|") {
            let (left, right) = l.split_once("|").unwrap();
            let (before, after): (usize, usize) = (left.parse().unwrap(), right.parse().unwrap());

            rules[before][after] = true;
        }
        if l.contains(",") {
            let mut update: Vec<i32> = vec![];
            l.split(",").for_each(|n| update.push(n.parse().unwrap()));

            updates.push(update);
        }
    });

    let mut middle_page_nums = vec![];
    for update in updates {
        let mut is_following_rules = true;
        'o: for (i, before) in update.iter().enumerate() {
            for after in update.iter().skip(i + 1) {
                if rules[*after as usize][*before as usize] {
                    is_following_rules = false;
                    break 'o;
                }
            }
        }
        if is_following_rules {
            let middle = update.len() / 2;
            middle_page_nums.push(update[middle]);
        }
    }

    let sum: i32 = middle_page_nums.iter().sum();

    println!("{sum}");
}

fn part_2() {}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");

    part_1(&input);
    part_2();
}
