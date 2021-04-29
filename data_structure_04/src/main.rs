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

fn use_slice(slice: &mut [i32]) {
    //simbol dan & pada parameter diatas adalah dia menerima slice dan borowing bagian dari array i32
    //jika ingin mengubah nilai di diborow, tipe data parameter harus berupa "mut"

    println!("first element is = {}, length {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    //slice mengambil beberapa element dari array, dan slice length array tidak diketahui pada compile time
    let mut data = [1, 2, 3, 4, 5];

    //lempar sebagian element array ke function use_slice [1..4] berarti element 1-3
    //meminjam data tersebut untuk dikirim ke function use_slice
    use_slice(&mut data[1..4]); // kirim element 2-3
                                // use_slice(&mut data); // kirim seluruh element
    println!("{:?}", data);
}

fn strings() {
    //dirust string seperti ini valid, string kumpulan dari char
    // let s = "hello there";

    //2 tipe string di rust

    //1. s mereferensikan ke "static str"
    //&str adalah string slice (string dari potongan array)
    //static memberitahukan kita "hello there" adalah sebuah string dan akan termasuk didalam program kita (ketika kita mereferensikannya, kita referensikan ke memory yg ada di program kita)
    //jadi seperti kita tempel kedalam kode kita yang kita compile
    let s: &'static str = "hello there";
    //slice string ini tidak flexibel, kita ga bisa reassign
    // s = "abc";
    //dan juga tidak bisa memanipulasi individual character seperti contoh dibawah
    //let h = s[0]; //ga bisa

    //dan static str ini valid sequence dari UTF-8, jadi kita bisa akses melalui sequence of character
    for c in s.chars() {
        println!("{}", c);
    }
    //atau chars reverse, for c in s.chars() { println!("{}", c); }

    //cara yg aman untuk mendapatkan character
    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    //2.heap, String Construct, flexibel string dapat diubah di sort dll
    //membuat object string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        //push_str() berbeda dngn push(), dia menerima slice string dan dan menerima apapun yg kamu mau tambahkan
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    //cara conversi &str ke/dari String
    //dref conversion
    // let u: &str = &letters;

    //concatination
    // jika kita mau concat String dan str, ini mudah dan bekerja, contoh
    // let z = letters + "abc";
    //jika kita concat String dan String, kita harus lakukan dref conversion, dengam menambahkan &, contoh
    // let z = letters + &letters;

    //cara membuat String dari str, ada 2 cara
    // 1.
    // let mut abc = String::from("hello world");
    //2.
    let mut abc = "Hello World".to_string();

    //coba manipulate String
    abc.remove(0);
    abc.push_str("!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    //return tuple, sederhananya adalah sebuah collection dari nilai yang tipe datanya berbeda

    (x + y, x * y) //tuples, kita tidak bikin struct tuple melainkan yg sederhananya kita return dengan roundbracket dan typenya bisa berbeda (kalo array sama tipe data)
}

fn tuples() {
    //tuples adalah beberapa nilai diambil secara bersamaan
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("{:?}", sp);
    //akses element di tuple
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //descructuring, bongkar tuples ke beberapa variabel
    let (a, b) = sp;
    //tuple sp dibongkar ke variabel a dan b
    println!("a = {} , b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    //membuat tuple of tuple
    let combined = (sp2, sp);
    println!("{:?}", combined);
    //cara akses tuple of tuple
    println!("last elem = {}", (combined.0).1);

    //descructuring tuple of tuple
    let ((c, d), (e, f)) = combined;

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    //cara membuat tuple yg single dengan menambahkan 1 koma ,
    // let meaning = (42); //kalo ini membuat integer, hasilnya integer
    let one_tuple = (42,);
    println!("{:?}", one_tuple);
}

fn how_many(x: i32) -> &'static str {
    match x {
        //if x == 0
        0 => "no",

        //if x == 1 or x == 2
        1 | 2 => "one or two",

        //if x == 12
        12 => "dozen",

        //if x range dari 9 - 11
        //z @ adalah memberi alias pada range, dan alias/var tsb bisa diproses lagi
        // z @ 9..=11 => "lots",
        9..=11 => "lots",

        //if x apapun/selain case diatas, yg true jika x genap (habis dibagi 2)
        _ if (x % 2 == 0) => "some",

        //x apapun/selain case diatas  case
        _ => "few",
    }
}

fn pattern_matching() {
    for x in 0..13 {
        println!("{}: i have {} orange", x, how_many(x));
    }

    let point = (3, 4);

    match (point) {
        (0, 0) => println!("origin"),

        // jika point == (0, apapun) misal (0,5) dan assign ke var y
        // jika point == (apapun, 0) misal (5,0) dan assign ke var x
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),

        // jika point == (apapun, apapun) misal (3,4) dan assign ke var x dan y
        // (x, y) => println!("{},{}", x, y),

        //variabel bisa pass by reference dan bisa juga menjadikannya mutable dan merubah nilai
        // (ref x, y) => println!("{},{}", x, y),
        // (ref mut x, y) => println!("{},{}", x, y),

        // jika point == (apapun, 0) misal (apapun,apapun) dan assign hanya ke y , _ tidak akan menjadi variabel
        (_, y) => println!("?,{}", y),
    }

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
        // Color::RgbColor(0, 0, 0)
        // | Color::Cmyk {
        //     cyan: _,
        //     magenta: _,
        //     yellow: _,
        //     black: 255,
        // } => println!("black"),

        //pada contoh diatas kita akan mencari warna yg hanya memiliki nilai black, pattern matching bisa melakukan pattern hanya dengan nilai yg kita mau cari saja, misalkan black:255, dan hapus semua yang berisi underscore _ dan tambahkan , dan 2 dot (..), seperti contoh dibawah ini
        //2 dot (..) ini akan melengkapi seperti underscore
        Color::RgbColor(0, 0, 0) | Color::Cmyk { black: 255, .. } => println!("black"),

        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => (),
    }
}

