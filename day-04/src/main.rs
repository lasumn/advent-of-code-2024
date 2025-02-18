fn check_word(
    grid: &Vec<Vec<char>>,
    word: &str,
    max_row: usize,
    max_col: usize,
    row: usize,
    col: usize,
    row_direction: &i32,
    col_direction: &i32,
) -> bool {
    let word_len: i32 = word.len() as i32;
    let end_row: i32 = row as i32 + row_direction * (word_len - 1);
    let end_col: i32 = col as i32 + col_direction * (word_len - 1);

    if 0 > end_row || end_row > max_row as i32 || 0 > end_col || end_col > max_col as i32 {
        return false;
    }

    let mut letters = String::new();

    for i in 0..word_len {
        //println!("row:{row}, row_direction: {row_direction}, col:{col}, col_direction: {col_direction}, i: {i}");
        letters.push(
            grid[(row as i32 + (row_direction * i)) as usize]
                [(col as i32 + (col_direction * i)) as usize],
        );
    }

    if letters == word {
        true
    } else {
        false
    }
}

fn part_1(input: &Vec<Vec<char>>, last_row: usize, last_col: usize) {
    let mut count = 0;
    #[rustfmt::skip]
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for r in 0..last_row + 1 {
        for c in 0..last_col + 1 {
            for (rowd, cold) in &directions {
                if check_word(input, "XMAS", last_row, last_col, r, c, rowd, cold) {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}

//fn part_2(input: &Vec<&str>, rowc: usize, colc: usize) {
//
//}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    part_1(&input, input[0].len() - 1, input.len() - 1);
    //part_2(&input);
}
