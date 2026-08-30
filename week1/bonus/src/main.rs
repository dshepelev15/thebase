
const SIZE: usize = 8;

pub fn parse_bitmap_8x8(lines: [&str; SIZE]) -> [u8; SIZE] {
    let mut result: [u8; SIZE] = [0; SIZE];
    

    for (element_index, line) in lines.iter().enumerate() {
        for (index, c) in line.chars().enumerate() {
            if c == '#' {
                result[element_index] |= 1 << (SIZE - 1 - index);
            }
        }
    }

    result
}

pub fn render_bitmap_8x8(bytes: [u8; SIZE]) -> [String; SIZE] {
    const EMPTY: String = String::new();
    let mut result: [String; SIZE] = [EMPTY; SIZE];

    for (element_index, value) in bytes.iter().enumerate() {
        let mut value = *value;
        let mut i = 0;
        let mut line = Vec::with_capacity(SIZE);
        while i < SIZE {
            if value & 1 == 1 {
                line.push('#');
            } else {
                line.push('.');
            }
            i += 1;
            value = value >> 1;
        }
        line.reverse();
        result[element_index] = String::from_iter(line);
    }

    result

}

pub fn invert_bitmap_8x8(bytes: [u8; SIZE]) -> [u8; SIZE] {
    let mut result: [u8; SIZE] = [0; SIZE];

    for (element_index, value) in bytes.iter().enumerate() {
        result[element_index] = !value
    }

    result
}

fn main() {
    let image = [
        "..####..",
        ".#....#.",
        "#.#..#.#",
        "#..##..#",
        "#......#",
        "#.#..#.#",
        ".#....#.",
        "..####..",
    ];
    let bytes = parse_bitmap_8x8(image);
    
    println!("Bytes:");
    for byte in bytes {
        println!("{byte:08b} 0x{byte:02X}");
    }
    println!();
    
    println!("Rendered:");
    for line in render_bitmap_8x8(bytes) {
        println!("{line}");
    }
    println!();

    println!("Inverted:");
    for line in render_bitmap_8x8(invert_bitmap_8x8(bytes)) {
        println!("{line}");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const IMAGE: [&str; SIZE] = [
        "........",
        "########",
        ".......#",
        "......##",
        ".....###",
        "....####",
        "...#####",
        "..######",
    ];
    const BYTES: [u8; SIZE] = [
        0, 255, 1, 3, 7, 15, 31, 63
    ];

    #[test]
    fn test_parse_bitmap() {
        let result = parse_bitmap_8x8(IMAGE);
        assert_eq!(result, BYTES);
    }

    #[test]
    fn test_render_bitmap() {
        let result = render_bitmap_8x8(BYTES);
        assert_eq!(result, IMAGE);
    }

    #[test]
    fn test_invert_bitmap() {
        let result = invert_bitmap_8x8(BYTES);
        assert_eq!(result, [255, 0, 254, 252, 248, 240, 224, 192]);
    }

    #[test]
    fn test_bitmap_functions_end_to_end() {
        let bytes = parse_bitmap_8x8(IMAGE);
        let inverted_bytes = invert_bitmap_8x8(bytes);
        let output_string = render_bitmap_8x8(inverted_bytes);
        assert_eq!(output_string, [
            "########",
            "........",
            "#######.",
            "######..",
            "#####...",
            "####....",
            "###.....",
            "##......",
        ])
    }
}