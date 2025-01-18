use ark_ff::Field;
use num_bigint::BigInt;

use crate::gnark::element::*;
use crate::gnark::emparam::FieldParams;
use crate::gnark::emulated::field_bls12381::e2::GE2;
use crate::gnark::limbs::decompose;
use crate::gnark::limbs::recompose;
use expander_compiler::frontend::*;
use ark_ff::Zero;
use ark_bls12_381::Fq2;

pub fn nb_multiplication_res_limbs(len_left: usize, len_right: usize) -> usize {
    let res = len_left + len_right - 1;
    if len_left + len_right < 1 {
        0
    } else {
        res
    }
}

pub fn sub_padding(
    modulus: &BigInt,
    bits_per_limbs: u32,
    overflow: u32,
    nb_limbs: u32,
) -> Vec<BigInt> {
    if modulus == &BigInt::default() {
        panic!("modulus is zero");
    }
    let mut n_limbs = vec![BigInt::default(); nb_limbs as usize];
    for n_limb in &mut n_limbs {
        *n_limb = BigInt::from(1) << (overflow + bits_per_limbs);
    }
    let mut n = recompose(n_limbs.clone(), bits_per_limbs);
    n %= modulus;
    n = modulus - n;
    let mut pad = vec![BigInt::default(); nb_limbs as usize];
    if let Err(err) = decompose(&n, bits_per_limbs, &mut pad) {
        panic!("decompose: {}", err);
    }
    let mut new_pad = vec![BigInt::default(); nb_limbs as usize];
    for i in 0..pad.len() {
        new_pad[i] = pad[i].clone() + n_limbs[i].clone();
    }
    new_pad
}

pub fn get_sign(x: &Fq2) -> bool {
    let x_a0 = x.c0.to_string()
    .parse::<BigInt>()
    .expect("Invalid decimal string");
    let x_a1 = x.c1.to_string()
    .parse::<BigInt>()
    .expect("Invalid decimal string");
    let z = x_a0.is_zero();
    let sgn0 = !(x_a0 % 2u32).is_zero();
    let sgn1 = !(x_a1 % 2u32).is_zero();
    sgn0 | (z & sgn1)
}
pub fn has_sqrt(x: &Fq2) -> (Fq2, bool) {
    match x.sqrt() {
        Some(sqrt_x) => {
            (sqrt_x, true)
        }
        None => {
            (x.clone(), false)
        }
    }
} 
pub fn print_e2<C: Config, B: RootAPI<C>>(native: &mut B, v: &GE2) {
    for i in 0..48 {
        println!(
            "{}: {:?} {:?}",
            i,
            native.value_of(v.a0.limbs[i]),
            native.value_of(v.a1.limbs[i])
        );
    }
}
pub fn print_element<C: Config, B: RootAPI<C>, T: FieldParams>(native: &mut B, v: &Element<T>) {
    for i in 0..v.limbs.len() {
        print!("{:?} ", native.value_of(v.limbs[i]));
    }
    println!(" ");
}
