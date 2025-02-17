fn part_1(input: String) {
    let safe_count = input
        .lines()
        .map(|l| {
            let nums: Vec<i32> = l.split_whitespace().map(|e| e.parse().unwrap()).collect();
            match nums.len() {
                0 | 1 => 1,
                _ => {
                    let incline = if nums[1] > nums[0] { 1 } else { -1 };
                    let mut is_safe = 1;
                    for (i, _) in nums.iter().enumerate() {
                        if i == nums.len() - 1 {
                            break;
                        }
                        match (nums[i + 1] - nums[i]) * incline {
                            1 | 2 | 3 => {}
                            _ => is_safe = 0,
                        }
                    }
                    is_safe
                }
            }
        })
        .sum::<i32>();
    println!("{safe_count}");
}

//fn part_2(list_1: &Vec<i32>, list_2: &Vec<i32>) {
//    todo!();
//}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");

    part_1(input);
    //part_2(&list_1, &list_2);
}
