use std::fmt;
use std::cmp::{Eq, PartialEq};
/*
fe means field element.
Here the field is \Z/(2^255-19).
An element t, entries t[0]...t[9], represents the integer
t[0]+2^26 t[1]+2^51 t[2]+2^77 t[3]+2^102 t[4]+...+2^230 t[9].
Bounds on each t[i] vary depending on context.
*/

pub struct Fe([i32, ..10]);

impl fmt::Show for Fe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Fe(elems) = self;
        write!(f, "Fe({})", elems.to_vec())
    }
}

impl PartialEq for Fe {
    fn eq(&self, other: &Fe) -> bool {
        let &Fe(self_elems) = self;
        let &Fe(other_elems) = other;
        self_elems.to_vec() == other_elems.to_vec()
    }
}
impl Eq for Fe { }

static FE_ZERO : Fe = Fe([0,0,0,0,0,0,0,0,0,0]);
static FE_ONE : Fe = Fe([1,0,0,0,0,0,0,0,0,0]);

fn load_4u(s: &[u8]) -> u64 {
    return (s[0] as u64)
        | ((s[1] as u64)<<8)
        | ((s[2] as u64)<<16)
        | ((s[3] as u64)<<24);
}
fn load_4i(s: &[u8]) -> i64 {
    return load_4u(s) as i64;
}
fn load_3u(s: &[u8]) -> u64 {
    return (s[0] as u64)
        | ((s[1] as u64)<<8)
        | ((s[2] as u64)<<16);
}
fn load_3i(s: &[u8]) -> i64 {
    return load_3u(s) as i64;
}

impl Add<Fe, Fe> for Fe {
    /*
    h = f + g
    Can overlap h with f or g.

    Preconditions:
       |f| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
       |g| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.

    Postconditions:
       |h| bounded by 1.1*2^26,1.1*2^25,1.1*2^26,1.1*2^25,etc.
    */
    fn add(&self, _rhs: &Fe) -> Fe {
        let &Fe(f) = self;
        let &Fe(g) = _rhs;

        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let g0 = g[0];
        let g1 = g[1];
        let g2 = g[2];
        let g3 = g[3];
        let g4 = g[4];
        let g5 = g[5];
        let g6 = g[6];
        let g7 = g[7];
        let g8 = g[8];
        let g9 = g[9];
        let h0 = f0 + g0;
        let h1 = f1 + g1;
        let h2 = f2 + g2;
        let h3 = f3 + g3;
        let h4 = f4 + g4;
        let h5 = f5 + g5;
        let h6 = f6 + g6;
        let h7 = f7 + g7;
        let h8 = f8 + g8;
        let h9 = f9 + g9;
        Fe([h0, h1, h2, h3, h4, h5, h6, h7, h8, h9])
    }
}

impl Sub<Fe, Fe> for Fe {
    /*
    h = f - g
    Can overlap h with f or g.

    Preconditions:
       |f| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
       |g| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.

    Postconditions:
       |h| bounded by 1.1*2^26,1.1*2^25,1.1*2^26,1.1*2^25,etc.
    */
    fn sub(&self, _rhs: &Fe) -> Fe {
        let &Fe(f) = self;
        let &Fe(g) = _rhs;

        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let g0 = g[0];
        let g1 = g[1];
        let g2 = g[2];
        let g3 = g[3];
        let g4 = g[4];
        let g5 = g[5];
        let g6 = g[6];
        let g7 = g[7];
        let g8 = g[8];
        let g9 = g[9];
        let h0 = f0 - g0;
        let h1 = f1 - g1;
        let h2 = f2 - g2;
        let h3 = f3 - g3;
        let h4 = f4 - g4;
        let h5 = f5 - g5;
        let h6 = f6 - g6;
        let h7 = f7 - g7;
        let h8 = f8 - g8;
        let h9 = f9 - g9;
        Fe([h0, h1, h2, h3, h4, h5, h6, h7, h8, h9])
    }
}

impl Mul<Fe, Fe> for Fe {
    /*
    h = f * g
    Can overlap h with f or g.

    Preconditions:
       |f| bounded by 1.1*2^26,1.1*2^25,1.1*2^26,1.1*2^25,etc.
       |g| bounded by 1.1*2^26,1.1*2^25,1.1*2^26,1.1*2^25,etc.

    Postconditions:
       |h| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
    */

    /*
    Notes on implementation strategy:

    Using schoolbook multiplication.
    Karatsuba would save a little in some cost models.

    Most multiplications by 2 and 19 are 32-bit precomputations;
    cheaper than 64-bit postcomputations.

    There is one remaining multiplication by 19 in the carry chain;
    one *19 precomputation can be merged into this,
    but the resulting data flow is considerably less clean.

    There are 12 carries below.
    10 of them are 2-way parallelizable and vectorizable.
    Can get away with 11 carries, but then data flow is much deeper.

    With tighter constraints on inputs can squeeze carries into int32.
    */

