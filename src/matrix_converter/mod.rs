pub fn get_square_charset() -> [String;16] {
    return [
        String::from(" "),  //0000
        String::from("▗"), //0001
        String::from("▖"), //0010
        String::from("▄"),  //0011

        String::from("▝"), //0100
        String::from("▐"),  //0101
        String::from("▞"), //0110
        String::from("▟"), //0111

        String::from("▘"), //1000
        String::from("▚"), //1001
        String::from("▌"),  //1010
        String::from("▙"), //1011

        String::from("▀"),  //1100
        String::from("▜"), //1101
        String::from("▛"), //1110
        String::from("■"),  //1111
    ];
}

pub fn get_dots_charset() -> [String;16] {
    return [
        String::from(" "), //0000
        String::from("⠠"), //0001
        String::from("⠄"), //0010
        String::from("⠤"), //0011

        String::from("⠈"), //0100
        String::from("⠨"), //0101
        String::from("⠌"), //0110
        String::from("⠬"), //0111

        String::from("⠁"), //1000
        String::from("⠡"), //1001
        String::from("⠅"), //1010
        String::from("⠥"), //1011

        String::from("⠉"), //1100
        String::from("⢉"), //1101
        String::from("⡉"), //1110
        String::from("ⵘ"), //1111
    ];
}

/*
the total amount of possibilities for our matrix is 16 :
    0000
    0001
    0010
    0011

    0100
    0101
    0110
    0111

    1000
    1001
    1010
    1011

    1100
    1101
    1110
    1111
*/



//matrix is a square of 4 pixels
//[up-left, up-right, down-left, down-right]
//charset is an array of all char that can represent a matrix
//chars that correspond to a given matrix must be given following the binary order (see the list of possibilities)
pub fn convert_square(matrix: [u8; 4], charset: [String; 16]) -> String {
    let matrix_as_string: String = matrix.into_iter().map(|i| i.to_string()).collect::<String>();
    let matrix_to_decimal = usize::from_str_radix(&matrix_as_string, 2).expect("invalid matrix");
    return charset[matrix_to_decimal].to_string();
}

