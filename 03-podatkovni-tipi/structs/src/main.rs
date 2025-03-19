use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

#[derive(Copy)]
struct AritmeticnoZaporedje<T> {
    zacetni_clen: T,
    razlika: T,
}

use AritmeticnoZaporedje as A;

impl<T> A<T> {
    fn new(a: T, d: T) -> A<T> {
        return A {
            zacetni_clen: a,
            razlika: d,
        };
    }
}

impl<T> A<T>
where
    T: Sum<T> + Copy + Add<Output = T> + Mul<Output = T> + From<u32>,
{
    fn next(&mut self) -> T {
        let clen = self.zacetni_clen;
        self.zacetni_clen = self.zacetni_clen + self.razlika;
        clen
    }

    fn n_th(&self, n: u32) -> T {
        self.zacetni_clen + self.razlika * T::from(n - 1)
    }

    fn sum(&self, n: u32) -> T {
        let mut vsota = self.zacetni_clen;
        for i in 2..=n {
            vsota = vsota + self.n_th(i);
        }
        vsota
    }
}
trait Zaporedje<T> {
    fn name(&self) -> String;
    fn k_th(&self, k: u32) -> Option<T>;
    fn contains(&self, x: T) -> bool;
    fn start(&self) -> T;
}

struct Constant<T> {
    c: T,
}

impl<T> Constant<T> {
    fn new(c: T) -> Constant<T> {
        Constant { c }
    }
}

// impl<T> Zaporedje<T> for Constant<T> {
//     fn name(&self) -> String {
//         "Konstantno zaporedje".to_string()
//     }

//     fn k_th(&self, k: u32) -> Option<T> {
//         Some(self.c)
//     }

//     fn contains(&self, x: T) -> bool {
//         self.c == x
//     }

//     fn start(&self) -> T {
//         self.c
//     }
// }

impl Zaporedje<i64> for Constant<i64> {
    fn name(&self) -> String {
        "Konstantno zaporedje".to_string()
    }

    fn k_th(&self, k: u32) -> Option<i64> {
        Some(self.c)
    }

    fn contains(&self, x: i64) -> bool {
        self.c == x
    }

    fn start(&self) -> i64 {
        self.c
    }
}

struct Const {
    c: i64,
}

impl Const {
    fn new(c: i64) -> Const {
        Const { c }
    }
}

//naredimo AST
#[derive(Copy, Clone)]

enum BinOperacija {
    Plus,
    Minus,
    Times,
}
#[derive(Clone)]
enum Izraz<T> {
    Konstanta(T),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>),
}

impl<T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>> Izraz<T> {
    fn eval(izraz: &Self) -> T {
        match izraz {
            Izraz::Konstanta(k) => *k,
            Izraz::Operacija(levi, operacija, desni) => {
                let levi_val = Izraz::eval(levi);
                let desni_val = Izraz::eval(desni);
                match operacija {
                    BinOperacija::Plus => levi_val + desni_val,
                    BinOperacija::Minus => levi_val - desni_val,
                    BinOperacija::Times => levi_val * desni_val,
                }
            }
        }
    }
}

fn main() {
    let prvi_izraz = {
        Izraz::Operacija(
            Box::new(Izraz::Konstanta(1)),
            BinOperacija::Plus,
            Box::new(Izraz::Operacija(
                Box::new(Izraz::Konstanta(2)),
                BinOperacija::Times,
                Box::new(Izraz::Konstanta(3)),
            )),
        )
    };
    let vrednost = { Izraz::eval(&prvi_izraz) };
    println!("Vrednost izraza je: {}", vrednost);
}
