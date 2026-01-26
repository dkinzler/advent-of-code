use std::collections::HashMap;

fn main() {
    // we can view this as
    // every piece moves at most twice
    // either directly from its starting to ending position
    // or from starting position to some position in the hallway
    // and then to its ending position
    //
    // we have 8 pods and for each there are 7+2 = 9 possible
    // positions it can move to
    // -> 9^8 ~ 40 mil, could test all those out?
    // or could do backtracking
    // for the 16 pods it is slightly larger
    //     but not all can move at the same time and so on, there
    //     are constraints
    //     so search space might be manageable?
    //     and certain situations are not allowed
    //     e.g. we can only move to a target column if that colum is empty
    //     or only contains pods of the same color
    // would need to precompute a map of distances from one position to another?
    // maybe with two data structures a map and a list of positon for each
    // that way can easily do all the lookups
    //
    // we can do this via backtracking
    // but how do we represent the problem efficiently?
    // could do it via a structure for the slots
    // and one for the hallway?
    // then just need a function to compute distances between locations?

    // part 1
    // let rooms = vec![[3, 1, 2, 4], [3, 1, 4, 2]];
    // part 2
    let rooms = vec![[3, 1, 2, 4], [4, 3, 2, 1], [4, 2, 1, 3], [3, 1, 4, 2]];
    let mut map = Map::new(rooms);
    let mut mem = HashMap::new();
    let result = backtrack(&mut map, &mut mem).unwrap();
    println!("part2: {result}");
}

fn backtrack(map: &mut Map, mem: &mut HashMap<(u64, u64), Option<u64>>) -> Option<u64> {
    //map.debug_print();
    if let Some(v) = mem.get(&map.key()) {
        return *v;
    }

    if map.is_valid() {
        return Some(0);
    }

    let mut min = 1 << 60;

    // try to move from room to correct room
    for room in 0..4 {
        for pos in 0..map.room_size {
            let c = map.rooms[pos][room];
            if c > 0 && c < 5 {
                if let Some((cost, target_pos)) = map.can_move_between_rooms(room, pos) {
                    map.move_from_room_to_room(room, pos, c - 1, target_pos, 5);
                    if let Some(b_cost) = backtrack(map, mem) {
                        let total_cost = cost + b_cost;
                        if total_cost < min {
                            min = total_cost;
                        }
                    }
                    map.move_from_room_to_room(c - 1, target_pos, room, pos, c);
                }
            }
        }
    }

    // try to move from room to hallway
    for room in 0..4 {
        for pos in 0..map.room_size {
            let c = map.rooms[pos][room];
            if c > 0 && c < 5 {
                for i in [0, 1, 3, 5, 7, 9, 10] {
                    if let Some(cost) = map.can_move_from_room_to_hallway(room, pos, i) {
                        map.move_from_room(room, pos, i, c);
                        if let Some(b_cost) = backtrack(map, mem) {
                            let total_cost = cost + b_cost;
                            if total_cost < min {
                                min = total_cost;
                            }
                        }
                        map.move_from_hallway(i, room, pos, c);
                    }
                }
            }
        }
    }

    // try to move from hallway to correct room
    for i in 0..11 {
        let c = map.hallway[i];
        if c > 0 {
            if let Some((cost, pos)) = map.can_move_from_hallway_to_room(i) {
                map.move_from_hallway(i, c - 1, pos, 5);
                if let Some(b_cost) = backtrack(map, mem) {
                    let total_cost = cost + b_cost;
                    if total_cost < min {
                        min = total_cost;
                    }
                }
                map.move_from_room(c - 1, pos, i, c);
            }
        }
    }
    let result = if min < (1 << 60) { Some(min) } else { None };
    mem.insert(map.key(), result);
    result
}

const ROOM_ENTRANCES: [usize; 4] = [2, 4, 6, 8];
const COLOR_TO_ENERGY: [u64; 4] = [1, 10, 100, 1000];

struct Map {
    room_size: usize,
    // entries can be
    // 0 = empty
    // 1 = A
    // 2 = B
    // 3 = C
    // 4 = D
    // 5 = pod that can't move anymore
    // will use 5 only for pods that are in the
    // correct room and don't have to move anymore
    rooms: Vec<[usize; 4]>,
    hallway: [usize; 11],
}

impl Map {
    fn new(rooms: Vec<[usize; 4]>) -> Self {
        let room_size = rooms.len();
        let mut rooms = rooms;
        // mark any pods that are already in the correct position
        // as 5
        for room in 0..4 {
            let color = room + 1;
            for pos in (0..room_size).rev() {
                if rooms[pos][room] == color {
                    rooms[pos][room] = 5;
                } else {
                    break;
                }
            }
        }
        Map {
            room_size: room_size,
            rooms: rooms,
            hallway: [0; 11],
        }
    }

    fn is_valid(&self) -> bool {
        for room in 0..4 {
            for pos in 0..self.room_size {
                if self.rooms[pos][room] < 5 {
                    return false;
                }
            }
        }
        return true;
    }

    fn move_from_hallway(&mut self, i: usize, room: usize, room_pos: usize, color: usize) {
        self.hallway[i] = 0;
        self.rooms[room_pos][room] = color;
    }

    fn move_from_room(&mut self, room: usize, room_pos: usize, i: usize, color: usize) {
        self.hallway[i] = color;
        self.rooms[room_pos][room] = 0;
    }

