fn main() {
  // let input: Vec<Result<i32, std::io::Error>>  =  std::fs::read_to_string("./input.txt")
  let input: Vec<i32>  =  std::fs::read_to_string("./input_two.txt")
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().expect("parse error"))
        .collect();

  let answer:i32 = (&input).into_iter().enumerate().fold(0, |sum, (index,_value)|{
    if index < 1{
      return 0
    }
    if (index + 3) > input.len() {
      return sum
    }
    let first = input.get(index-1).expect("index invalid") + input.get(index).expect("index invalid") + input.get(index+1).expect("index invalid");

    let second = input.get(index).expect("index invalid") + input.get(index+1).expect("index invalid") + input.get(index+2).expect("index invalid");

    if second > first {
      return sum + 1
    }else {
      return sum
    }
  });
  println!("{}", answer)
}