    fn mul(&self, _rhs: &Fe) -> Fe {
        let &Fe(f) = self;
        let &Fe(g) = _rhs;
        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let g0 = g[0];
        let g1 = g[1];
        let g2 = g[2];
        let g3 = g[3];
        let g4 = g[4];
        let g5 = g[5];
        let g6 = g[6];
        let g7 = g[7];
        let g8 = g[8];
        let g9 = g[9];
        let g1_19 = 19 * g1; /* 1.4*2^29 */
        let g2_19 = 19 * g2; /* 1.4*2^30; still ok */
        let g3_19 = 19 * g3;
        let g4_19 = 19 * g4;
        let g5_19 = 19 * g5;
        let g6_19 = 19 * g6;
        let g7_19 = 19 * g7;
        let g8_19 = 19 * g8;
        let g9_19 = 19 * g9;
        let f1_2 = 2 * f1;
        let f3_2 = 2 * f3;
        let f5_2 = 2 * f5;
        let f7_2 = 2 * f7;
        let f9_2 = 2 * f9;
        let f0g0    = (f0   as i64) * (g0 as i64);
        let f0g1    = (f0   as i64) * (g1 as i64);
        let f0g2    = (f0   as i64) * (g2 as i64);
        let f0g3    = (f0   as i64) * (g3 as i64);
        let f0g4    = (f0   as i64) * (g4 as i64);
        let f0g5    = (f0   as i64) * (g5 as i64);
        let f0g6    = (f0   as i64) * (g6 as i64);
        let f0g7    = (f0   as i64) * (g7 as i64);
        let f0g8    = (f0   as i64) * (g8 as i64);
        let f0g9    = (f0   as i64) * (g9 as i64);
        let f1g0    = (f1   as i64) * (g0 as i64);
        let f1g1_2  = (f1_2 as i64) * (g1 as i64);
        let f1g2    = (f1   as i64) * (g2 as i64);
        let f1g3_2  = (f1_2 as i64) * (g3 as i64);
        let f1g4    = (f1   as i64) * (g4 as i64);
        let f1g5_2  = (f1_2 as i64) * (g5 as i64);
        let f1g6    = (f1   as i64) * (g6 as i64);
        let f1g7_2  = (f1_2 as i64) * (g7 as i64);
        let f1g8    = (f1   as i64) * (g8 as i64);
        let f1g9_38 = (f1_2 as i64) * (g9_19 as i64);
        let f2g0    = (f2   as i64) * (g0 as i64);
        let f2g1    = (f2   as i64) * (g1 as i64);
        let f2g2    = (f2   as i64) * (g2 as i64);
        let f2g3    = (f2   as i64) * (g3 as i64);
        let f2g4    = (f2   as i64) * (g4 as i64);
        let f2g5    = (f2   as i64) * (g5 as i64);
        let f2g6    = (f2   as i64) * (g6 as i64);
        let f2g7    = (f2   as i64) * (g7 as i64);
        let f2g8_19 = (f2   as i64) * (g8_19 as i64);
        let f2g9_19 = (f2   as i64) * (g9_19 as i64);
        let f3g0    = (f3   as i64) * (g0 as i64);
        let f3g1_2  = (f3_2 as i64) * (g1 as i64);
        let f3g2    = (f3   as i64) * (g2 as i64);
        let f3g3_2  = (f3_2 as i64) * (g3 as i64);
        let f3g4    = (f3   as i64) * (g4 as i64);
        let f3g5_2  = (f3_2 as i64) * (g5 as i64);
        let f3g6    = (f3   as i64) * (g6 as i64);
        let f3g7_38 = (f3_2 as i64) * (g7_19 as i64);
        let f3g8_19 = (f3   as i64) * (g8_19 as i64);
        let f3g9_38 = (f3_2 as i64) * (g9_19 as i64);
        let f4g0    = (f4   as i64) * (g0 as i64);
        let f4g1    = (f4   as i64) * (g1 as i64);
        let f4g2    = (f4   as i64) * (g2 as i64);
        let f4g3    = (f4   as i64) * (g3 as i64);
        let f4g4    = (f4   as i64) * (g4 as i64);
        let f4g5    = (f4   as i64) * (g5 as i64);
        let f4g6_19 = (f4   as i64) * (g6_19 as i64);
        let f4g7_19 = (f4   as i64) * (g7_19 as i64);
        let f4g8_19 = (f4   as i64) * (g8_19 as i64);
        let f4g9_19 = (f4   as i64) * (g9_19 as i64);
        let f5g0    = (f5   as i64) * (g0 as i64);
        let f5g1_2  = (f5_2 as i64) * (g1 as i64);
        let f5g2    = (f5   as i64) * (g2 as i64);
        let f5g3_2  = (f5_2 as i64) * (g3 as i64);
        let f5g4    = (f5   as i64) * (g4 as i64);
        let f5g5_38 = (f5_2 as i64) * (g5_19 as i64);
        let f5g6_19 = (f5   as i64) * (g6_19 as i64);
        let f5g7_38 = (f5_2 as i64) * (g7_19 as i64);
        let f5g8_19 = (f5   as i64) * (g8_19 as i64);
        let f5g9_38 = (f5_2 as i64) * (g9_19 as i64);
        let f6g0    = (f6   as i64) * (g0 as i64);
        let f6g1    = (f6   as i64) * (g1 as i64);
        let f6g2    = (f6   as i64) * (g2 as i64);
        let f6g3    = (f6   as i64) * (g3 as i64);
        let f6g4_19 = (f6   as i64) * (g4_19 as i64);
        let f6g5_19 = (f6   as i64) * (g5_19 as i64);
        let f6g6_19 = (f6   as i64) * (g6_19 as i64);
        let f6g7_19 = (f6   as i64) * (g7_19 as i64);
        let f6g8_19 = (f6   as i64) * (g8_19 as i64);
        let f6g9_19 = (f6   as i64) * (g9_19 as i64);
        let f7g0    = (f7   as i64) * (g0 as i64);
        let f7g1_2  = (f7_2 as i64) * (g1 as i64);
        let f7g2    = (f7   as i64) * (g2 as i64);
        let f7g3_38 = (f7_2 as i64) * (g3_19 as i64);
        let f7g4_19 = (f7   as i64) * (g4_19 as i64);
        let f7g5_38 = (f7_2 as i64) * (g5_19 as i64);
        let f7g6_19 = (f7   as i64) * (g6_19 as i64);
        let f7g7_38 = (f7_2 as i64) * (g7_19 as i64);
        let f7g8_19 = (f7   as i64) * (g8_19 as i64);
        let f7g9_38 = (f7_2 as i64) * (g9_19 as i64);
        let f8g0    = (f8   as i64) * (g0 as i64);
        let f8g1    = (f8   as i64) * (g1 as i64);
        let f8g2_19 = (f8   as i64) * (g2_19 as i64);
        let f8g3_19 = (f8   as i64) * (g3_19 as i64);
        let f8g4_19 = (f8   as i64) * (g4_19 as i64);
        let f8g5_19 = (f8   as i64) * (g5_19 as i64);
        let f8g6_19 = (f8   as i64) * (g6_19 as i64);
        let f8g7_19 = (f8   as i64) * (g7_19 as i64);
        let f8g8_19 = (f8   as i64) * (g8_19 as i64);
        let f8g9_19 = (f8   as i64) * (g9_19 as i64);
        let f9g0    = (f9   as i64) * (g0 as i64);
        let f9g1_38 = (f9_2 as i64) * (g1_19 as i64);
        let f9g2_19 = (f9   as i64) * (g2_19 as i64);
        let f9g3_38 = (f9_2 as i64) * (g3_19 as i64);
        let f9g4_19 = (f9   as i64) * (g4_19 as i64);
        let f9g5_38 = (f9_2 as i64) * (g5_19 as i64);
        let f9g6_19 = (f9   as i64) * (g6_19 as i64);
        let f9g7_38 = (f9_2 as i64) * (g7_19 as i64);
        let f9g8_19 = (f9   as i64) * (g8_19 as i64);
        let f9g9_38 = (f9_2 as i64) * (g9_19 as i64);
        let mut h0 = f0g0+f1g9_38+f2g8_19+f3g7_38+f4g6_19+f5g5_38+f6g4_19+f7g3_38+f8g2_19+f9g1_38;
        let mut h1 = f0g1+f1g0   +f2g9_19+f3g8_19+f4g7_19+f5g6_19+f6g5_19+f7g4_19+f8g3_19+f9g2_19;
        let mut h2 = f0g2+f1g1_2 +f2g0   +f3g9_38+f4g8_19+f5g7_38+f6g6_19+f7g5_38+f8g4_19+f9g3_38;
        let mut h3 = f0g3+f1g2   +f2g1   +f3g0   +f4g9_19+f5g8_19+f6g7_19+f7g6_19+f8g5_19+f9g4_19;
        let mut h4 = f0g4+f1g3_2 +f2g2   +f3g1_2 +f4g0   +f5g9_38+f6g8_19+f7g7_38+f8g6_19+f9g5_38;
        let mut h5 = f0g5+f1g4   +f2g3   +f3g2   +f4g1   +f5g0   +f6g9_19+f7g8_19+f8g7_19+f9g6_19;
        let mut h6 = f0g6+f1g5_2 +f2g4   +f3g3_2 +f4g2   +f5g1_2 +f6g0   +f7g9_38+f8g8_19+f9g7_38;
        let mut h7 = f0g7+f1g6   +f2g5   +f3g4   +f4g3   +f5g2   +f6g1   +f7g0   +f8g9_19+f9g8_19;
        let mut h8 = f0g8+f1g7_2 +f2g6   +f3g5_2 +f4g4   +f5g3_2 +f6g2   +f7g1_2 +f8g0   +f9g9_38;
        let mut h9 = f0g9+f1g8   +f2g7   +f3g6   +f4g5   +f5g4   +f6g3   +f7g2   +f8g1   +f9g0   ;
        let mut carry0;
        let mut carry1;
        let mut carry2;
        let mut carry3;
        let mut carry4;
        let mut carry5;
        let mut carry6;
        let mut carry7;
        let mut carry8;
        let mut carry9;

        /*
        |h0| <= (1.1*1.1*2^52*(1+19+19+19+19)+1.1*1.1*2^50*(38+38+38+38+38))
          i.e. |h0| <= 1.2*2^59; narrower ranges for h2, h4, h6, h8
        |h1| <= (1.1*1.1*2^51*(1+1+19+19+19+19+19+19+19+19))
          i.e. |h1| <= 1.5*2^58; narrower ranges for h3, h5, h7, h9
        */

        carry0 = (h0 + (1<<25)) >> 26; h1 += carry0; h0 -= carry0 << 26;
        carry4 = (h4 + (1<<25)) >> 26; h5 += carry4; h4 -= carry4 << 26;
        /* |h0| <= 2^25 */
        /* |h4| <= 2^25 */
        /* |h1| <= 1.51*2^58 */
        /* |h5| <= 1.51*2^58 */

        carry1 = (h1 + (1<<24)) >> 25; h2 += carry1; h1 -= carry1 << 25;
        carry5 = (h5 + (1<<24)) >> 25; h6 += carry5; h5 -= carry5 << 25;
        /* |h1| <= 2^24; from now on fits into int32 */
        /* |h5| <= 2^24; from now on fits into int32 */
        /* |h2| <= 1.21*2^59 */
        /* |h6| <= 1.21*2^59 */

        carry2 = (h2 + (1<<25)) >> 26; h3 += carry2; h2 -= carry2 << 26;
        carry6 = (h6 + (1<<25)) >> 26; h7 += carry6; h6 -= carry6 << 26;
        /* |h2| <= 2^25; from now on fits into int32 unchanged */
        /* |h6| <= 2^25; from now on fits into int32 unchanged */
        /* |h3| <= 1.51*2^58 */
        /* |h7| <= 1.51*2^58 */

        carry3 = (h3 + (1<<24)) >> 25; h4 += carry3; h3 -= carry3 << 25;
        carry7 = (h7 + (1<<24)) >> 25; h8 += carry7; h7 -= carry7 << 25;
        /* |h3| <= 2^24; from now on fits into int32 unchanged */
        /* |h7| <= 2^24; from now on fits into int32 unchanged */
        /* |h4| <= 1.52*2^33 */
        /* |h8| <= 1.52*2^33 */

        carry4 = (h4 + (1<<25)) >> 26; h5 += carry4; h4 -= carry4 << 26;
        carry8 = (h8 + (1<<25)) >> 26; h9 += carry8; h8 -= carry8 << 26;
        /* |h4| <= 2^25; from now on fits into int32 unchanged */
        /* |h8| <= 2^25; from now on fits into int32 unchanged */
        /* |h5| <= 1.01*2^24 */
        /* |h9| <= 1.51*2^58 */

        carry9 = (h9 + (1<<24)) >> 25; h0 += carry9 * 19; h9 -= carry9 << 25;
        /* |h9| <= 2^24; from now on fits into int32 unchanged */
        /* |h0| <= 1.8*2^37 */

        carry0 = (h0 + (1<<25)) >> 26; h1 += carry0; h0 -= carry0 << 26;
        /* |h0| <= 2^25; from now on fits into int32 unchanged */
        /* |h1| <= 1.01*2^24 */

        Fe([h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32,
            h5 as i32, h6 as i32, h7 as i32, h8 as i32, h9 as i32])
    }
}