    fn move_from_room_to_room(
        &mut self,
        room: usize,
        room_pos: usize,
        target_room: usize,
        target_pos: usize,
        color: usize,
    ) {
        self.rooms[room_pos][room] = 0;
        self.rooms[target_pos][target_room] = color;
    }

    // pods cannot stay on hallway positions
    // that are the entrace to a room
    // fn is_valid_hallway(i: usize) -> bool {
    //     match i {
    //         2 | 4 | 6 | 8 => false,
    //         0..11 => true,
    //         _ => panic!("invalid hallway position"),
    //     }
    // }

    fn is_empty_hallway(&self, i: usize) -> bool {
        self.hallway[i] == 0
    }

    fn can_move_hallway(&self, from: usize, to: usize) -> bool {
        let start;
        let end;
        if from <= to {
            start = from + 1;
            end = to;
        } else {
            start = to;
            end = from - 1;
        }
        for x in start..=end {
            if !self.is_empty_hallway(x) {
                return false;
            }
        }
        true
    }

    fn hallway_dist(i: usize, j: usize) -> u64 {
        if i <= j {
            (j - i) as u64
        } else {
            (i - j) as u64
        }
    }

    // whether or not a pod of the given color
    // could move into its correct room
    // i.e. the room must either be empty or only contain
    // pods of the correct color
    fn can_move_into_room(&self, color: usize) -> Option<usize> {
        let mut empty_pos = 0;
        let room = color - 1;
        for i in 0..self.room_size {
            match self.rooms[i][room] {
                0 => empty_pos = i,
                5 => {}
                // this case could happen if
                // a pod is initially already in the correct room
                // at the bottom
                c if c == color => {}
                _ => return None,
            }
        }
        return Some(empty_pos);
    }

    // if the move is possible returns the required energy
    // otherwise None
    fn can_move_from_room_to_hallway(&self, room: usize, pos: usize, i: usize) -> Option<u64> {
        for x in 0..pos {
            if self.rooms[x][room] != 0 {
                return None;
            }
        }

        if !self.can_move_hallway(ROOM_ENTRANCES[room], i) {
            return None;
        }

        // compute cost
        let color = self.rooms[pos][room];
        let n_steps = pos as u64 + 1 + Map::hallway_dist(ROOM_ENTRANCES[room], i);
        let cost = n_steps * COLOR_TO_ENERGY[color - 1];
        return Some(cost);
    }

    // note here that a pod with color c can only move
    // into the room for color c
    fn can_move_from_hallway_to_room(&self, i: usize) -> Option<(u64, usize)> {
        let color = self.hallway[i];
        let room = color - 1;

        let room_entrance = ROOM_ENTRANCES[room];
        if !self.can_move_hallway(i, room_entrance) {
            return None;
        }

        if let Some(pos) = self.can_move_into_room(color) {
            // compute cost
            let n_steps = pos as u64 + 1 + Map::hallway_dist(room_entrance, i);
            let cost = n_steps * COLOR_TO_ENERGY[color - 1];
            return Some((cost, pos));
        }
        return None;
    }

    // if a pod in the given room can move directly
    // to its correct room
    fn can_move_between_rooms(&self, room: usize, pos: usize) -> Option<(u64, usize)> {
        let color = self.rooms[pos][room];
        let target_room = color - 1;

        let hallway_start = ROOM_ENTRANCES[room];
        let hallway_end = ROOM_ENTRANCES[target_room];
        if !self.can_move_hallway(hallway_start, hallway_end) {
            return None;
        }

        // check that there is no pod above in the room
        // i.e. the way out is clear
        for i in 0..pos {
            if self.rooms[i][room] != 0 {
                return None;
            }
        }
        if let Some(target_pos) = self.can_move_into_room(color) {
            // compute cost
            let n_steps = target_pos as u64
                + 1
                + pos as u64
                + 1
                + Map::hallway_dist(hallway_start, hallway_end);
            let cost = n_steps * COLOR_TO_ENERGY[color - 1];
            return Some((cost, target_pos));
        }
        return None;
    }

    fn key(&self) -> (u64, u64) {
        let mut k1 = 0;
        // can have values 0 to 5
        // use 3 bits each
        for i in 0..11 {
            k1 |= (self.hallway[i] as u64) << (3 * i);
        }
        let mut k2 = 0;
        for room in 0..4 {
            for pos in 0..self.room_size {
                k2 |= (self.rooms[pos][room] as u64) << (3 * (room * self.room_size + pos));
            }
        }
        (k1, k2)
    }

    fn debug_print(&self) {
        let mut hallway_str = String::new();
        for c in self.hallway.iter() {
            match *c {
                0 => hallway_str.push('.'),
                1 => hallway_str.push('A'),
                2 => hallway_str.push('B'),
                3 => hallway_str.push('C'),
                4 => hallway_str.push('D'),
                5 => hallway_str.push('X'),
                _ => panic!(),
            }
        }
        println!("{hallway_str}");

        for pos in 0..self.room_size {
            let mut row_str = String::from("  ");
            for room in 0..4 {
                match self.rooms[pos][room] {
                    0 => row_str.push_str(". "),
                    1 => row_str.push_str("A "),
                    2 => row_str.push_str("B "),
                    3 => row_str.push_str("C "),
                    4 => row_str.push_str("D "),
                    5 => row_str.push_str("X "),
                    _ => panic!(),
                }
            }
            println!("{row_str}");
        }
        println!();
    }
}
