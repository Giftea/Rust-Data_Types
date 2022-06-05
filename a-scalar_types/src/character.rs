// This is a primitive alphabetic type
// It stores a single character
// Uses 4bytes memory space
// Uses single quotation '' instead of double quotes " "


fn main() {
    // Examples
    let char_a: char = 'a';
    let char_b: char = 'b';

    // It also represents more than just ASCII
    // Represents unicode scalar value
    let char_c: char = '国';
    let char_d: char = 'ℤ';
    let char_e: char = 'ñ';
    let char_f: char = '🙂'; // emojis
    let char_g: char = ' '; // supports single empty string
}