impl Fe {
    pub fn from_bytes(s: &[u8]) -> Fe {
        let mut h0 = load_4i(s.slice(0,4));
        let mut h1 = load_3i(s.slice(4,7)) << 6;
        let mut h2 = load_3i(s.slice(7,10)) << 5;
        let mut h3 = load_3i(s.slice(10,13)) << 3;
        let mut h4 = load_3i(s.slice(13,16)) << 2;
        let mut h5 = load_4i(s.slice(16,20));
        let mut h6 = load_3i(s.slice(20,23)) << 7;
        let mut h7 = load_3i(s.slice(23,26)) << 5;
        let mut h8 = load_3i(s.slice(26,29)) << 4;
        let mut h9 = (load_3i(s.slice(29,32)) & 8388607) << 2;

        let carry9 = (h9 + (1<<24)) >> 25; h0 += carry9 * 19; h9 -= carry9 << 25;
        let carry1 = (h1 + (1<<24)) >> 25; h2 += carry1; h1 -= carry1 << 25;
        let carry3 = (h3 + (1<<24)) >> 25; h4 += carry3; h3 -= carry3 << 25;
        let carry5 = (h5 + (1<<24)) >> 25; h6 += carry5; h5 -= carry5 << 25;
        let carry7 = (h7 + (1<<24)) >> 25; h8 += carry7; h7 -= carry7 << 25;

        let carry0 = (h0 + (1<<25)) >> 26; h1 += carry0; h0 -= carry0 << 26;
        let carry2 = (h2 + (1<<25)) >> 26; h3 += carry2; h2 -= carry2 << 26;
        let carry4 = (h4 + (1<<25)) >> 26; h5 += carry4; h4 -= carry4 << 26;
        let carry6 = (h6 + (1<<25)) >> 26; h7 += carry6; h6 -= carry6 << 26;
        let carry8 = (h8 + (1<<25)) >> 26; h9 += carry8; h8 -= carry8 << 26;

        Fe([h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32,
            h5 as i32, h6 as i32, h7 as i32, h8 as i32, h9 as i32])
    }

    /*
    Preconditions:
      |h| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.

    Write p=2^255-19; q=floor(h/p).
    Basic claim: q = floor(2^(-255)(h + 19 2^(-25)h9 + 2^(-1))).

    Proof:
      Have |h|<=p so |q|<=1 so |19^2 2^(-255) q|<1/4.
      Also have |h-2^230 h9|<2^230 so |19 2^(-255)(h-2^230 h9)|<1/4.

      Write y=2^(-1)-19^2 2^(-255)q-19 2^(-255)(h-2^230 h9).
      Then 0<y<1.

      Write r=h-pq.
      Have 0<=r<=p-1=2^255-20.
      Thus 0<=r+19(2^-255)r<r+19(2^-255)2^255<=2^255-1.

      Write x=r+19(2^-255)r+y.
      Then 0<x<2^255 so floor(2^(-255)x) = 0 so floor(q+2^(-255)x) = q.

      Have q+2^(-255)x = 2^(-255)(h + 19 2^(-25) h9 + 2^(-1))
      so floor(2^(-255)(h + 19 2^(-25) h9 + 2^(-1))) = q.
    */

