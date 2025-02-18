fn check_word_1(
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

fn check_word_2(
    grid: &Vec<Vec<char>>,
    max_row: usize,
    max_col: usize,
    row: usize,
    col: usize,
) -> bool {
    let start_row: i32 = row as i32 - 1;
    let start_col: i32 = col as i32 - 1;
    let end_row: i32 = row as i32 + 1;
    let end_col: i32 = col as i32 + 1;

    if 0 > start_row || end_row > max_row as i32 || 0 > start_col || end_col > max_col as i32 {
        return false;
    }

    let mut letters_1 = vec![];
    let mut letters_2 = vec![];

    for i in -1..2 {
        //println!("row:{row}, row_direction: {row_direction}, col:{col}, col_direction: {col_direction}, i: {i}");
        letters_1.push(grid[(row as i32 + i) as usize][(col as i32 + i) as usize]);
        letters_2.push(grid[(row as i32 - i) as usize][(col as i32 + i) as usize]);
    }

    if ((letters_1[0] == 'M' && letters_1[1] == 'A' && letters_1[2] == 'S')
        || (letters_1[0] == 'S' && letters_1[1] == 'A' && letters_1[2] == 'M'))
        && ((letters_2[0] == 'M' && letters_2[1] == 'A' && letters_2[2] == 'S')
            || (letters_2[0] == 'S' && letters_2[1] == 'A' && letters_2[2] == 'M'))
    {
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
                if check_word_1(input, "XMAS", last_row, last_col, r, c, rowd, cold) {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}

fn part_2(input: &Vec<Vec<char>>, last_row: usize, last_col: usize) {
    let mut count = 0;

    for r in 0..last_row + 1 {
        for c in 0..last_col + 1 {
            if check_word_2(input, last_row, last_col, r, c) {
                count += 1;
            }
        }
    }

    println!("{count}");
}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    part_1(&input, input[0].len() - 1, input.len() - 1);
    part_2(&input, input[0].len() - 1, input.len() - 1);
}
