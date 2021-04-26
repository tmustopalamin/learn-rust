fn if_statement() {
    // if statement di rust sama seperti dibahasa lainnya
    //hanya saja tempat kondisi/argument tidak ada kurung / parentheses

    let temp = 5;
    if temp > 30 {
        println!("Really Hot Outside");
    } else if temp < 10 {
        println!("Really Cold!")
    } else {
        println!("Temperature is OK!")
    }

    //di rust bisa langsung return nilai, tapi kedua return harus 1 tipe yg sama
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("Today is {}", day);

    println!(
        "is it {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );

    //inner if bisa menghasilkan return juga
    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hoy"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );
}

fn while_and_loop() {
    //while loop di rust sama seperti bahasa lainnnya
    //loop { } sama seperti while true (infinite loop)
    //continue (skip) , break (stop looping)

    let mut x = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x = {}", x);
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    //loop dimulai dari 1 sampai 11
    //continue dan break berlaku juga disini
    // 1..11 ini akan looping range antara 1 sampai 10 ( 1..11 == 1-10 )
    // dot dot (titik 2x) lalu nilai 11 adalah jika 2x dot (..) nilai yg akan dipakai adalah nilai yang dibawah 11 yaitu hanya sampai 10
    for x in 1..11 {
        println!("x = {}", x);
    }

    //loop dengan menentukan variabel index dan value
    for (index, val) in (30..41).enumerate() {
        println!("{} = {}", index, val);
    }
}

fn match_statement() {
    let country_code = 998;

    //match keyword dia akan mencari dan menenukan (match) country_code dengan beberapa nilai yang kita tau
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "Unknown", //range antara 1-999
        _ => "Invalid",       //diluar range 1..=9 === 1-999 akan masuk kesini
    };
    //yg dilakukan diatas adalah , match the country_code dengan nilai 44, kalo match dia akan menyimpan UK ke variabel country
    //_ (underscore) digunakan jika tidak ada yg match dia akan menyimpan nilai unknown ke variabel contry (modelnya sama seperti else)
    //range (..=) nilai terakhir 999 akan termasuk dalam range jadi rangenya 1-999

    println!("the country with code {} is {}", country_code, country);
}

fn main() {
    // if_statement();
    // while_and_loop();
    // for_loop();
    // match_statement();
}
