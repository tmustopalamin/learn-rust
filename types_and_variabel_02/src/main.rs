//info -> keterangan
//practice -> mulai ngoding

use std::mem;

//info
//membuat constant variabel, variabel constant tidak ada address yg tetap di memory, ketika kita menggunakan variabel INI_CONSTANT dimana-mana/ dimain program ini akan mereplace variabel ke nilainya bkan address
//contoh println!("{}", INI_CONSTANT); di compile time INI_CONSTANT berubah menjadi 42
//practice
const INI_CONSTANT: u8 = 42;

//info
//static variabel tidak seperti constant dia akan disimpan ke memory dan memiliki alamat memory, dan ini immutable
//jika ingin mutable, tinggal tambahkan keyword "mut" setelah static keyword
//practice
// static Z: i32 = 123; //immutable
static mut Z: i32 = 123; //mutable
                         //info tambahan
                         //penggunaan mut harus dengan scope unsafe{ }, unsafe yg berarti kita akan berhati2 pada variabel tersebut, jangan sampai beberapa thread/proses merubah variabel tersebut secara langsung

fn fundamental_data_types() {
    //info
    // u -> unsigned integer (ga ada negatif > 0 ...)
    // i -> signed integer (normal ..-1 0 1 ..)
    //practice
    let a: u8 = 123; // 8 bit of integer

    // let b:i8 = 123;
    //info
    //{} -> akan mereplace argumen setelah komas
    //practice
    println!("a = {}", a);

    //info
    //variabel di rust immutable (tidak bisa diubah/replace) setelah assign pertama
    //practice
    // a = 256; <- tidak bisa di ganti

    //info
    //mutable variabel (bisa di reassign dan di ganti/replace) dengan menambahkan keyword "mut"
    //practice
    let mut c: i8 = 0; //mutable
    println!("c = {}", c);
    c = 42; //coba di ubah, harusnya tidak error
    println!("c = {}", c);

    //info
    //cara melihat bytes pada variabel
    //tambahkan function yg ada di std::mem (function dari rust), lalu kita import dan tulis di linepaling atas
    //lalu gunakan functionnya mem:size_of_val(&namavariabel)
    //practice
    let c = 123456789; //32-bit signed i32 (default integer pada rust)
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    //info tambahan
    //berbagai macam tipe bytes pada rust
    //i8 u8 i16 u16 i32 u32 i64 u64 (8,16,32,64 adalah besaran byte)

    //info
    //isize, usize adalah integral data type yang berhubungan dengan ukuran dari kata / ukuran dari pointer
    //pada practice dibawah kita mencari size dari variabel d dan mencari brapa bit os yg kita pakai
    //practice
    let d: isize = 123;
    let size_of_d = mem::size_of_val(&d);
    println!(
        "d = {}, takes up {} bytes, {}-bit OS",
        d,
        size_of_d,
        size_of_d * 8
    );

    //info
    //contoh assign type char, :char is optional, assign char bisa langsung tanpa menyebutkan typenya
    //practice
    // let e:char = 's'; //optional
    let e = 's';
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //info
    //contoh assign type double/floating point, :f32 is optional, assign f32 bisa langsung tanpa menyebutkan typenya, tapi lebih baik kasih type datanya
    //practice
    // let e:f32 = 's'; //optional
    let f: f32 = 2.5; //double-precision, 8 bytes or 64 bits, f64 (8bytes) f32 (4bytes)
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    //info
    //contoh assign type boolean,  assign boolean bisa langsung tanpa menyebutkan typenya, boolean berukuran 1 bytes
    //practice
    // let e:f32 = 's'; //optional
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
    //arithmatic
    let mut a = 2 + 3 * 4; // seperti biasa dimulai dari * lalu +
    println!("{}", a); //result 14

    //info
    //di rust tidak ada ++ dan --, untuk increase dan decrease bisa mnggunakan += dan -=
    //practice
    a = a + 1; //14 + 1 = 15
    a -= 2; //15 - 2 = 13

    //info
    //mencari nilai sisa menggunakan % (modulus)
    //practice
    println!("remainder of {} / {} = {}", a, 3, (a % 3)); // 13/3 = 1

    //info
    //mencari cubed (power)
    //practice
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed {}", a, a_cubed);

    let b = 2.5;
    //info
    //powi digunakan untuk power yang nilainya whole number (bukan double/float) contoh 3
    //powf digunakan untuk power yang nilainya double/float, contoh 3.14
    //practice
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //info
    //bitwise (hanya untuk integer)
    //seperti bahasa pemograman pada umumnya | OR, & AND, ^ XOR, ! NOR
    //practice
    let c = 1 | 2; //1 (01) , 2 (10), maka hasilnya adalah 11 == 3_10
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    //info
    // logical operator, sama seperti bahasa programman pada umumnya
    // > < => <= ==
    //practice
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("pi less than 4 = {}, x == {} ? {} ", pi_less_4, x, x_is_5);
}

fn scope_and_shadowing() {
    //info
    //variabel a live / ada dalam seluruh function scope_and_shadowing
    //practice
    let a = 777;

    //info
    //dirust kita bisa re-declare variabel yang sama
    //practice
    let a = 123;

    //info
    //kita bisa membuat scope dengan hanya { }, dan jika kita telah selesai dengan scope tersebut maka nilai yang ada di dalam memori tersebut tidak akan tersedia lagi
    {
        let b = 456;
        println!("inside, b = {}", b);

        //info
        //variabel a dia akan mereplace/shadowing variabel diluar scope
        //practice
        let a = 777;
        println!("inside, a = {}", a);
    }
    //info tambahan
    //diluar scope diatas variabel b tidak tersedia lagi
    // println!("b = {}", b); akan erro

    println!("outside a = {}", a);
}

fn constants() {
    //kodingan ada di global
    //lebih baik menggunakan const daripada static, jika contohnya kita hanya mau pakai pi

    println!("{}", INI_CONSTANT);

    // println!("{}", Z); //immutable
    unsafe {
        println!("{}", Z); //mutable
    }
}

fn stack_and_heap() {
    //info
    //stack, limited memory
    //assign variabel langsung/primitif datatype dan nilai return pada function akan masuk ke stack, dan setelah function berakhir maka semua data di stack akan di hapus.

    //heap, allocate untuk longterm memory menggunakan function Box pada rust
    let y = Box::new(10);
    //y juga tersimpan distack, tapi me-reference address memory yg ada diheap
    // 10 disimpan ke heap, tapi y distack, agar terkoneksi maka y menyimpan address dari nilai 10, jadi y tidak menyimpan nilai y melainkan hanya addressnya saja
    //cara untuk mendapatkan nilai 10 melalui y maka gunakan *y
    println!("y = {}", *y);

    //sama seperti data tipe vector dibawah
    // z ada di stack, tapi nilainya ada di heap
    // let z = vec![1, 2, 3];

    struct Point {
        x: f64,
        y: f64,
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    let p1 = origin();
    let p2 = Box::new(origin()); //boxing

    println!("p1 takes {} bytes", mem::size_of_val(&p1));
    println!("p2 takes {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("{}", p3.x);
}

fn main() {
    // fundamental_data_types();
    // operators();
    // scope_and_shadowing();
    // constants();
    stack_and_heap();
}