    pub fn to_bytes(&self) -> [u8, ..32] {
        let &Fe(es) = self;
        let mut h0 = es[0];
        let mut h1 = es[1];
        let mut h2 = es[2];
        let mut h3 = es[3];
        let mut h4 = es[4];
        let mut h5 = es[5];
        let mut h6 = es[6];
        let mut h7 = es[7];
        let mut h8 = es[8];
        let mut h9 = es[9];
        let mut q;

        q = (19 * h9 + (1 << 24)) >> 25;
        q = (h0 + q) >> 26;
        q = (h1 + q) >> 25;
        q = (h2 + q) >> 26;
        q = (h3 + q) >> 25;
        q = (h4 + q) >> 26;
        q = (h5 + q) >> 25;
        q = (h6 + q) >> 26;
        q = (h7 + q) >> 25;
        q = (h8 + q) >> 26;
        q = (h9 + q) >> 25;

        /* Goal: Output h-(2^255-19)q, which is between 0 and 2^255-20. */
        h0 += 19 * q;
        /* Goal: Output h-2^255 q, which is between 0 and 2^255-20. */

        let carry0 = h0 >> 26; h1 += carry0; h0 -= carry0 << 26;
        let carry1 = h1 >> 25; h2 += carry1; h1 -= carry1 << 25;
        let carry2 = h2 >> 26; h3 += carry2; h2 -= carry2 << 26;
        let carry3 = h3 >> 25; h4 += carry3; h3 -= carry3 << 25;
        let carry4 = h4 >> 26; h5 += carry4; h4 -= carry4 << 26;
        let carry5 = h5 >> 25; h6 += carry5; h5 -= carry5 << 25;
        let carry6 = h6 >> 26; h7 += carry6; h6 -= carry6 << 26;
        let carry7 = h7 >> 25; h8 += carry7; h7 -= carry7 << 25;
        let carry8 = h8 >> 26; h9 += carry8; h8 -= carry8 << 26;
        let carry9 = h9 >> 25;               h9 -= carry9 << 25;
                            /* h10 = carry9 */

        /*
        Goal: Output h0+...+2^255 h10-2^255 q, which is between 0 and 2^255-20.
        Have h0+...+2^230 h9 between 0 and 2^255-1;
        evidently 2^255 h10-2^255 q = 0.
        Goal: Output h0+...+2^230 h9.
        */
        [
            (h0 >> 0) as u8,
            (h0 >> 8) as u8,
            (h0 >> 16) as u8,
            ((h0 >> 24) | (h1 << 2)) as u8,
            (h1 >> 6) as u8,
            (h1 >> 14) as u8,
            ((h1 >> 22) | (h2 << 3)) as u8,
            (h2 >> 5) as u8,
            (h2 >> 13) as u8,
            ((h2 >> 21) | (h3 << 5)) as u8,
            (h3 >> 3) as u8,
            (h3 >> 11) as u8,
            ((h3 >> 19) | (h4 << 6)) as u8,
            (h4 >> 2) as u8,
            (h4 >> 10) as u8,
            (h4 >> 18) as u8,
            (h5 >> 0) as u8,
            (h5 >> 8) as u8,
            (h5 >> 16) as u8,
            ((h5 >> 24) | (h6 << 1)) as u8,
            (h6 >> 7) as u8,
            (h6 >> 15) as u8,
            ((h6 >> 23) | (h7 << 3)) as u8,
            (h7 >> 5) as u8,
            (h7 >> 13) as u8,
            ((h7 >> 21) | (h8 << 4)) as u8,
            (h8 >> 4) as u8,
            (h8 >> 12) as u8,
            ((h8 >> 20) | (h9 << 6)) as u8,
            (h9 >> 2) as u8,
            (h9 >> 10) as u8,
            (h9 >> 18) as u8,
        ]
    }

    pub fn maybe_swap_with(&mut self, other: &mut Fe, do_swap: i32) {
        let &Fe(f) = self;
        let &Fe(g) = other;
        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let g0 = g[0];
        let g1 = g[1];
        let g2 = g[2];
        let g3 = g[3];
        let g4 = g[4];
        let g5 = g[5];
        let g6 = g[6];
        let g7 = g[7];
        let g8 = g[8];
        let g9 = g[9];
        let mut x0 = f0 ^ g0;
        let mut x1 = f1 ^ g1;
        let mut x2 = f2 ^ g2;
        let mut x3 = f3 ^ g3;
        let mut x4 = f4 ^ g4;
        let mut x5 = f5 ^ g5;
        let mut x6 = f6 ^ g6;
        let mut x7 = f7 ^ g7;
        let mut x8 = f8 ^ g8;
        let mut x9 = f9 ^ g9;
        let b = -do_swap;
        x0 &= b;
        x1 &= b;
        x2 &= b;
        x3 &= b;
        x4 &= b;
        x5 &= b;
        x6 &= b;
        x7 &= b;
        x8 &= b;
        x9 &= b;
        *self  = Fe([f0^x0, f1^x1, f2^x2, f3^x3, f4^x4,
                     f5^x5, f6^x6, f7^x7, f8^x8, f9^x9]);
        *other = Fe([g0^x0, g1^x1, g2^x2, g3^x3, g4^x4,
                     g5^x5, g6^x6, g7^x7, g8^x8, g9^x9]);
    }


    /*
    h = f * 121666
    Can overlap h with f.

    Preconditions:
       |f| bounded by 1.1*2^26,1.1*2^25,1.1*2^26,1.1*2^25,etc.

    Postconditions:
       |h| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
    */

    fn mul_121666(&self) -> Fe {
        let &Fe(f) = self;

        let mut h0 = (f[0] as i64) * 121666;
        let mut h1 = (f[1] as i64) * 121666;
        let mut h2 = (f[2] as i64) * 121666;
        let mut h3 = (f[3] as i64) * 121666;
        let mut h4 = (f[4] as i64) * 121666;
        let mut h5 = (f[5] as i64) * 121666;
        let mut h6 = (f[6] as i64) * 121666;
        let mut h7 = (f[7] as i64) * 121666;
        let mut h8 = (f[8] as i64) * 121666;
        let mut h9 = (f[9] as i64) * 121666;

        let carry9 = (h9 + (1<<24)) >> 25; h0 += carry9 * 19; h9 -= carry9 << 25;
        let carry1 = (h1 + (1<<24)) >> 25; h2 += carry1; h1 -= carry1 << 25;
        let carry3 = (h3 + (1<<24)) >> 25; h4 += carry3; h3 -= carry3 << 25;
        let carry5 = (h5 + (1<<24)) >> 25; h6 += carry5; h5 -= carry5 << 25;
        let carry7 = (h7 + (1<<24)) >> 25; h8 += carry7; h7 -= carry7 << 25;

        let carry0 = (h0 + (1<<25)) >> 26; h1 += carry0; h0 -= carry0 << 26;
        let carry2 = (h2 + (1<<25)) >> 26; h3 += carry2; h2 -= carry2 << 26;
        let carry4 = (h4 + (1<<25)) >> 26; h5 += carry4; h4 -= carry4 << 26;
        let carry6 = (h6 + (1<<25)) >> 26; h7 += carry6; h6 -= carry6 << 26;
        let carry8 = (h8 + (1<<25)) >> 26; h9 += carry8; h8 -= carry8 << 26;

        Fe([h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32,
            h5 as i32, h6 as i32, h7 as i32, h8 as i32, h9 as i32])
    }


    /*
    h = f * f
    Can overlap h with f.

    Preconditions:
       |f| bounded by 1.1*2^26,1.1*2^25,1.1*2^26,1.1*2^25,etc.

    Postconditions:
       |h| bounded by 1.1*2^25,1.1*2^24,1.1*2^25,1.1*2^24,etc.
    */

