use digest::Digest;
use sha2::{Sha512};
use curve25519::{GeP3, ge_scalarmult_base, sc_reduce, sc_muladd};
use std::iter::range_step;

pub fn keypair(seed: &[u8]) -> ([u8, ..64], [u8, ..32]) {
    let mut secret: [u8, ..64] = {
        let mut hash_output: [u8, ..64] = [0, ..64];
        let mut hasher = Sha512::new();
        hasher.input(seed);
        hasher.result(hash_output.as_mut_slice());
        hash_output[0] &= 248;
        hash_output[31] &= 63;
        hash_output[31] |= 64;
        hash_output
    };

    let a = ge_scalarmult_base(secret.slice(0, 32));
    let public_key = a.to_bytes();
    for (dest, src) in secret.slice_mut(32,64).iter_mut().zip(public_key.iter()) {
        *dest = *src;
    }
    for (dest, src) in secret.slice_mut(0,32).iter_mut().zip(seed.iter()) {
        *dest = *src;
    }
    (secret, public_key) 
}

pub fn signature(message: &[u8], secret_key: &[u8]) -> [u8, ..64] {
    let seed = secret_key.slice(0, 32);
    let public_key = secret_key.slice(32, 64);
    let az: [u8, ..64] = {
        let mut hash_output: [u8, ..64] = [0, ..64];
        let mut hasher = Sha512::new();
        hasher.input(seed);
        hasher.result(hash_output.as_mut_slice());
        hash_output[0] &= 248;
        hash_output[31] &= 63;
        hash_output[31] |= 64;
        hash_output
    };

    let nonce = {
        let mut hash_output: [u8, ..64] = [0, ..64];
        let mut hasher = Sha512::new();
        hasher.input(az.slice(32, 64));
        hasher.input(message);
        hasher.result(hash_output.as_mut_slice());
        sc_reduce(hash_output.slice_mut(0, 32));
        hash_output
    };

    let mut signature: [u8, ..64] = [0, ..64];
    let r: GeP3 = ge_scalarmult_base(nonce.slice(0, 32));
    for (result_byte, source_byte) in signature.slice_mut(0, 32).iter_mut().zip(r.to_bytes().iter()) {
        *result_byte = *source_byte;
    }
    for (result_byte, source_byte) in signature.slice_mut(32, 64).iter_mut().zip(public_key.iter()) {
        *result_byte = *source_byte;
    }

    {
        let mut hasher = Sha512::new();
        hasher.input(signature.as_slice());
        hasher.input(message);
        let mut hram: [u8, ..64] = [0, ..64];
        hasher.result(hram.as_mut_slice());
        sc_reduce(hram.as_mut_slice());
        sc_muladd(signature.slice_mut(0, 32), hram.slice(0, 32), az.slice(0, 32), nonce.slice(0, 32));
    }

    signature
}
fn check_s_lt_l(s: &[u8]) -> bool
{
    let l: [u8, ..32] = 
      [ 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x14, 0xde, 0xf9, 0xde, 0xa2, 0xf7, 0x9c, 0xd6,
        0x58, 0x12, 0x63, 0x1a, 0x5c, 0xf5, 0xd3, 0xed ];
    let mut c: u8 = 0;
    let mut n: u8 = 1;

    for i in range_step(31, -1, -1) {
        c |= ((((s[i] as i32) - (l[i] as i32)) >> 8) as u8) & n;
        n &= (((((s[i] ^ l[i]) as i32)) - 1) >> 8) as u8;
    }

    return c == 0;
}

pub fn verify(message: &[u8], public_key: &[u8], signature: &[u8]) {
/*        crypto_hash_sha512_state hs;
    unsigned char h[64];
    unsigned char rcheck[32];
    unsigned int  i;
    unsigned char d = 0;
    ge_p3 A;
    ge_p2 R;

    if check_S_lt_l(sig + 32) {
        return false;
    }
    if (ge_frombytes_negate_vartime(&A, pk) != 0) {
        return false;
    }
    let mut d = 0;
    for pk_byte in public_key.iter() {
        d |= *pk_byte;
    }
    if d == 0 {
        return -1;
    }

    let mut hasher = Sha512::new();
    hasher.input(signature.slice(0, 32));
    hasher.input(public_key)
    hasher.input(message);
    let mut hash: [u8, ..64] = [0, ..64];
    hasher.result(hash);
    sc_reduce(hash);

    let R = ge_double_scalarmult_vartime(h, &A, sig.slice(32, 64));
    let rcheck = R.to_bytes();
    ge_tobytes(rcheck, &R);

    return crypto_verify_32(rcheck, sig) | (-(rcheck - sig == 0)) |
           sodium_memcmp(sig, rcheck, 32);
*/
}

