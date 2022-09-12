//use sscanf::*;

fn main() {
    // let input: Vec<Result<i32, std::io::Error>>  =  std::fs::read_to_string("./input.txt")

    let input = std::fs::read_to_string("./input.txt").expect("file not found!");

    fn reduce_mostcommon(input: Vec<&str>, index: usize) -> Vec<&str> {
        // found the number
        if input.len() == 1 {
            return input;
        }

        let one_count = input.iter().fold(0, |sum, row| {
            if row.chars().nth(index).unwrap() == '1' {
                return sum + 1;
            }
            return sum;
        });

        if one_count > (input.len() / 2) {
            return reduce_mostcommon(
                input
                    .into_iter()
                    .filter(|row| return row.chars().nth(index).unwrap() == '1')
                    .collect(),
                index + 1,
            );
        } else {
            return reduce_mostcommon(
                input
                    .into_iter()
                    .filter(|row| return row.chars().nth(index).unwrap() != '1')
                    .collect(),
                index + 1,
            );
        }
    }

    let oxygen = reduce_mostcommon(input.lines().collect(), 0);

    fn reduce_leastcommon(input: Vec<&str>, index: usize) -> Vec<&str> {
        // found the number
        if input.len() == 1 {
            return input;
        }

        let one_count = input.iter().fold(0, |sum, row| {
            if row.chars().nth(index).unwrap() == '1' {
                return sum + 1;
            }
            return sum;
        });

        if one_count < (input.len() / 2) {
            return reduce_leastcommon(
                input
                    .into_iter()
                    .filter(|row| return row.chars().nth(index).unwrap() == '1')
                    .collect(),
                index + 1,
            );
        } else {
            return reduce_leastcommon(
                input
                    .into_iter()
                    .filter(|row| return row.chars().nth(index).unwrap() != '1')
                    .collect(),
                index + 1,
            );
        }
    }
    let c02 = reduce_leastcommon(input.lines().collect(), 0);

    // todo convert most common and least common into decimal and then mult them
}
