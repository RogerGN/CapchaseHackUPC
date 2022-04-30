/*
for w in map.workers {
    // Logic to check worker placement
}

for x in 0..40 {
    for y in 0..40 {
        if map[x][y] == Tile::EMPTY {
            // more logic
        }
        // other logic
    }
}
*/

// 0: false
// 1: true

// 400 hundreds positions to allocate

// I have to store already visited positions

// improve bfs to choose not adjacent tiles.

// worker that wants to 

// use a worker to attack other players, just to create gaps in enemy territories, We could build circles. 

// I assume map (40 by 40)
let width = 40;
let height = 40;

// version
let version = 1;
info(`version ${version}`);

let occupied_positions = [];
let number_of_occupied_positions = 0;

fn is_position_free(x, y, occupied_positions) {
    // search in occupied positions (linear search)
    for i in 0..len(occupied_positions) {
        if occupied_positions.x == x  && occupied_positions.y == y {
            return false;
        }
    }
    
    return true;
}

fn is_useful_tile(x, y, occupied_positions, number_of_occupied_positions, map, width, height) {
    if !is_position_free(x, y, occupied_positions, number_of_occupied_positions) {
        return false;
    }

    // search in occupied positions (linear search)
    for i in 0..number_of_occupied_positions {
        if occupied_positions.x == x  && occupied_positions.y == y {
            return false;
        }
    }
    
    return true;
}


fn get_neighbours(map, width, hwight, worker) {
    return [];
}

// no checks
fn move_to_position(current_x, current_y, dest_x, dest_y) {
    return [];
}

fn find_corner_position(width, height) {
    let position = [0, 0];
    if worker(0).x < (width / 2) {
        // left side
        if worker(0).y < (height / 2) {
            // bottom left
            position[0] = 0;
            position[1] = 0;
        } else {
            // top left
            position[0] = 0;
            position[1] = height - 1;
        }
    } else {
        // right side
        if worker(0).y < (height / 2) {
            // bottom right
            position[0] = width - 1;
            position[1] = 0;
        } else {
            // top right
            position[0] = width - 1;
            position[1] = height - 1;
        }
    }

    return position;
}

// info(`${is_position_free(0, 0, occupied_positions, number_of_occupied_positions)}`);

// w1 -> 3,2
// w2 -> 3,2

// move randomly to empty tiles

info(`corner position: ${find_corner_position(width, height)}`);

for w in 0..8 {
    // get useful tiles
    // useful_tiles = get_useful_tiles();

    // choose randomly between useful tiles
    // let choice = (rand() % len(useful_tiles)).abs();
    // position = useful_tiles[choice];

    // move_to_position(worker(w).x, worker(w).y, po);
    
    info(`worker ${w} finished`);
}