    /*
    See fe_mul.c for discussion of implementation strategy.
    */
    fn square(&self) -> Fe {
        let &Fe(f) = self;

        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let f0_2 = 2 * f0;
        let f1_2 = 2 * f1;
        let f2_2 = 2 * f2;
        let f3_2 = 2 * f3;
        let f4_2 = 2 * f4;
        let f5_2 = 2 * f5;
        let f6_2 = 2 * f6;
        let f7_2 = 2 * f7;
        let f5_38 = 38 * f5; /* 1.31*2^30 */
        let f6_19 = 19 * f6; /* 1.31*2^30 */
        let f7_38 = 38 * f7; /* 1.31*2^30 */
        let f8_19 = 19 * f8; /* 1.31*2^30 */
        let f9_38 = 38 * f9; /* 1.31*2^30 */
        let f0f0    = (f0   as i64) * (f0 as i64);
        let f0f1_2  = (f0_2 as i64) * (f1 as i64);
        let f0f2_2  = (f0_2 as i64) * (f2 as i64);
        let f0f3_2  = (f0_2 as i64) * (f3 as i64);
        let f0f4_2  = (f0_2 as i64) * (f4 as i64);
        let f0f5_2  = (f0_2 as i64) * (f5 as i64);
        let f0f6_2  = (f0_2 as i64) * (f6 as i64);
        let f0f7_2  = (f0_2 as i64) * (f7 as i64);
        let f0f8_2  = (f0_2 as i64) * (f8 as i64);
        let f0f9_2  = (f0_2 as i64) * (f9 as i64);
        let f1f1_2  = (f1_2 as i64) * (f1 as i64);
        let f1f2_2  = (f1_2 as i64) * (f2 as i64);
        let f1f3_4  = (f1_2 as i64) * (f3_2 as i64);
        let f1f4_2  = (f1_2 as i64) * (f4 as i64);
        let f1f5_4  = (f1_2 as i64) * (f5_2 as i64);
        let f1f6_2  = (f1_2 as i64) * (f6 as i64);
        let f1f7_4  = (f1_2 as i64) * (f7_2 as i64);
        let f1f8_2  = (f1_2 as i64) * (f8 as i64);
        let f1f9_76 = (f1_2 as i64) * (f9_38 as i64);
        let f2f2    = (f2   as i64) * (f2 as i64);
        let f2f3_2  = (f2_2 as i64) * (f3 as i64);
        let f2f4_2  = (f2_2 as i64) * (f4 as i64);
        let f2f5_2  = (f2_2 as i64) * (f5 as i64);
        let f2f6_2  = (f2_2 as i64) * (f6 as i64);
        let f2f7_2  = (f2_2 as i64) * (f7 as i64);
        let f2f8_38 = (f2_2 as i64) * (f8_19 as i64);
        let f2f9_38 = (f2   as i64) * (f9_38 as i64);
        let f3f3_2  = (f3_2 as i64) * (f3 as i64);
        let f3f4_2  = (f3_2 as i64) * (f4 as i64);
        let f3f5_4  = (f3_2 as i64) * (f5_2 as i64);
        let f3f6_2  = (f3_2 as i64) * (f6 as i64);
        let f3f7_76 = (f3_2 as i64) * (f7_38 as i64);
        let f3f8_38 = (f3_2 as i64) * (f8_19 as i64);
        let f3f9_76 = (f3_2 as i64) * (f9_38 as i64);
        let f4f4    = (f4   as i64) * (f4 as i64);
        let f4f5_2  = (f4_2 as i64) * (f5 as i64);
        let f4f6_38 = (f4_2 as i64) * (f6_19 as i64);
        let f4f7_38 = (f4   as i64) * (f7_38 as i64);
        let f4f8_38 = (f4_2 as i64) * (f8_19 as i64);
        let f4f9_38 = (f4   as i64) * (f9_38 as i64);
        let f5f5_38 = (f5   as i64) * (f5_38 as i64);
        let f5f6_38 = (f5_2 as i64) * (f6_19 as i64);
        let f5f7_76 = (f5_2 as i64) * (f7_38 as i64);
        let f5f8_38 = (f5_2 as i64) * (f8_19 as i64);
        let f5f9_76 = (f5_2 as i64) * (f9_38 as i64);
        let f6f6_19 = (f6   as i64) * (f6_19 as i64);
        let f6f7_38 = (f6   as i64) * (f7_38 as i64);
        let f6f8_38 = (f6_2 as i64) * (f8_19 as i64);
        let f6f9_38 = (f6   as i64) * (f9_38 as i64);
        let f7f7_38 = (f7   as i64) * (f7_38 as i64);
        let f7f8_38 = (f7_2 as i64) * (f8_19 as i64);
        let f7f9_76 = (f7_2 as i64) * (f9_38 as i64);
        let f8f8_19 = (f8   as i64) * (f8_19 as i64);
        let f8f9_38 = (f8   as i64) * (f9_38 as i64);
        let f9f9_38 = (f9   as i64) * (f9_38 as i64);
        let mut h0 = f0f0  +f1f9_76+f2f8_38+f3f7_76+f4f6_38+f5f5_38;
        let mut h1 = f0f1_2+f2f9_38+f3f8_38+f4f7_38+f5f6_38;
        let mut h2 = f0f2_2+f1f1_2 +f3f9_76+f4f8_38+f5f7_76+f6f6_19;
        let mut h3 = f0f3_2+f1f2_2 +f4f9_38+f5f8_38+f6f7_38;
        let mut h4 = f0f4_2+f1f3_4 +f2f2   +f5f9_76+f6f8_38+f7f7_38;
        let mut h5 = f0f5_2+f1f4_2 +f2f3_2 +f6f9_38+f7f8_38;
        let mut h6 = f0f6_2+f1f5_4 +f2f4_2 +f3f3_2 +f7f9_76+f8f8_19;
        let mut h7 = f0f7_2+f1f6_2 +f2f5_2 +f3f4_2 +f8f9_38;
        let mut h8 = f0f8_2+f1f7_4 +f2f6_2 +f3f5_4 +f4f4   +f9f9_38;
        let mut h9 = f0f9_2+f1f8_2 +f2f7_2 +f3f6_2 +f4f5_2;

        let carry0 = (h0 + (1<<25)) >> 26; h1 += carry0; h0 -= carry0 << 26;
        let carry4 = (h4 + (1<<25)) >> 26; h5 += carry4; h4 -= carry4 << 26;

        let carry1 = (h1 + (1<<24)) >> 25; h2 += carry1; h1 -= carry1 << 25;
        let carry5 = (h5 + (1<<24)) >> 25; h6 += carry5; h5 -= carry5 << 25;

        let carry2 = (h2 + (1<<25)) >> 26; h3 += carry2; h2 -= carry2 << 26;
        let carry6 = (h6 + (1<<25)) >> 26; h7 += carry6; h6 -= carry6 << 26;

        let carry3 = (h3 + (1<<24)) >> 25; h4 += carry3; h3 -= carry3 << 25;
        let carry7 = (h7 + (1<<24)) >> 25; h8 += carry7; h7 -= carry7 << 25;

        let carry4 = (h4 + (1<<25)) >> 26; h5 += carry4; h4 -= carry4 << 26;
        let carry8 = (h8 + (1<<25)) >> 26; h9 += carry8; h8 -= carry8 << 26;

        let carry9 = (h9 + (1<<24)) >> 25; h0 += carry9 * 19; h9 -= carry9 << 25;

        let carrya = (h0 + (1<<25)) >> 26; h1 += carrya; h0 -= carrya << 26;

        Fe([h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32,
            h5 as i32, h6 as i32, h7 as i32, h8 as i32, h9 as i32])
    }

