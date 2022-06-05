// There are Two types of Integers
// 1.) Signed Integers [whole numbers that can be positive(+) or negative(-) e.g: -4, +4, -3, +3 ]
// 2.) Unsigned Integers [whole numbers with no sign, therefore can only be positive; e.g: 2,3, 4, 5]

// There are Six type of each
/*
Signed Integers Types: i8, i16, i32, i64, i128, isize
Unsigned Integers Types: u8, u16, u32, u64, u128, usize
*/

fn main() {
    // Examples
    let num: i32 = -40;
    let num_a: u32 = 34;
    
    //    /*
    //    {the `i` stands for integer and the `u` stands for unsigned integer}
    //    {the number after `i` and `u` stands for the number of bits of memory}
    //    */
    let num_b: i8 = 127; // stores numbers from -128 to +127
    let num_c: u8 = 255; // stores numbers from 0 to 255

    let num_d: i16 = -327; // stores numbers from -32768 to +32737
    let num_e: u16 = 65535; // stores numbers from 0 to 65535

    {/*To find out MAX and MIN of others, print this */}
    println!("Min i32 is {}", std::i32::MIN);
    println!("Max i32 is {}", std::i32::MAX);

    // _size: is based on local computers architecture, either 32bits or 64 bits
    // isize: either i32 or i64 (based on computer)
    // usize: either u32 or u64 (based on computer)

}
