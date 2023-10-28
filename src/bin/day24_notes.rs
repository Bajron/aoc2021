use std::collections::HashSet;

type Register = i32;

fn monad(input: &Vec<i32>) -> Register {
    let mut w: Register;
    let mut x: Register;
    let mut y: Register;
    let mut z: Register = 0;

    let mut it = input.iter();
    let mut next = move || *it.next().unwrap() as Register;

    /*

    cat inputs/day_24_input.txt  | sed -r \
        -e 's/add\s+([wxyz])\s+(\S+)/\1 += \2;/' \
        -e 's/mul\s+([wxyz])\s+(\S+)/\1 *= \2;/' \
        -e 's/div\s+([wxyz])\s+(\S+)/\1 \/= \2;/' \
        -e 's/mod\s+([wxyz])\s+(\S+)/\1 %= \2;/' \
        -e 's/eql\s+([wxyz])\s+(\S+)/\1 = if \1 == \2 {1} else {0};/' \
        -e 's/inp\s+([wxyz])/\1 = next();/'

    */
    // --------------8<------------

    w = next(); // w0
    x = z % 26 + 13;
    x = if x != w { 1 } else { 0 }; // initially 13 != [1-9], so x == 1
    y = 25 * x + 1;
    z *= y;
    y = (w + 5) * x;
    z += y;
    // z is initially 0, so z = w0 + 5

    w = next(); // w1
    x = z % 26 + 15;
    x = if x != w { 1 } else { 0 }; // x > 15 and w < 10, so x == 1
    y = 25 * x + 1;
    z *= y;
    y = (w + 14) * x;
    z += y;
    // z = 26 * (w0 + 5) + w1 + 14

    w = next(); // w2
    x = z % 26 + 15;
    x = if x != w { 1 } else { 0 }; // x > 15 and w < 10, so x == 1
    y = 25 * x + 1;
    z *= y;
    y = (w + 15) * x;
    z += y;
    // z = 26 * (26 * (w0 + 5) + w1 + 14) + w2 + 15

    w = next(); // w3
    x = z % 26 + 11;
    x = if x != w { 1 } else { 0 }; // x >= 11 and w < 10, so x == 1
    y = 25 * x + 1;
    z *= y;
    y = (w + 16) * x;
    z += y;
    // z = 26 * (26 * (26 * (w0 + 5) + w1 + 14) + w2 + 15) + w3 + 16

    w = next(); // w4
    x = z % 26 - 16; // w3 + 16 - 16 = w3
    z /= 26; // OPTIMIZE: w3 does not matter?
             // z = 26 * (26 * (w0 + 5) + w1 + 14) + w2 + 15
    x = if x != w { 1 } else { 0 }; // w4 != w3 ?
    y = 25 * x + 1; //     26 or 1
    z *= y;
    y = (w + 8) * x; // w + 8 or 0
    z += y;
    // w4 != w3
    // z = 26 * (26 * (26 * (w0 + 5) + w1 + 14) + w2 + 15) + w4 + 8
    // or
    // z =       26 * (26 * (w0 + 5) + w1 + 14) + w2 + 15

    w = next(); // w5
    x = z % 26 - 11;
    z /= 26; // OPTIMIZE: w4 does not matter?
             // It influnces the z value at this point, and the branching below
             // But does not appear in the final value of z

    // z =  26 * (26 * (w0 + 5) + w1 + 14) + w2 + 15
    // or
    // z =        26 * (w0 + 5) + w1 + 14
    x = if x != w { 1 } else { 0 }; // w5 relation with w4 or w2; w4-3 != w5 or w2 + 4 != w5
    y = 25 * x + 1;
    z *= y;
    y = (w + 9) * x;
    z += y;
    // x == 0
    // z =  26 * (26 * (w0 + 5) + w1 + 14) + w2 + 15
    // or
    // z =        26 * (w0 + 5) + w1 + 14
    // else
    // z =  26 * (26 * (26 * (w0 + 5) + w1 + 14) + w2 + 15) + w5 + 9
    // or
    // z =        26 * (26 * (w0 + 5) + w1 + 14) + w5 + 9

    w = next(); // w6
    x = z % 26 - 6;
    z /= 26; // OPTIMIZE: w5 does not matter?
    x = if x != w { 1 } else { 0 }; // w6 ~ w2, w1, w5
    y = 25 * x + 1;
    z *= y;
    y = w + 2;
    y *= x;
    z += y;

    w = next(); // w7
    x = z % 26 + 11;
    x = if x != w { 1 } else { 0 }; // always 1, x >= 11
    y = 25 * x + 1;
    z *= y;
    y = w + 13;
    y *= x;
    z += y;

    // z8 = 26 * (z7) + w7 + 13

    w = next(); // w8
    x = z % 26 + 10;
    x = if x != w { 1 } else { 0 }; // always 1, x >= 10
    y = 25 * x + 1;
    z *= y;
    y = w + 16;
    y *= x;
    z += y;

    // z8 = 26 * (26 * (z7) + w7 + 13) + w8 + 16

    w = next(); // w9
    x = z % 26 - 10;
    z /= 26; // OPTIMIZE: w8 does not matter?
    x = if x != w { 1 } else { 0 }; // w8 ~ w9
    y = 25 * x + 1;
    z *= y;
    y = w + 6;
    y *= x;
    z += y;

    w = next(); // w10
    x = z % 26 - 8;
    z /= 26; // OPTIMIZE: w9 does not matter?
    x = if x != w { 1 } else { 0 }; // w10
    y = 25 * x + 1;
    z *= y;
    y = w + 6;
    y *= x;
    z += y;

    w = next(); // w11
    x = z % 26 - 11;
    z /= 26; // OPTIMIZE: w10 does not matter?
    x = if x != w { 1 } else { 0 };
    y = 25 * x + 1;
    z *= y;
    y = (w + 9) * x;
    z += y;

    w = next(); // w12
    x = z % 26 + 12;
    x = if x != w { 1 } else { 0 }; // always 1, x >= 12
    y = 25 * x + 1;
    z *= y;
    y = w + 11;
    y *= x;
    z += y;

    w = next(); // w13
    x = z % 26 - 15;
    z /= 26; // OPTIMIZE: w12 does not matter?
    x = if x != w { 1 } else { 0 };
    y = 25 * x + 1; // y = 26 or 1
    z *= y;
    y = (w + 5) * x; // y == 6..=14 or 0
    z += y;

    // z_prev/26 * (26 or 1) + (6..=14 or 0)

    // we want z == 0

    // ----------------------------
    z
}

