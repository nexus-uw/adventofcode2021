//use sscanf::*;

fn main() {
    // let input: Vec<Result<i32, std::io::Error>>  =  std::fs::read_to_string("./input.txt")
    let input: Vec<(String, i32)> = std::fs::read_to_string("./input_two.txt")
        .expect("file not found!")
        .lines()
        .map(|x| {
            return sscanf::scanf!(x, "{} {}", String, i32).unwrap();
        })
        .collect();

    let answer: (i32, i32, i32) =
        (&input)
            .into_iter()
            .enumerate()
            .fold((0, 0, 0), |result, (_, value)| {
                let (mut horizontal, mut vertical, mut aim) = result;
                let (movement, distance) = &*value;
                /*
                    forward X does two things:
                      It increases your horizontal position by X units.
                      It increases your depth by your aim multiplied by X.

                    down X increases the depth by X units.
                    up X decreases the depth by X units.

                */
                if movement == "forward" {
                    horizontal = horizontal + distance;
                    vertical = vertical + (aim * distance);
                } else if movement == "up" {
                    aim = aim - distance;
                } else if movement == "down" {
                    aim = aim + distance;
                } else {
                    panic!("unknown movement")
                }

                return (horizontal, vertical, aim);
            });
    println!("{}", answer.0 * answer.1);
}