struct PointGeneric<T> {
    //sebelum generic
    // x: f64,
    // y: f64,

    //generic
    x: T,
    y: T,
    //generic banyak tipe
    //struct PointGeneric<T,V>
    // x: T,
    // y: V,
}

struct LineGeneric<T> {
    start: PointGeneric<T>,
    end: PointGeneric<T>,
}

fn generics() {
    let a: PointGeneric<f64> = PointGeneric { x: 0.0, y: 4f64 };
    let b = PointGeneric { x: 1.2, y: 3.4 };

    //jika ingin explisit menambahkan type kita bisa menuliskannya, seperti dibawah
    //bisa i32, u32, i64 , u64, f64 dll
    //let a:Point<i32> = Point { x: 0, y: 0 };
    // let b:Point<f64> = Point { x: 1.2, y: 3.4 };

    //generic banyak tipe
    //let a:Point<u32, i32> = Point { x: 0, y: 0 };
    //let b:Point<f64,f64> = Point { x: 1.2, y: 3.4 };

    //lebih general genericnya, otomatis deteksi dari nilai yg diinputkan
    //let a:Point = Point { x: 0, y: 0 };
    //let b:Point = Point { x: 1.2, y: 3.4 };

    //pada contoh dibawah Point a bermasalah karena tidak 1 tipe, Point b sdh 1 tipe generic
    //yg kita lakukan adalah mengeneralisir type yg berbeda dengan menambahkan type setelah nilai
    //misal contoh diatas, nilai 4 (tidak bisa), agar bisa kita tambahkan type setelahnya menjadi 4f64
    let myLine = LineGeneric { start: a, end: b };
}

fn main() {
    // structures();
    // enums();
    // option();
    // arrays();
    // vector();
    // slices();
    // strings();
    // tuples();
    // pattern_matching();
    // generics();
}
