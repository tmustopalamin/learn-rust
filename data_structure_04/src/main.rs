use std::mem;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
}

enum Color {
    Red,
    Green,
    Blue,

    // enum bisa punya nilainya sendiri
    //tuple-like format
    RgbColor(u8, u8, u8),

    //enum bisa berbentuk seperti struct
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn enums() {
    //membuat variabel c dengan data type enum Color
    // let c: Color = Color::RgbColor(0, 20, 0);
    let c: Color = Color::Cmyk {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };

    // cari nilai c menggunakan match patttern
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),

        //match jika enum tuple-like RgbColor yang memiliki nilai 0,0,0 -> RgbColor(0,0,0)
        Color::RgbColor(0, 0, 0)
        | Color::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => (),
    }
}

fn option() {
    //option adalah tipe yang menunjukan ada atau tidaknya keberadaan dari nilai tertentu
    //contoh x dibagi y kalo nilainya ada ga masalah , tapi jika y nya 0 akan menjadi masalah (infinity), kasus tsb bisa dihindari oleh type Option<t>, daripada kita mengembalikan nilai hasil pembagian, disini kita akan menunjukan bahwa kita tidak mengembalikan hasil kalkulasi.
    //option pada kapanpun bisa 1 dari 2 state/nilai, option bisa berupa Some(z) atau None
    //Some(z), some of z adalah jawaban dan nilainya adalah z
    //None, berarti tidak ada jawaban
    let x = 3.0;
    let y = 2.0;

    //jika y tidak sama dengan 0.0 daripada kita kalkulasi langsung x/y, tapi kita return Some(x/y), else None
    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };
    println!("{:?}", result);

    //jika nilai y tidak sama dengan 0.0 hasilnya Some(x/y), dan jika nilai y == 0.0 maka hasilnya None

    //cara mendapatkan hasil dari variabel result
    //cara 1.bisa pakai match
    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y),
    }

    //cara 2. pakai if let / while let
    //if let /while menunjukan bahwa nilai itu ada atau tidak
    if let Some(z) = result {
        println!("z = {}", z);
    }
    //penjelasan
    //ketika result menghasilkan nilai Some atau None, kita destructed it (kita bongkar) dan dimasukan ke variabel "z" pada Some
    //jika pembongkaran terjadi/bekerja dan memberikan nilai ke Some, maka kita akan menjalankan/execute perintah yang ada di antara {  }
    //jika result ternyata None (Some(z) tidak sama dengan result -> None ), maka kita tidak akan menjalanakan/execute yg ada di antara { }
}

fn arrays() {
    //menyisipkan tipe data dan jumlah element di array rust itu optional, bisa langsung let a = [1,2,3,4,5];
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("a has {} element, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    //:? adalah print macro debug
    println!("{:?}", a);

    //syarat untuk bisa di compile pada if kedua element harus sama, jika elementnya tidak sama rust tidak bisa mencompilenya
    if a == [321, 2, 3, 4, 5] {
        println!("match");
    }

    //keterangan [1;10], 1 adalah nilai awal untuk variabel i, 10 itu besaran array
    //jika kita ingin mengatur besaran byte pada nilai awal kita tinggal tambahkan seperti contoh ini [1u16; 10] -> nilai awal 1 dan berupa integer unsigned 16bit
    let b = [1; 10]; //b.len() == 10
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    //multidimensi array
    //cara membuat array 3 x 2 [ [type; jumlah element], jumlah element ]
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);

    //cara akses multidimensi array
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn vector() {
    //vector seperti list di java, ketika kita tidak tau element dari list yg akan kita assign, kita bisa gunakan Vector
    //Vector harus mutable karena dia berubah2 nilainya disebabkan oleh function push pop dll
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    a.push(44);
    println!("a = {:?}", a);

    //data type index
    //usize isize , size adalah size dari memory sistem operasi mereka bisa 32bit atau 64bit
    //index tidak bisa memakai size i32 karena integer signed bisa saja negative, dan memory tidak boleh negative dan 32 tidak masuk akal jika kita menggunakan sistem 64bit
    //oleh karena itu maka index harus menggunakan usize, usize (unsigned size dari sistem kita dan tanpa nilai negative)

    let idx: usize = 0;

    //jika index lebih dari element Vector maka kita akan dapat error Panic/Crash the program
    //bagaimana cara menghindarinya? 1. kita bisa cek lengthnya terlebih dahulu, 2. menggunakan Option<T>

    //cara get nilai dari vector dan cara menghindar dari Panic menggunakan Option<T>
    //get function akan return Option dan untuk mencari nilai kita bisa gunakanan type option di matcher
    match a.get(3) {
        Some(x) => println!("a[0] = {}", x),
        None => println!("No such element!"),
    };

    //mengakses vector sama seperti mengakses array a[0]
    println!("a[0] = {}", a[idx]);

    //cara iterate vector
    for x in &a {
        println!("{}", x);
    }
    a.push(77);
    println!("{:?}", a);

    //removing element di vector menggunakan pop, removing dan return nilai yg di pop tsb ke variabel
    //tapi return berupa type Option
    let last_elem = a.pop();
    println!("last element is {:?}, a = {:?}", last_elem, a); // keluar Some(77) dan Vector a
                                                              //kita tidak bisa menggunakan let Some(last_elem) = a.pop();, kenapa? karena pop() mengeluarkan Some() atau None, jika kita lakukan seperti itu maka nilai None tidak bisa ditangani, syntax diatas hanya menangani Some() saja

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
    //penjelasan jika a.pop() mengeluarkan nilai Some() maka nilai dari Some() nya bisa di assign ke variabel x, contoh return Some(5) ini valid dan nilai dari Some(5) yaitu 5 diassign ke x, lalu di body x bisa di print
    //jika a.pop() mengeluarkan None maka tidak valid dan body tidak akan di execute
}

fn main() {
    // structures();
    // enums();
    // option();
    // arrays();
    vector();
}
