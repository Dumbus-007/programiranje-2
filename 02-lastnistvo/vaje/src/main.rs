use std::time::{Duration, Instant};

fn time_it<F: FnOnce() -> R, R>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn on_stack() {
    // Narišite shemo spreminjanja sklada in kopice
    // Za vsako vrstico napiši, kolikokrat se v pomnilniku pojavi 13?
    let mut a = [13; 100]; //100 13k na stacku
    let mut b = a; //a se skopira -> imamo 200 13k na stacku
    let q = String::from("13"); // 201 13k (ena je na kopici)
    println!("{}", q); //isto kot prej
    let r = q; //isto kot prej
    let p = &r; //isto kot prej
    a[0] = 1; // ena manj: 200 13k
    {
        let c = &b;
        println!("{}", c[0]); //izpiše 13
    }
    println!("{}", b[0]); //izpiše 13
    println!("{}", a[0]); //izpiše 1
    println!("{}", p); //izpiše 13
    println!("{}", r); //izpiše 13
                       // println!("{}", q); // Razloži, zakaj to ne deluje <<- q nima več lastništva
}

/// Napišite funkcijo `swap`, ki zamenja vrednosti dveh celoštevilskih spremenljivk.
fn swap(x: &mut i32, y: &mut i32) {
    let c = *x;
    *x = *y;
    *y = c;
}
fn test_swap() {
    // V spremenljivko `a` shranite vrednost 13, v spremenljivko `b` pa vrednost 42.
    let mut a = 13;
    let mut b = 42;
    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 13, b: 42`.

    // Naredite swap s pomočjo pomožne funkcije `swap`.
    swap(&mut a, &mut b);
    //

    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 42, b: 13`.
}

/// Popravite zakomentiran del spodnje funkcije, da bo deloval
fn str_own() {
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}

/// Popravite brez uporabe funkcije `clone`
/// Namig: sklad in kopiranje na skladu - kodo lahko spremenite
fn str_own2() {
    let x = (1, 2, (), String::from("Hello world"));
    let y = &x;
    println!("{:?}, {:?}", x, y);
}

/// Popravite spodnji dve funkciji, da bosta delovali

fn wrong() {
    let s = String::from("Hello World");
    print_str(&s);
    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala
fn fn1() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala

fn fn2() {
    let x = Box::new(5);

    let mut y = Box::new(42);

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------

fn fn3() {
    let t = (
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    );

    let _s = t.1;

    // Izpišite čim večji del t-ja.
    println!("{}{}", t.0, t.2);
}

/// ------------------------------------------------------------------------------------------------

fn fn4() {
    let x = 5;
    println!("{:p}", &x)
}

/// ------------------------------------------------------------------------------------------------

fn fn5() {
    let x = 13;
    let y = &x;

    // Popravite spodnjo vrstico, da bo bo enakost držala
    assert_eq!(13, *y);
}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper` se mora poklicati čim bolj učinkovito.
fn fn6() {
    let mut s = String::from("hello, ");

    helper(&s);

    println!("Success!");
}

// Te funkcije ne spreminjajte
fn helper(s: &String) {}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper2` se mora poklicati čim bolj učinkovito.
fn fn7() {
    let mut s = String::from("hello, ");

    helper2(&mut s);

    println!("Success!");
}
// Te funkcije ne spreminjajte
fn helper2(s: &mut String) {
    s.push_str("world")
}

/// ------------------------------------------------------------------------------------------------

/// Pojasnite, zakaj spodnja koda ne deluje
fn fn8() {
    // let mut s = String::from("hello, ");

    // let p = &mut s;

    // p.push_str("world");

    // println!("Success! {}", p);
    // println!("Success! {}", s); <- tle je težava, ker si ga sposojamo kot immutable al neki
    // p.push_str("!");
}

/// ------------------------------------------------------------------------------------------------
/// Pojasnite, zakaj spodnja koda ne deluje in jo popravite
/// Pojasnite tudi zakaj je popravek ok

fn fn9() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s; //<- ne smemo si 2x sposodit mutable

    println!("{}, {}", r1, r2);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
fn fn10() {
    // Popravite spodnjo vrstico
    let mut s = String::from("hello, ");

    helper3(&mut s);

    println!("Success!");
}

fn helper3(s: &mut String) {}

/// ------------------------------------------------------------------------------------------------

fn main() {
    fn10()
}
