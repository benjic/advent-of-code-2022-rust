use std::io;

fn main() {
    let mut sum = 0;
    let mut max = (0, 0, 0);

    for line in io::stdin().lines() {
        match line {
            Ok(val) => {
                if val.is_empty() {
                    if sum > max.0 {
                        max = (sum, max.1, max.2)
                    } else if sum > max.1 {
                        max = (max.0, sum, max.1)
                    } else if sum > max.2 {
                        max = (max.0, max.1, sum)
                    }

                    sum = 0;
                } else {
                    match val.parse::<i32>() {
                        Ok(num) => sum = sum + num,
                        Err(err) => panic!("malformed line {}", err),
                    }
                }
            }
            Err(err) => {
                panic!("failed to iterate lines {}", err);
            }
        }
    }

    println!("part 1: {}", max.0);
    println!("part 2: {}", max.0 + max.1 + max.2);
}

