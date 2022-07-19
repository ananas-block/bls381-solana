
#![feature(test)]

extern crate test;
use {
    // bincode::{deserialize, serialize},
    // solana_sdk::{
    //     instruction::{AccountMeta, Instruction},
    //     message::{Message, SanitizedMessage},
    //     pubkey::{self, Pubkey},
    //     sysvar::instructions::{self, construct_instructions_data},
    // },
    std::convert::TryFrom,
    test::Bencher,
    solana_zk_token_sdk::curve25519::ristretto::*,

};
use solana_zk_token_sdk::curve25519::scalar::PodScalar;
// fn make_instructions() -> Vec<Instruction> {
//     let meta = AccountMeta::new(pubkey::new_rand(), false);
//     let inst = Instruction::new_with_bincode(pubkey::new_rand(), &[0; 10], vec![meta; 4]);
//     vec![inst; 4]
// }

// #[bench]
// fn bench_bincode_instruction_serialize(b: &mut Bencher) {
//     let instructions = make_instructions();
//     b.iter(|| {
//         test::black_box(serialize(&instructions).unwrap());
//     });
// }
//
// #[bench]
// fn bench_multiscalar_multiplication_edwards(b: &mut Bencher) {
//     let scalar = PodScalar([
//         205, 73, 127, 173, 83, 80, 190, 66, 202, 3, 237, 77, 52, 223, 238, 70, 80, 242, 24, 87,
//         111, 84, 49, 63, 194, 76, 202, 108, 62, 240, 83, 15,
//     ]);
//     let point = PodEdwardsPoint([
//         222, 174, 184, 139, 143, 122, 253, 96, 0, 207, 120, 157, 112, 38, 54, 189, 91, 144, 78,
//         111, 111, 122, 140, 183, 65, 250, 191, 133, 6, 42, 212, 93,
//     ]);
//     b.iter(|| {
//         let msm_product = multiscalar_multiply_edwards(&[scalar], &[point]).unwrap();
//     });
// }

#[bench]
fn bench_add_ristretto(b: &mut Bencher) {

    let point_a = PodRistrettoPoint([
        208, 165, 125, 204, 2, 100, 218, 17, 170, 194, 23, 9, 102, 156, 134, 136, 217, 190, 98,
        34, 183, 194, 228, 153, 92, 11, 108, 103, 28, 57, 88, 15,
    ]);
    let point_b = PodRistrettoPoint([
        208, 241, 72, 163, 73, 53, 32, 174, 54, 194, 71, 8, 70, 181, 244, 199, 93, 147, 99,
        231, 162, 127, 25, 40, 39, 19, 140, 132, 112, 212, 145, 108,
    ]);

    b.iter(|| {
        let msm_product = add_ristretto(&point_a, &point_b).unwrap();
    });
}

#[bench]
fn bench_subtract_ristretto(b: &mut Bencher) {
    let point_a = PodRistrettoPoint([
        208, 165, 125, 204, 2, 100, 218, 17, 170, 194, 23, 9, 102, 156, 134, 136, 217, 190, 98,
        34, 183, 194, 228, 153, 92, 11, 108, 103, 28, 57, 88, 15,
    ]);
    let point_b = PodRistrettoPoint([
        208, 241, 72, 163, 73, 53, 32, 174, 54, 194, 71, 8, 70, 181, 244, 199, 93, 147, 99,
        231, 162, 127, 25, 40, 39, 19, 140, 132, 112, 212, 145, 108,
    ]);

    b.iter(|| {
        let msm_product = subtract_ristretto(&point_a, &point_b).unwrap();
    });
}

#[bench]
fn bench_multiply_ristretto(b: &mut Bencher) {
    let scalar_x = PodScalar([
        254, 198, 23, 138, 67, 243, 184, 110, 236, 115, 236, 205, 205, 215, 79, 114, 45, 250,
        78, 137, 3, 107, 136, 237, 49, 126, 117, 223, 37, 191, 88, 6,
    ]);
    let point_a = PodRistrettoPoint([
        68, 80, 232, 181, 241, 77, 60, 81, 154, 51, 173, 35, 98, 234, 149, 37, 1, 39, 191, 201,
        193, 48, 88, 189, 97, 126, 63, 35, 144, 145, 203, 31,
    ]);

    b.iter(|| {
        let ax = multiply_ristretto(&scalar_x, &point_a).unwrap();
    });
}

#[bench]
fn bench_multiscalar_multiplication_ristretto(b: &mut Bencher) {
    let scalar = PodScalar([
        123, 108, 109, 66, 154, 185, 88, 122, 178, 43, 17, 154, 201, 223, 31, 238, 59, 215, 71,
        154, 215, 143, 177, 158, 9, 136, 32, 223, 139, 13, 133, 5,
    ]);
    let point = PodRistrettoPoint([
        158, 2, 130, 90, 148, 36, 172, 155, 86, 196, 74, 139, 30, 98, 44, 225, 155, 207, 135,
        111, 238, 167, 235, 67, 234, 125, 0, 227, 146, 31, 24, 113,
    ]);

    b.iter(|| {
        let msm_product = multiscalar_multiply_ristretto(&[scalar], &[point]).unwrap();
    });
}
