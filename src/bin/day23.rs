use std::fmt;

fn cost(c: char) -> i32 {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Bad letter"),
    }
}

fn corridor_index(c: char) -> usize {
    match c {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => panic!("Bad letter"),
    }
}

fn main() {
    let state = State {
        corridor: ['.'; 11],
        room_a: ['B', 'A'],
        room_b: ['C', 'D'],
        room_c: ['B', 'C'],
        room_d: ['D', 'A'],
        cost: 0,
    };

    let mut minimal_cost = i32::MAX;
    search(&state, &mut minimal_cost);

    println!("Final minimal cost example {minimal_cost}");

    let state = State {
        corridor: ['.'; 11],
        room_a: ['A', 'C'],
        room_b: ['D', 'D'],
        room_c: ['A', 'B'],
        room_d: ['C', 'B'],
        cost: 0,
    };

    let mut minimal_cost = i32::MAX;
    search(&state, &mut minimal_cost);

    println!("Final minimal cost mine {minimal_cost}");

}

const ALLOWED: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

fn allowed_on_right(c: char) -> usize {
    match c {
        'A' => 2,
        'B' => 3,
        'C' => 4,
        'D' => 5,
        _ => panic!("Bad letter"),
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    // Disallowed indices 2,4,6,8
    corridor: [char; 11],
    // A-room at 2, B:4, C:6, D:8
    room_a: [char; 2], // Index 0 is next to corridor
    room_b: [char; 2],
    room_c: [char; 2],
    room_d: [char; 2],
    cost: i32,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.room_a;
        let b = self.room_b;
        let c = self.room_c;
        let d = self.room_d;

        write!(
            f,
            "{}\n  {} {} {} {}\n  {} {} {} {}    {}",
            self.corridor.iter().collect::<String>(),
            a[0],
            b[0],
            c[0],
            d[0],
            a[1],
            b[1],
            c[1],
            d[1],
            self.cost
        )
    }
}

impl State {
    fn room(&self, ch: char) -> &[char; 2] {
        match ch {
            'A' => &self.room_a,
            'B' => &self.room_b,
            'C' => &self.room_c,
            'D' => &self.room_d,
            _ => panic!("bad room"),
        }
    }
    fn room_mut(&mut self, ch: char) -> &mut [char; 2] {
        match ch {
            'A' => &mut self.room_a,
            'D' => &mut self.room_d,
            'B' => &mut self.room_b,
            'C' => &mut self.room_c,
            _ => panic!("bad room"),
        }
    }

    fn is_done(&self) -> bool {
        self.room_a == ['A', 'A']
            && self.room_b == ['B', 'B']
            && self.room_c == ['C', 'C']
            && self.room_d == ['D', 'D']
    }
}

fn can_enter(room: &[char; 2], native: char) -> Option<usize> {
    match room {
        ['.', '.'] => Some(1),
        ['.', x] => {
            if *x == native {
                Some(0)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn search(s: &State, minimal_cost: &mut i32) {
    if s.is_done() {
        // println!("{s:?}");
        // println!("Done {}", s.cost);
        *minimal_cost = s.cost.min(*minimal_cost);
        return;
    }

    // println!("{s}");

    for i in 0..s.corridor.len() {
        let ch = s.corridor[i];
        if ch == '.' {
            continue;
        }
        let position_in_room = can_enter(s.room(ch), ch);
        if position_in_room.is_none() {
            continue;
        }
        let ci = corridor_index(ch);
        let can_reach = if i < ci {
            s.corridor[i + 1..=ci].iter().all(|&x| x == '.')
        } else {
            s.corridor[ci..i].iter().all(|&x| x == '.')
        };
        if !can_reach {
            continue;
        }

        // optimal move (go deeper, don't try other options later)
        let mut next = s.clone();
        let ri = position_in_room.unwrap();
        next.corridor[i] = '.';
        next.room_mut(ch)[position_in_room.unwrap()] = ch;
        next.cost += (i.abs_diff(ci) + 1 + ri) as i32 * cost(ch);

        search(&next, minimal_cost);
        return;
    }

    // let room_index = 2; // A is close to index 2 in the corridor
    // let ri = 0; // Index in the room
    // let room_index = 2;
    // let fa = 2; // from allowed i=2
    search_room(s, minimal_cost, 0, 'A');
    search_room(s, minimal_cost, 1, 'A');

    search_room(s, minimal_cost, 0, 'B');
    search_room(s, minimal_cost, 1, 'B');

    search_room(s, minimal_cost, 0, 'C');
    search_room(s, minimal_cost, 1, 'C');

    search_room(s, minimal_cost, 0, 'D');
    search_room(s, minimal_cost, 1, 'D');
}

fn search_room(s: &State, minimal_cost: &mut i32, ri: usize, native: char) {
    let s_room = s.room(native);
    let room_index = corridor_index(native);

    if s_room[ri] != '.' {
        if ri == 1 && s_room[0] != '.' {
            return; // cannot move
        }
        let ch = s_room[ri];
        if room_index == corridor_index(ch) {
            if ri == 1 {
                return; // already in best place
            } else {
                if s_room[1] == ch {
                    return; // room complete
                }
                if s_room[1] == '.' {
                    println!(" NOOOOOO ")
                    // only makes sense to go there
                    // should not happen, make sure in going back from corridor
                }
            }
        }

        // At this point, we can and should move out, all possibilities below
        // println!("{native}[{ri}] = {ch}");

        let mut i = allowed_on_right(native);
        while i < ALLOWED.len() {
            let pos = ALLOWED[i];
            if s.corridor[pos] == '.' {
                let mut next = s.clone();
                next.room_mut(native)[ri] = '.';
                next.corridor[pos] = ch;
                next.cost += pos.abs_diff(room_index - (ri + 1)) as i32 * cost(ch);

                search(&next, minimal_cost);
            } else {
                break; // someone is standing here
            }
            i += 1;
        }
        let mut i: i32 = (allowed_on_right(native) - 1) as i32;
        while i >= 0 {
            let pos = ALLOWED[i as usize];
            if s.corridor[pos] == '.' {
                let mut next = s.clone();
                next.room_mut(native)[ri] = '.';
                next.corridor[pos] = ch;
                next.cost += (pos.abs_diff(room_index) + 1 + ri) as i32 * cost(ch);

                // println!("{s}\n");
                // println!("{next}\n");
                // panic!("wait");

                search(&next, minimal_cost);
            } else {
                break; // someone is standing here
            }
            i -= 1;
        }
    }
}
