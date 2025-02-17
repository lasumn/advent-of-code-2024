enum SafetyReport {
    Safe,
    FailureAt(usize),
}

fn test_safety(nums: &Vec<i32>) -> SafetyReport {
    match nums.len() {
        0 | 1 => SafetyReport::Safe,
        _ => {
            let incline = if nums[1] > nums[0] { 1 } else { -1 };
            let mut safety_report = SafetyReport::Safe;
            for (i, _) in nums.iter().enumerate() {
                if i == nums.len() - 1 {
                    break;
                }
                match (nums[i + 1] - nums[i]) * incline {
                    1 | 2 | 3 => {}
                    _ => {
                        safety_report = SafetyReport::FailureAt(i);
                        break;
                    }
                }
            }
            safety_report
        }
    }
}

fn part_1(input: &str) {
    let safe_count: i32 = input
        .lines()
        .map(|l| {
            let nums: Vec<i32> = l.split_whitespace().map(|e| e.parse().unwrap()).collect();
            match test_safety(&nums) {
                SafetyReport::Safe => 1,
                SafetyReport::FailureAt(_) => 0,
            }
        })
        .sum();
    println!("{safe_count}");
}

fn part_2(input: &str) {
    let safe_count: i32 = input
        .lines()
        .map(|l| {
            let nums: Vec<i32> = l.split_whitespace().map(|e| e.parse().unwrap()).collect();
            match test_safety(&nums) {
                SafetyReport::Safe => 1,
                SafetyReport::FailureAt(i) => {
                    let mut test_1 = nums.clone();
                    let mut test_2 = nums.clone();
                    let mut test_3 = nums.clone();
                    test_1.remove(i.saturating_sub(1));
                    test_2.remove(i);
                    test_3.remove(i.saturating_add(1));
                    match (
                        test_safety(&test_1),
                        test_safety(&test_2),
                        test_safety(&test_3),
                    ) {
                        (SafetyReport::Safe, _, _) => 1,
                        (_, SafetyReport::Safe, _) => 1,
                        (_, _, SafetyReport::Safe) => 1,
                        _ => 0,
                    }
                }
            }
        })
        .sum();
    println!("{safe_count}");
}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");

    part_1(&input);
    part_2(&input);
}