mod tests {
    use ed25519::{keypair};

    fn do_keypair_case(seed: [u8, ..32], expected_secret: [u8, ..64], expected_public: [u8, ..32]) {
        let (actual_secret, actual_public) = keypair(seed.as_slice());
        assert_eq!(actual_secret.to_vec(), expected_secret.to_vec());
        assert_eq!(actual_public.to_vec(), expected_public.to_vec());

    }


    #[test] 
    fn keypair_cases() {
        do_keypair_case(
            [0x26, 0x27, 0xf6, 0x85, 0x97, 0x15, 0xad, 0x1d, 0xd2, 0x94, 0xdd, 0xc4, 0x76, 0x19, 0x39, 0x31,
             0xf1, 0xad, 0xb5, 0x58, 0xf0, 0x93, 0x97, 0x32, 0x19, 0x2b, 0xd1, 0xc0, 0xfd, 0x16, 0x8e, 0x4e],
            [0x26, 0x27, 0xf6, 0x85, 0x97, 0x15, 0xad, 0x1d, 0xd2, 0x94, 0xdd, 0xc4, 0x76, 0x19, 0x39, 0x31,
             0xf1, 0xad, 0xb5, 0x58, 0xf0, 0x93, 0x97, 0x32, 0x19, 0x2b, 0xd1, 0xc0, 0xfd, 0x16, 0x8e, 0x4e,
             0x5d, 0x6d, 0x23, 0x6b, 0x52, 0xd1, 0x8e, 0x3a, 0xb6, 0xd6, 0x07, 0x2f, 0xb6, 0xe4, 0xc7, 0xd4,
             0x6b, 0xd5, 0x9a, 0xd9, 0xcc, 0x19, 0x47, 0x26, 0x5f, 0x00, 0xb7, 0x20, 0xfa, 0x2c, 0x8f, 0x66],
            [0x5d, 0x6d, 0x23, 0x6b, 0x52, 0xd1, 0x8e, 0x3a, 0xb6, 0xd6, 0x07, 0x2f, 0xb6, 0xe4, 0xc7, 0xd4,
             0x6b, 0xd5, 0x9a, 0xd9, 0xcc, 0x19, 0x47, 0x26, 0x5f, 0x00, 0xb7, 0x20, 0xfa, 0x2c, 0x8f, 0x66]);
        do_keypair_case(
            [0x29, 0x23, 0xbe, 0x84, 0xe1, 0x6c, 0xd6, 0xae, 0x52, 0x90, 0x49, 0xf1, 0xf1, 0xbb, 0xe9, 0xeb,
             0xb3, 0xa6, 0xdb, 0x3c, 0x87, 0x0c, 0x3e, 0x99, 0x24, 0x5e, 0x0d, 0x1c, 0x06, 0xb7, 0x47, 0xde],
            [0x29, 0x23, 0xbe, 0x84, 0xe1, 0x6c, 0xd6, 0xae, 0x52, 0x90, 0x49, 0xf1, 0xf1, 0xbb, 0xe9, 0xeb,
             0xb3, 0xa6, 0xdb, 0x3c, 0x87, 0x0c, 0x3e, 0x99, 0x24, 0x5e, 0x0d, 0x1c, 0x06, 0xb7, 0x47, 0xde,
             0x5d, 0x83, 0x31, 0x26, 0x56, 0x0c, 0xb1, 0x9a, 0x14, 0x19, 0x37, 0x27, 0x78, 0x96, 0xf0, 0xfd,
             0x43, 0x7b, 0xa6, 0x80, 0x1e, 0xb2, 0x10, 0xac, 0x4c, 0x39, 0xd9, 0x00, 0x72, 0xd7, 0x0d, 0xa8],
            [0x5d, 0x83, 0x31, 0x26, 0x56, 0x0c, 0xb1, 0x9a, 0x14, 0x19, 0x37, 0x27, 0x78, 0x96, 0xf0, 0xfd,
             0x43, 0x7b, 0xa6, 0x80, 0x1e, 0xb2, 0x10, 0xac, 0x4c, 0x39, 0xd9, 0x00, 0x72, 0xd7, 0x0d, 0xa8]);
    }
}