    fn invert(&self) -> Fe
    {
        let z1 = self;

        /* qhasm: z2 = z1^2^1 */
        let z2 = z1.square();
        /* qhasm: z8 = z2^2^2 */
        let z8 = z2.square().square();
        /* qhasm: z9 = z1*z8 */
        let z9 = z1*z8;

        /* qhasm: z11 = z2*z9 */
        let z11 = z2*z9;

        /* qhasm: z22 = z11^2^1 */
        let z22 = z11.square();

        /* qhasm: z_5_0 = z9*z22 */
        let z_5_0 = z9*z22;

        /* qhasm: z_10_5 = z_5_0^2^5 */
        let z_10_5 = range(0u, 5).fold(z_5_0, |z_5_n, _| z_5_n.square());

        /* qhasm: z_10_0 = z_10_5*z_5_0 */
        let z_10_0 = z_10_5*z_5_0;

        /* qhasm: z_20_10 = z_10_0^2^10 */
        let z_20_10 = range(0u, 10).fold(z_10_0, |x, _| x.square());

        /* qhasm: z_20_0 = z_20_10*z_10_0 */
        let z_20_0 = z_20_10*z_10_0;

        /* qhasm: z_40_20 = z_20_0^2^20 */
        let z_40_20 = range(0u, 20).fold(z_20_0, |x, _| x.square());

        /* qhasm: z_40_0 = z_40_20*z_20_0 */
        let z_40_0 = z_40_20*z_20_0;

        /* qhasm: z_50_10 = z_40_0^2^10 */
        let z_50_10 = range(0u, 10).fold(z_40_0, |x, _| x.square());

        /* qhasm: z_50_0 = z_50_10*z_10_0 */
        let z_50_0 = z_50_10*z_10_0;

        /* qhasm: z_100_50 = z_50_0^2^50 */
        let z_100_50 = range(0u, 50).fold(z_50_0, |x, _| x.square());

        /* qhasm: z_100_0 = z_100_50*z_50_0 */
        let z_100_0 = z_100_50*z_50_0;

        /* qhasm: z_200_100 = z_100_0^2^100 */
        let z_200_100 = range(0u, 100).fold(z_100_0, |x, _| x.square());

        /* qhasm: z_200_0 = z_200_100*z_100_0 */
        /* asm 1: fe_mul(>z_200_0=fe#3,<z_200_100=fe#4,<z_100_0=fe#3); */
        /* asm 2: fe_mul(>z_200_0=t2,<z_200_100=t3,<z_100_0=t2); */
        let z_200_0 = z_200_100*z_100_0;

        /* qhasm: z_250_50 = z_200_0^2^50 */
        let z_250_50 = range(0u, 50).fold(z_200_0, |x, _| x.square());

        /* qhasm: z_250_0 = z_250_50*z_50_0 */
        let z_250_0 = z_250_50*z_50_0;

        /* qhasm: z_255_5 = z_250_0^2^5 */
        let z_255_5 = range(0u, 5).fold(z_250_0, |x, _| x.square());

        /* qhasm: z_255_21 = z_255_5*z11 */
        /* asm 1: fe_mul(>z_255_21=fe#12,<z_255_5=fe#2,<z11=fe#1); */
        /* asm 2: fe_mul(>z_255_21=out,<z_255_5=t1,<z11=t0); */
        let z_255_21 = z_255_5*z11;

        z_255_21
    }

}


struct GeP2 {
    x: Fe,
    y: Fe,
    z: Fe,
}

struct GeP3 {
    x: Fe,
    y: Fe,
    z: Fe,
    t: Fe,
}

struct GeP1P1 {
    x: Fe,
    y: Fe,
    z: Fe,
    t: Fe,
}


/*
Input:
    s[0]+256*s[1]+...+256^63*s[63] = s

Output:
    s[0]+256*s[1]+...+256^31*s[31] = s mod l
    where l = 2^252 + 27742317777372353535851937790883648493.
    Overwrites s in place.
*/

