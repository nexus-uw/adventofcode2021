fn main() {
  // let input: Vec<Result<i32, std::io::Error>>  =  std::fs::read_to_string("./input.txt")
  let input: Vec<i32>  =  std::fs::read_to_string("./input_one.txt")
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().expect("parse error"))
        .collect();

  let answer:i32 = (&input).into_iter().enumerate().fold(0, |sum, (index,value)|{
    if index == 0{
      return 0
    }
    if value > input.get(index-1).expect("index invalid"){
      return sum + 1
    }else {
      return sum
    }
  });
  println!("{}", answer)
}