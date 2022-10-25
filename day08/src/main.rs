use array2d::Array2D;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found");
    let images = parse_images(contents, 6, 25);
    println!("{}", part1(&images)); //2318
    part2(images);
}

fn parse_images(data: String, height: usize, width: usize) -> Vec<Array2D<u8>> {
    let mut images: Vec<Array2D<u8>> = Vec::new();
    let mut digits = data.bytes().peekable();

    while digits.peek().is_some() {
        let mut image = Array2D::filled_with(0, height, width);
        for row in 0..image.num_rows() {
            for col in 0..image.num_columns() {
                image[(row, col)] = digits.next().unwrap();
            }
        }
        images.push(image);
    }
    images
}

fn part1(images: &Vec<Array2D<u8>>) -> i32 {
    let mut min = i32::MAX;
    let mut result = 0;
    for image in images {
        let mut z_count = 0;
        let mut one_count = 0;
        let mut two_count = 0;
        for row in image.rows_iter() {
            for x in row {
                match x {
                    b'0' => z_count += 1,
                    b'1' => one_count += 1,
                    b'2' => two_count += 1,
                    _ => (),
                }
            }
        }
        if z_count < min {
            min = z_count;
            result = one_count * two_count;
        }
    }
    result
}

fn part2(images: Vec<Array2D<u8>>) {
    let mut result = Array2D::filled_with(b'2', 6, 25);
    for image in images {
        for row in 0..image.num_rows() {
            for col in 0..image.num_columns() {
                if result[(row, col)] == b'2' {
                    result[(row, col)] = image[(row, col)];
                }
            }
        }
        if filled(&result) {
            break;
        }
    }
    print_image(&result);
}

fn filled(image: &Array2D<u8>) -> bool {
    for row in 0..image.num_rows() {
        for col in 0..image.num_columns() {
            if image[(row, col)] == b'2' {
                return false;
            }
        }
    }
    true
}

fn print_image(image: &Array2D<u8>) {
    for row in 0..image.num_rows() {
        for col in 0..image.num_columns() {
            if image[(row, col)] == b'0' {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("input.txt").expect("File not found");
        let images = parse_images(contents, 6, 25);
        assert_eq!(part1(&images), 2318);
    }
}
