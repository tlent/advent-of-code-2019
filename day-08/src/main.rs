const INPUT: &str = include_str!("../input.txt");

const IMAGE_DIMENSIONS: (usize, usize) = (25, 6);

type Layer = Vec<Vec<u8>>;

fn main() {
    let layers = parse_input();
    println!("Part one: {}", solve_part_one(&layers));
    println!("Part two:\n{}", solve_part_two(&layers));
}

fn parse_input() -> Vec<Layer> {
    let mut layers = vec![];
    let mut current_layer = vec![];
    let mut current_row = vec![];
    let (row_size, layer_size) = IMAGE_DIMENSIONS;
    for c in INPUT.trim().chars() {
        let digit = c.to_digit(10).unwrap() as u8;
        current_row.push(digit);
        if current_row.len() >= row_size {
            current_layer.push(current_row.clone());
            if current_layer.len() >= layer_size {
                layers.push(current_layer.clone());
                current_layer.clear();
            }
            current_row.clear();
        }
    }
    layers
}

fn solve_part_one(layers: &[Layer]) -> u32 {
    let fewest_zeroes_layer = layers
        .iter()
        .min_by_key(|layer| count_digit(layer, 0))
        .unwrap();
    let one_count = count_digit(fewest_zeroes_layer, 1);
    let two_count = count_digit(fewest_zeroes_layer, 2);
    one_count * two_count
}

fn count_digit(layer: &Layer, digit: u8) -> u32 {
    layer.iter().flatten().filter(|&&d| d == digit).count() as u32
}

fn solve_part_two(layers: &[Layer]) -> String {
    let last_layer = &layers[layers.len() - 1];
    let decoded_image: Layer =
        layers
            .iter()
            .rev()
            .skip(1)
            .fold(last_layer.clone(), |mut image, layer| {
                for (y, row) in layer.iter().enumerate() {
                    for (x, &pixel) in row.iter().enumerate() {
                        if pixel != 2 {
                            image[y][x] = pixel;
                        }
                    }
                }
                image
            });
    let mut output = String::new();
    for row in decoded_image {
        for pixel in row {
            output.push_str(&pixel.to_string());
        }
        output.push('\n');
    }
    output
}
