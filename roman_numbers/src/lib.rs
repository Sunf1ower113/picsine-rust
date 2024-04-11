use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> RomanDigit {
        match n {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => unreachable!()
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> RomanNumber {
        let mut  v:Vec<RomanDigit> = vec![];
        let mut nb = n;
        if nb >= 1000 {
            for i in 1..=nb/1000 {
                v.push(RomanDigit::from(1000));
            }
            nb -= (nb/1000) * 1000;
        }
        if nb >= 500 {
            v.push(RomanDigit::from(500));
            nb -= 500;
        }
        if nb >= 100 {
            for i in 1..=nb/100 {
                v.push(RomanDigit::from(100));
            }
            nb -= (nb/100) * 100;
        }
        if nb >= 50 {
            v.push(RomanDigit::from(50));
            nb -= 50;
        }
        if nb >= 10 {
            for i in 1..=nb/10 {
                v.push(RomanDigit::from(10));
            }
            nb -= (nb/10) * 10;
        }
        if nb == 9 {
            v.push(RomanDigit::from(1));
            v.push(RomanDigit::from(10));
            nb -= 9;
        }else if nb >= 5 {
            v.push(RomanDigit::from(5));
            nb -= 5;
        }else if nb >= 1{
            for i in 1..=nb {
                v.push(RomanDigit::from(1));
            }
        }else if nb == 0 {
            v.push(RomanDigit::from(0));
        }
        RomanNumber {
            0: v
        }
    }
}