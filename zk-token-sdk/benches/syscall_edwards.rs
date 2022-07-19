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
    solana_zk_token_sdk::curve25519::edwards::*,

};
use solana_zk_token_sdk::curve25519::scalar::PodScalar;

#[bench]
fn bench_edwards_addition(b: &mut Bencher) {
    let point_a = PodEdwardsPoint([
        33, 124, 71, 170, 117, 69, 151, 247, 59, 12, 95, 125, 133, 166, 64, 5, 2, 27, 90, 27,
        200, 167, 59, 164, 52, 54, 52, 200, 29, 13, 34, 213,
    ]);
    let point_b = PodEdwardsPoint([
        70, 222, 137, 221, 253, 204, 71, 51, 78, 8, 124, 1, 67, 200, 102, 225, 122, 228, 111,
        183, 129, 14, 131, 210, 212, 95, 109, 246, 55, 10, 159, 91,
    ]);

    b.iter(|| {
        add_edwards(&point_a, &point_b).unwrap();
    });
}

#[bench]
fn bench_edwards_multiplication(b: &mut Bencher) {
    let scalar_a = PodScalar([
        72, 191, 131, 55, 85, 86, 54, 60, 116, 10, 39, 130, 180, 3, 90, 227, 47, 228, 252, 99,
        151, 71, 118, 29, 34, 102, 117, 114, 120, 50, 57, 8,
    ]);
    let point_x = PodEdwardsPoint([
        176, 121, 6, 191, 108, 161, 206, 141, 73, 14, 235, 97, 49, 68, 48, 112, 98, 215, 145,
        208, 44, 188, 70, 10, 180, 124, 230, 15, 98, 165, 104, 85,
    ]);

    b.iter(|| {
        let ax = multiply_edwards(&scalar_a, &point_x).unwrap();
    });
}

#[bench]
fn test_multiscalar_multiplication_edwards(b: &mut Bencher) {
    let scalar = PodScalar([
        205, 73, 127, 173, 83, 80, 190, 66, 202, 3, 237, 77, 52, 223, 238, 70, 80, 242, 24, 87,
        111, 84, 49, 63, 194, 76, 202, 108, 62, 240, 83, 15,
    ]);
    let point = PodEdwardsPoint([
        222, 174, 184, 139, 143, 122, 253, 96, 0, 207, 120, 157, 112, 38, 54, 189, 91, 144, 78,
        111, 111, 122, 140, 183, 65, 250, 191, 133, 6, 42, 212, 93,
    ]);
    b.iter(|| {
        let msm_product = multiscalar_multiply_edwards(&[scalar], &[point]).unwrap();
    });
}
/*
#[test]
fn test_edwards_add_subtract() {
    // identity
    let identity = PodEdwardsPoint(EdwardsPoint::identity().compress().to_bytes());
    let point = PodEdwardsPoint([
        201, 179, 241, 122, 180, 185, 239, 50, 183, 52, 221, 0, 153, 195, 43, 18, 22, 38, 187,
        206, 179, 192, 210, 58, 53, 45, 150, 98, 89, 17, 158, 11,
    ]);

    assert_eq!(add_edwards(&point, &identity).unwrap(), point);
    assert_eq!(subtract_edwards(&point, &identity).unwrap(), point);

    // associativity
    let point_a = PodEdwardsPoint([
        33, 124, 71, 170, 117, 69, 151, 247, 59, 12, 95, 125, 133, 166, 64, 5, 2, 27, 90, 27,
        200, 167, 59, 164, 52, 54, 52, 200, 29, 13, 34, 213,
    ]);
    let point_b = PodEdwardsPoint([
        70, 222, 137, 221, 253, 204, 71, 51, 78, 8, 124, 1, 67, 200, 102, 225, 122, 228, 111,
        183, 129, 14, 131, 210, 212, 95, 109, 246, 55, 10, 159, 91,
    ]);
    let point_c = PodEdwardsPoint([
        72, 60, 66, 143, 59, 197, 111, 36, 181, 137, 25, 97, 157, 201, 247, 215, 123, 83, 220,
        250, 154, 150, 180, 192, 196, 28, 215, 137, 34, 247, 39, 129,
    ]);

    assert_eq!(
        add_edwards(&add_edwards(&point_a, &point_b).unwrap(), &point_c),
        add_edwards(&point_a, &add_edwards(&point_b, &point_c).unwrap()),
    );

    assert_eq!(
        subtract_edwards(&subtract_edwards(&point_a, &point_b).unwrap(), &point_c),
        subtract_edwards(&point_a, &add_edwards(&point_b, &point_c).unwrap()),
    );

    // commutativity
    assert_eq!(
        add_edwards(&point_a, &point_b).unwrap(),
        add_edwards(&point_b, &point_a).unwrap(),
    );

    // subtraction
    let point = PodEdwardsPoint(G.compress().to_bytes());
    let point_negated = PodEdwardsPoint((-G).compress().to_bytes());

    assert_eq!(point_negated, subtract_edwards(&identity, &point).unwrap(),)
}

#[test]
fn test_edwards_mul() {
    let scalar_a = PodScalar([
        72, 191, 131, 55, 85, 86, 54, 60, 116, 10, 39, 130, 180, 3, 90, 227, 47, 228, 252, 99,
        151, 71, 118, 29, 34, 102, 117, 114, 120, 50, 57, 8,
    ]);
    let point_x = PodEdwardsPoint([
        176, 121, 6, 191, 108, 161, 206, 141, 73, 14, 235, 97, 49, 68, 48, 112, 98, 215, 145,
        208, 44, 188, 70, 10, 180, 124, 230, 15, 98, 165, 104, 85,
    ]);
    let point_y = PodEdwardsPoint([
        174, 86, 89, 208, 236, 123, 223, 128, 75, 54, 228, 232, 220, 100, 205, 108, 237, 97,
        105, 79, 74, 192, 67, 224, 185, 23, 157, 116, 216, 151, 223, 81,
    ]);

    let ax = multiply_edwards(&scalar_a, &point_x).unwrap();
    let bx = multiply_edwards(&scalar_a, &point_y).unwrap();

    assert_eq!(
        add_edwards(&ax, &bx),
        multiply_edwards(&scalar_a, &add_edwards(&point_x, &point_y).unwrap()),
    );
}
*/
