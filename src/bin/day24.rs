fn monad<T>(input: T) -> i128
where
    T: IntoIterator<Item = i32>,
{
    let mut w = 0_i128;
    let mut x = 0_i128;
    let mut y = 0_i128;
    let mut z = 0_i128;

    let mut it = input.into_iter();
    let mut next = move || it.next().unwrap() as i128;

    /*

    cat inputs/day_24_input.txt  | sed -r \
        -e 's/add\s+([wxyz])\s+(\S+)/\1 += \2;/' \
        -e 's/mul\s+([wxyz])\s+(\S+)/\1 *= \2;/' \
        -e 's/div\s+([wxyz])\s+(\S+)/\1 \/= \2;/' \
        -e 's/mod\s+([wxyz])\s+(\S+)/\1 %= \2;/' \
        -e 's/eql\s+([wxyz])\s+(\S+)/if \1 == \2 {1} else {0};/' \
        -e 's/inp\s+([wxyz])/\1 = next();/'

    */
    // --------------8<------------

    w = next();
    x *= 0;
    x += z;
    x %= 26;
    z /= 1;
    x += 13;
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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
    if x == w {
        1
    } else {
        0
    };
    if x == 0 {
        1
    } else {
        0
    };
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

fn main() {
    let mut i = 100_000_000_000_000_i128;
    let mut check = 1_i128;
    let mut s: Vec<i32>;
    while check != 0 {
        i -= 1;
        s = i
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect::<Vec<i32>>();
        check = monad(s);

        if i % 1000_000_000 == 0 {
            println!("{i} {check}");
        }
    }

    println!("Got it: {i}")
}
