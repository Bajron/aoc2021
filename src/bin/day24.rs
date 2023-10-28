type Register = i32;

fn monad<T>(input: T) -> Register
where
    T: IntoIterator<Item = i32>,
{
    let mut w: Register;
    let mut x: Register = 0;
    let mut y: Register = 0;
    let mut z: Register = 0;

    let mut it = input.into_iter();
    let mut next = move || it.next().unwrap() as Register;

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

    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 13;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 5;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 15;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 14;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 15;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 15;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 16;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -16;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 8;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 9;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -6;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 2;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 13;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 16;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 6;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -8;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 6;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 9;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 12;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 11;
    y *= x;
    z += y;
    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -15;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += 5;
    y *= x;
    z += y;

    // ----------------------------
    z
}

// use rand::distributions::Uniform;
// use rand::{thread_rng, Rng};

fn main() {
    // let mut i = 91599994399395_i128 + 1;
    let mut i = 71111591176151_i128 + 1;
    let mut check: Register = Register::MAX;
    let mut best: Register = Register::MAX;
    let mut s: Vec<i32>;
    while check != 0 {
        i -= 1;

        s = i
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect::<Vec<i32>>();
        if s.iter().any(|&d| d == 0) {
            continue;
        }

        // s = thread_rng()
        //     .sample_iter(Uniform::new(1, 10))
        //     .take(14)
        //     .collect::<Vec<i32>>();

        check = monad(s.clone());

        if check.abs() < best {
            println!("{:?} {check}", s);
            best = check.abs();
        }
    }

    println!("Got it: {i}")
}