fn possible_monad(input: &Vec<i32>) -> bool {
    let mut z: Register;
    // Not needed 3, 4, 5, 8, 9, 10, 12

    let mut it = input.iter();
    let mut next = move || *it.next().unwrap() as Register;

    let mut possible_z: Vec<i32> = vec![];
    let mut possible_z_next: Vec<i32> = vec![];

    let w0 = next();
    z = w0 + 5;

    let w1 = next();
    z = 26 * z + w1 + 14;

    let w2 = next();
    z = 26 * z + w2 + 15;

    let w3 = next();
    z = 26 * z + w3 + 16;

    possible_z.push(z);

    let w4 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        let zz = pos_z / 26;
        possible_z_next.push(26 * zz + w4 + 8);
        possible_z_next.push(zz);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w5 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        let zz = pos_z / 26;
        possible_z_next.push(26 * zz + w5 + 9);
        possible_z_next.push(zz);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w6 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        let zz = pos_z / 26;
        possible_z_next.push(26 * zz + w6 + 2);
        possible_z_next.push(zz);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w7 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        possible_z_next.push(26 * pos_z + w7 + 13);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w8: i32 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        possible_z_next.push(pos_z / 26 + w8 + 16);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w9 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        let zz = pos_z / 26;
        possible_z_next.push(26 * zz + w9 + 6);
        possible_z_next.push(zz);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w10 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        let zz = pos_z / 26;
        possible_z_next.push(26 * zz + w10 + 6);
        possible_z_next.push(zz);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w11 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        let zz = pos_z / 26;
        possible_z_next.push(26 * zz + w11 + 9);
        possible_z_next.push(zz);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w12 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        possible_z_next.push(26 * pos_z + w12 + 11);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    let w13 = next();
    possible_z_next.clear();
    for pos_z in &possible_z {
        let zz = pos_z / 26;
        possible_z_next.push(26 * zz + w13 + 5);
        possible_z_next.push(zz);
    }
    std::mem::swap(&mut possible_z, &mut possible_z_next);

    assert!(possible_z.len() == 128);
    println!("{possible_z:?}");
    possible_z.iter().any(|&z| z == 0)
}

fn main() {
    let mut i = 10_000_000;
    let mut check: Register = Register::MAX;
    let mut best: Register = Register::MAX;

    let not_in_z: [usize; 7] = [3, 4, 5, 8, 9, 10, 12];
    let mut s: Vec<i32> = [9; 14].into();
    for i in not_in_z {
        s[i] = 0;
    }
    println!("{s:?}");

    while check != 0 && i >= 1_111_111 {
        i -= 1;

        let digits = i
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect::<Vec<i32>>();
        if digits.iter().any(|&d| d == 0) {
            continue;
        }
        assert!(digits.len() == 7);

        s.iter_mut()
            .filter(|x| **x != 0)
            .zip(digits.iter())
            .for_each(|(s, &d)| *s = d);

        if !possible_monad(&s) {
            continue;
        }
        println!("{i}...");

        let mut j = 10_000_000;
        while check != 0 && j >= 1_111_111 {
            j -= 1;

            let digits = j
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as i32)
                .collect::<Vec<i32>>();
            if digits.iter().any(|&d| d == 0) {
                continue;
            }

            not_in_z
                .iter()
                .zip(digits.iter())
                .for_each(|(&i, d)| s[i] = *d);

            check = monad(&s);

            if check < best {
                println!("{i} {j} / {s:?} / {check}");
                best = check;
            }
        }
    }

    println!("Got it: {i}")
}