fn sc_reduce(s: &mut [u8]) {
    let mut s0: i64 = 2097151 & load_3i(s);
    let mut s1: i64 = 2097151 & (load_4i(s.slice(2, 6)) >> 5);
    let mut s2: i64 = 2097151 & (load_3i(s.slice(5, 8)) >> 2);
    let mut s3: i64 = 2097151 & (load_4i(s.slice(7, 11)) >> 7);
    let mut s4: i64 = 2097151 & (load_4i(s.slice(10, 14)) >> 4);
    let mut s5: i64 = 2097151 & (load_3i(s.slice(13, 16)) >> 1);
    let mut s6: i64 = 2097151 & (load_4i(s.slice(15, 19)) >> 6);
    let mut s7: i64 = 2097151 & (load_3i(s.slice(18, 21)) >> 3);
    let mut s8: i64 = 2097151 & load_3i(s.slice(21, 24));
    let mut s9: i64 = 2097151 & (load_4i(s.slice(23, 27)) >> 5);
    let mut s10: i64 = 2097151 & (load_3i(s.slice(26, 29)) >> 2);
    let mut s11: i64 = 2097151 & (load_4i(s.slice(28, 32)) >> 7);
    let mut s12: i64 = 2097151 & (load_4i(s.slice(31, 35)) >> 4);
    let mut s13: i64 = 2097151 & (load_3i(s.slice(34, 37)) >> 1);
    let mut s14: i64 = 2097151 & (load_4i(s.slice(36, 40)) >> 6);
    let mut s15: i64 = 2097151 & (load_3i(s.slice(39, 42)) >> 3);
    let mut s16: i64 = 2097151 & load_3i(s.slice(42, 45));
    let mut s17: i64 = 2097151 & (load_4i(s.slice(44, 48)) >> 5);
    let mut s18: i64 = 2097151 & (load_3i(s.slice(47, 50)) >> 2);
    let mut s19: i64 = 2097151 & (load_4i(s.slice(49, 53)) >> 7);
    let mut s20: i64 = 2097151 & (load_4i(s.slice(52, 56)) >> 4);
    let mut s21: i64 = 2097151 & (load_3i(s.slice(55, 58)) >> 1);
    let mut s22: i64 = 2097151 & (load_4i(s.slice(57, 61)) >> 6);
    let mut s23: i64 = (load_4i(s.slice(60, 64)) >> 3);
    let mut carry0: i64;
    let mut carry1: i64;
    let mut carry2: i64;
    let mut carry3: i64;
    let mut carry4: i64;
    let mut carry5: i64;
    let mut carry6: i64;
    let mut carry7: i64;
    let mut carry8: i64;
    let mut carry9: i64;
    let mut carry10: i64;
    let mut carry11: i64;
    let mut carry12: i64;
    let mut carry13: i64;
    let mut carry14: i64;
    let mut carry15: i64;
    let mut carry16: i64;

    s11 += s23 * 666643;
    s12 += s23 * 470296;
    s13 += s23 * 654183;
    s14 -= s23 * 997805;
    s15 += s23 * 136657;
    s16 -= s23 * 683901;


    s10 += s22 * 666643;
    s11 += s22 * 470296;
    s12 += s22 * 654183;
    s13 -= s22 * 997805;
    s14 += s22 * 136657;
    s15 -= s22 * 683901;


    s9 += s21 * 666643;
    s10 += s21 * 470296;
    s11 += s21 * 654183;
    s12 -= s21 * 997805;
    s13 += s21 * 136657;
    s14 -= s21 * 683901;


    s8 += s20 * 666643;
    s9 += s20 * 470296;
    s10 += s20 * 654183;
    s11 -= s20 * 997805;
    s12 += s20 * 136657;
    s13 -= s20 * 683901;


    s7 += s19 * 666643;
    s8 += s19 * 470296;
    s9 += s19 * 654183;
    s10 -= s19 * 997805;
    s11 += s19 * 136657;
    s12 -= s19 * 683901;


    s6 += s18 * 666643;
    s7 += s18 * 470296;
    s8 += s18 * 654183;
    s9 -= s18 * 997805;
    s10 += s18 * 136657;
    s11 -= s18 * 683901;


    carry6 = (s6 + (1<<20)) >> 21; s7 += carry6; s6 -= carry6 << 21;
    carry8 = (s8 + (1<<20)) >> 21; s9 += carry8; s8 -= carry8 << 21;
    carry10 = (s10 + (1<<20)) >> 21; s11 += carry10; s10 -= carry10 << 21;
    carry12 = (s12 + (1<<20)) >> 21; s13 += carry12; s12 -= carry12 << 21;
    carry14 = (s14 + (1<<20)) >> 21; s15 += carry14; s14 -= carry14 << 21;
    carry16 = (s16 + (1<<20)) >> 21; s17 += carry16; s16 -= carry16 << 21;

    carry7 = (s7 + (1<<20)) >> 21; s8 += carry7; s7 -= carry7 << 21;
    carry9 = (s9 + (1<<20)) >> 21; s10 += carry9; s9 -= carry9 << 21;
    carry11 = (s11 + (1<<20)) >> 21; s12 += carry11; s11 -= carry11 << 21;
    carry13 = (s13 + (1<<20)) >> 21; s14 += carry13; s13 -= carry13 << 21;
    carry15 = (s15 + (1<<20)) >> 21; s16 += carry15; s15 -= carry15 << 21;

    s5 += s17 * 666643;
    s6 += s17 * 470296;
    s7 += s17 * 654183;
    s8 -= s17 * 997805;
    s9 += s17 * 136657;
    s10 -= s17 * 683901;


    s4 += s16 * 666643;
    s5 += s16 * 470296;
    s6 += s16 * 654183;
    s7 -= s16 * 997805;
    s8 += s16 * 136657;
    s9 -= s16 * 683901;


    s3 += s15 * 666643;
    s4 += s15 * 470296;
    s5 += s15 * 654183;
    s6 -= s15 * 997805;
    s7 += s15 * 136657;
    s8 -= s15 * 683901;


    s2 += s14 * 666643;
    s3 += s14 * 470296;
    s4 += s14 * 654183;
    s5 -= s14 * 997805;
    s6 += s14 * 136657;
    s7 -= s14 * 683901;


    s1 += s13 * 666643;
    s2 += s13 * 470296;
    s3 += s13 * 654183;
    s4 -= s13 * 997805;
    s5 += s13 * 136657;
    s6 -= s13 * 683901;


    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;
    s12 = 0;

    carry0 = (s0 + (1<<20)) >> 21; s1 += carry0; s0 -= carry0 << 21;
    carry2 = (s2 + (1<<20)) >> 21; s3 += carry2; s2 -= carry2 << 21;
    carry4 = (s4 + (1<<20)) >> 21; s5 += carry4; s4 -= carry4 << 21;
    carry6 = (s6 + (1<<20)) >> 21; s7 += carry6; s6 -= carry6 << 21;
    carry8 = (s8 + (1<<20)) >> 21; s9 += carry8; s8 -= carry8 << 21;
    carry10 = (s10 + (1<<20)) >> 21; s11 += carry10; s10 -= carry10 << 21;

    carry1 = (s1 + (1<<20)) >> 21; s2 += carry1; s1 -= carry1 << 21;
    carry3 = (s3 + (1<<20)) >> 21; s4 += carry3; s3 -= carry3 << 21;
    carry5 = (s5 + (1<<20)) >> 21; s6 += carry5; s5 -= carry5 << 21;
    carry7 = (s7 + (1<<20)) >> 21; s8 += carry7; s7 -= carry7 << 21;
    carry9 = (s9 + (1<<20)) >> 21; s10 += carry9; s9 -= carry9 << 21;
    carry11 = (s11 + (1<<20)) >> 21; s12 += carry11; s11 -= carry11 << 21;

    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;
    s12 = 0;

    carry0 = s0 >> 21; s1 += carry0; s0 -= carry0 << 21;
    carry1 = s1 >> 21; s2 += carry1; s1 -= carry1 << 21;
    carry2 = s2 >> 21; s3 += carry2; s2 -= carry2 << 21;
    carry3 = s3 >> 21; s4 += carry3; s3 -= carry3 << 21;
    carry4 = s4 >> 21; s5 += carry4; s4 -= carry4 << 21;
    carry5 = s5 >> 21; s6 += carry5; s5 -= carry5 << 21;
    carry6 = s6 >> 21; s7 += carry6; s6 -= carry6 << 21;
    carry7 = s7 >> 21; s8 += carry7; s7 -= carry7 << 21;
    carry8 = s8 >> 21; s9 += carry8; s8 -= carry8 << 21;
    carry9 = s9 >> 21; s10 += carry9; s9 -= carry9 << 21;
    carry10 = s10 >> 21; s11 += carry10; s10 -= carry10 << 21;
    carry11 = s11 >> 21; s12 += carry11; s11 -= carry11 << 21;

    s0 += s12 * 666643;
    s1 += s12 * 470296;
    s2 += s12 * 654183;
    s3 -= s12 * 997805;
    s4 += s12 * 136657;
    s5 -= s12 * 683901;


    carry0 = s0 >> 21; s1 += carry0; s0 -= carry0 << 21;
    carry1 = s1 >> 21; s2 += carry1; s1 -= carry1 << 21;
    carry2 = s2 >> 21; s3 += carry2; s2 -= carry2 << 21;
    carry3 = s3 >> 21; s4 += carry3; s3 -= carry3 << 21;
    carry4 = s4 >> 21; s5 += carry4; s4 -= carry4 << 21;
    carry5 = s5 >> 21; s6 += carry5; s5 -= carry5 << 21;
    carry6 = s6 >> 21; s7 += carry6; s6 -= carry6 << 21;
    carry7 = s7 >> 21; s8 += carry7; s7 -= carry7 << 21;
    carry8 = s8 >> 21; s9 += carry8; s8 -= carry8 << 21;
    carry9 = s9 >> 21; s10 += carry9; s9 -= carry9 << 21;
    carry10 = s10 >> 21; s11 += carry10; s10 -= carry10 << 21;

    s[0] = (s0 >> 0) as u8;
    s[1] = (s0 >> 8) as u8;
    s[2] = ((s0 >> 16) | (s1 << 5)) as u8;
    s[3] = (s1 >> 3) as u8;
    s[4] = (s1 >> 11) as u8;
    s[5] = ((s1 >> 19) | (s2 << 2)) as u8;
    s[6] = (s2 >> 6) as u8;
    s[7] = ((s2 >> 14) | (s3 << 7)) as u8;
    s[8] = (s3 >> 1) as u8;
    s[9] = (s3 >> 9) as u8;
    s[10] = ((s3 >> 17) | (s4 << 4)) as u8;
    s[11] = (s4 >> 4) as u8;
    s[12] = (s4 >> 12) as u8;
    s[13] = ((s4 >> 20) | (s5 << 1)) as u8;
    s[14] = (s5 >> 7) as u8;
    s[15] = ((s5 >> 15) | (s6 << 6)) as u8;
    s[16] = (s6 >> 2) as u8;
    s[17] = (s6 >> 10) as u8;
    s[18] = ((s6 >> 18) | (s7 << 3)) as u8;
    s[19] = (s7 >> 5) as u8;
    s[20] = (s7 >> 13) as u8;
    s[21] = (s8 >> 0) as u8;
    s[22] = (s8 >> 8) as u8;
    s[23] = ((s8 >> 16) | (s9 << 5)) as u8;
    s[24] = (s9 >> 3) as u8;
    s[25] = (s9 >> 11) as u8;
    s[26] = ((s9 >> 19) | (s10 << 2)) as u8;
    s[27] = (s10 >> 6) as u8;
    s[28] = ((s10 >> 14) | (s11 << 7)) as u8;
    s[29] = (s11 >> 1) as u8;
    s[30] = (s11 >> 9) as u8;
    s[31] = (s11 >> 17) as u8;
}



