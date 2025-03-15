struct AritmeticnoZaporedje {
    zacetni_clen: u32,
    razlika: u32,
}
use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

use AritmeticnoZaporedje as A;

impl A {
    fn new(a: u32, d: u32) -> A {
        return A {
            zacetni_clen: a,
            razlika: d,
        };
    }

    //fn n_th(zap: A, n: u32) -> u32 {}
}
// tole zgorej je zalost, da ni narejeno

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
