fn main() {

    //1. Declaring Integer Variables
    let a: i8 = 32;
    let b: i16 = 54;
    let c: i32 = 766;
    let d: i64 = 8966;
    let e: i128 = 9876656;

    let f: u8 = 233;
    let g: u16 = 1500;
    let h: u32 =5123455;
    let i: u64 = 64566545;
    let j: u128 = 76786546;

    println!("a = {}", {a},);
    println!("b = {}", {b},);
    println!("c = {}", {c},);
    println!("d = {}", {d},);
    println!("e = {}", {e},);
    println!("f = {}", {f},);
    println!("g = {}", {g},);
    println!("h = {}", {h},);
    println!("i = {}", {i},);
    println!("j= {} \n", {j},);

    //2. Conversion
    let decimal  = 12390;
    let dec_hex = format!("{:#x}", decimal);
    let dec_bin = format!("{:#b}", decimal);
    println!("Hexa = {} e Binário = {} \n", dec_hex, dec_bin);

    //3. Exploring Limits
    let mini8 =i8::MIN;
    let maxi8 =i8::MAX;
    println!("min_i8 = {}, max_i8 = {}", mini8, maxi8);

    let min_i16 = i16::MIN;
    let max_i16 = i16::MAX;
    println!("min_i16 = {}, max_i16 = {}", min_i16, max_i16);

    let min_i32 = i32::MIN;
    let max_i32 = i32::MAX;
    println!("min_i32 = {}, max_i32 = {}", min_i32, max_i32);

    let min_i64 = i64::MIN;
    let max_i64 = i64::MAX;
    println!("min_i64 = {}, max_i64 = {}", min_i64, max_i64);

    let min_128 = i128::MIN;
    let max_i128 = i128::MAX;
    println!("min_i128 = {}, max_i128 = {}", min_128, max_i128);

    let min_u8 = u8::MIN;
    let max_u8 = u8::MAX;
    println!("min_u8 = {}, max_u8 = {}", min_u8, max_u8);

    let min_u16 = u16::MIN;
    let max_u16 = u16::MAX;
    println!("min_u16 = {}, max_u16 = {}", min_u16, max_u16);

    let min_u32 = u32::MIN;
    let max_u32 = u32::MAX;
    println!("min_u32 = {}, max_u32 = {}", min_u32, max_u32);

    let min_u64 = u64::MIN;
    let max_u64 = u64::MAX;
    println!("min_u64 = {}, max_u64 = {}", min_u64, max_u64);

    let min_u128 = u128::MIN;
    let max_u128 = u128::MAX;
    println!("min_u128 = {}, max_u128 = {} \n", min_u128, max_u128);

    //let teste :i16 = 40000;
    // Error Code ->  literal out of range for `i16`
    // the literal `40000` does not fit into the type `i16` whose range is `-32768..=32767`
    //    = help: consider using the type `u16` instead
    //    = note: `#[deny(overflowing_literals)]` on by default
    // Este erro ocorre pois o valor 40000 ultrapassa a capacidade de bits que podem ser atribuídos à um tipo i16

    //4. Integer Operations
    let soma = a + a;
    println!("soma = {}", soma);

    let multi = b * b;
    println!("multiplicação = {}", multi);

    let x: u16 = 18000;
    let y: u16 = 200;

    let subtracao = x - y;
    println!("subtração = {}",subtracao);

    let divisao = x/y;
    println!("divisão = {}", divisao);

    let _z: u8 = 255;
    let _w: u8 = 1;
    //let over = z + w;  Overflow ocorre quando tentamos atribuir um número maior do que a capacidade daquele tipo suporta.


    let _p: u8 = 0;
    //let under = p - 1; Underflow acontece quando o resultado de uma operação fica abaixo do valor mínimo que o tipo pode suportar.

}