pub fn curve25519(n: &[u8], p: &[u8]) -> [u8, ..32] {
    let mut e = [0u8, ..32];
    let mut x2;
    let mut z2;
    let mut x3;
    let mut z3;
    let mut pos: int;
    let mut swap: i32;
    let mut b: i32;

    for (d,s) in e.iter_mut().zip(n.iter()) {
      *d = *s;
    }
    e[0] &= 248;
    e[31] &= 127;
    e[31] |= 64;
    let x1 = Fe::from_bytes(p);
    x2 = FE_ONE;
    z2 = FE_ZERO;
    x3 = x1;
    z3 = FE_ONE;

    swap = 0;
    pos = 254;
    while pos >= 0 {
        b = (e[(pos as uint) / 8] >> ((pos as uint) & 7)) as i32;
        b &= 1;
        swap ^= b;
        x2.maybe_swap_with(&mut x3, swap);
        z2.maybe_swap_with(&mut z3, swap);
        swap = b;

        let d = x3 - z3;
        let b = x2 - z2;
        let a = x2 + z2;
        let c = x3 + z3;
        let da = d * a;
        let cb = c * b;
        let bb = b.square();
        let aa = a.square();
        let t0 = da + cb;
        let t1 = da - cb;
        let x4 = aa*bb;
        let e = aa - bb;
        let t2 = t1.square();
        let t3 = e.mul_121666();
        let x5 = t0.square();
        let t4 = bb + t3;
        let z5 = x1 * t2;
        let z4 = e*t4;

        z2 = z4;
        z3 = z5;
        x2 = x4;
        x3 = x5;
        pos -= 1;
    }
    x2.maybe_swap_with(&mut x3, swap);
    z2.maybe_swap_with(&mut z3, swap);

    (z2.invert() * x2).to_bytes()
}

pub fn curve25519_base(x: &[u8]) -> [u8, ..32] {
    let mut base : [u8, ..32] = [0, ..32];
    base[0] = 9;
    curve25519(x, base)
}

mod tests {
    use curve25519_fe::{Fe, curve25519, curve25519_base};

    #[test]
    fn from_to_bytes_preserves() {
        for i in range(0u, 50) {
            let mut e = Vec::from_fn(32, |idx| (idx*(1289+i*761)) as u8);
            e.as_mut_slice()[0] &= 248;
            e.as_mut_slice()[31] &= 127;
            e.as_mut_slice()[31] |= 64;
            let fe = Fe::from_bytes(e.as_slice());
            let e_preserved = fe.to_bytes();
            assert_eq!(e, e_preserved.to_vec());
        }
    }

    #[test]
    fn swap_test() {
        let mut f = Fe([10,20,30,40,50,60,70,80,90,100]);
        let mut g = Fe([11,21,31,41,51,61,71,81,91,101]);
        let f_initial = f;
        let g_initial = g;
        f.maybe_swap_with(&mut g, 0);
        assert_eq!(f, f_initial);
        assert_eq!(g, g_initial);

        f.maybe_swap_with(&mut g, 1);
        assert_eq!(f, g_initial);
        assert_eq!(g, f_initial);
    }

    struct CurveGen {
        which: uint
    }
    impl CurveGen {
        fn new(seed: uint) -> CurveGen {
            CurveGen{which: seed}
        }
    }
    impl Iterator<Fe> for CurveGen {
        fn next(&mut self) -> Option<Fe> {
            let mut e = Vec::from_fn(32, |idx| (idx*(1289+self.which*761)) as u8);
            e.as_mut_slice()[0] &= 248;
            e.as_mut_slice()[31] &= 127;
            e.as_mut_slice()[31] |= 64;
            Some(Fe::from_bytes(e.as_slice()))
        }
    }

    #[test]
    fn mul_commutes() {
       for (x,y) in CurveGen::new(1).zip(CurveGen::new(2)).take(40) {
          assert_eq!(x*y, y*x);
       };
    }

    #[test]
    fn mul_assoc() {
       for (x,(y,z)) in CurveGen::new(1).zip(CurveGen::new(2).zip(CurveGen::new(3))).take(40) {
          assert_eq!((x*y)*z, x*(y*z));
       };
    }

    #[test]
    fn invert_inverts() {
       for x in CurveGen::new(1).take(40) {
          assert_eq!(x.invert().invert(), x);
       };
    }

    #[test]
    fn square_by_mul() {
       for x in CurveGen::new(1).take(40) {
          assert_eq!(x*x, x.square());
       };
    }

    #[test]
    fn base_example() {
        let sk : [u8, ..32] = [
            0x77, 0x07, 0x6d, 0x0a, 0x73, 0x18, 0xa5, 0x7d, 0x3c, 0x16, 0xc1,
            0x72, 0x51, 0xb2, 0x66, 0x45, 0xdf, 0x4c, 0x2f, 0x87, 0xeb, 0xc0,
            0x99, 0x2a, 0xb1, 0x77, 0xfb, 0xa5, 0x1d, 0xb9, 0x2c, 0x2a ];
        let pk = curve25519_base(sk);
        let correct : [u8, ..32] = [
             0x85,0x20,0xf0,0x09,0x89,0x30,0xa7,0x54
            ,0x74,0x8b,0x7d,0xdc,0xb4,0x3e,0xf7,0x5a
            ,0x0d,0xbf,0x3a,0x0d,0x26,0x38,0x1a,0xf4
            ,0xeb,0xa4,0xa9,0x8e,0xaa,0x9b,0x4e,0x6a ];
        assert_eq!(pk.to_vec(), correct.to_vec());
    }
}
