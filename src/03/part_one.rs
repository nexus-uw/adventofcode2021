//use sscanf::*;

fn main() {
  // let input: Vec<Result<i32, std::io::Error>>  =  std::fs::read_to_string("./input.txt")
  let input = std::fs::read_to_string("./input.txt").expect("file not found!");

  let rowCount = input.lines().count(); // number of rows

  let columCounts = input
    .lines()
    .map(|x| x.split(""))
    .fold(Vec::new(), |mut accum, split| {
      // go through each row to count up the 1s in each column, idx is 1 based (for some reason....)
      split.enumerate().for_each(|(idx, value)| {
        // handle jnk input/eol
        if value.is_empty() {
          return;
        }
        // for first row, make sure the accum vector has enough items in it to get keep count
        if accum.len() <= (idx - 1) {
          accum.push(0);
        }
        if value == "1" {
          accum[(idx - 1)] += 1;
        }
      });
      accum
    });
  let mut gamma = 0;
  let mut epsilon = 0;
  for col in columCounts {
    gamma = gamma * 2;
    epsilon = epsilon * 2;
    if col > rowCount / 2 {
      gamma = gamma + 1
      // 1 was most common digit for  col
    } else {
      epsilon = epsilon + 1
      // 0 was most common digit
    }
  }
  println!("{}", gamma * epsilon)
}
