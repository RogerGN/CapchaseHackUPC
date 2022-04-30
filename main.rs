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

fn build_matrix(width, height) {
    /*
    It builds a matrix initializing it to 0
    */

    let matrix = [];
    for i in 0..width {
        matrix.push([]);
        for j in 0..height {
            matrix[i].push(0);
        }
    }

    return matrix;
}

fn find_neighbours_positions(current_position, map, width, height) {
    let neighbours_positions = [];

    let x = current_position[0];
    let y = current_position[1];

    let target_x = 0;
    let target_y = 0;

    // left
    target_x = x - 1;
    target_y = y;

    if target_x >= 0 && target_x < width && target_y >= 0 && target_y < height {
        // legal move
        neighbours_positions.push([target_x, target_y])
    }

    // top
    target_x = x;
    target_y = y + 1;

    if target_x >= 0 && target_x < width && target_y >= 0 && target_y < height {
        // legal move
        neighbours_positions.push([target_x, target_y])
    }

    // right
    target_x = x + 1;
    target_y = y;

    if target_x >= 0 && target_x < width && target_y >= 0 && target_y < height {
        // legal move
        neighbours_positions.push([target_x, target_y])
    }

    // bottom
    target_x = x;
    target_y = y - 1;

    if target_x >= 0 && target_x < width && target_y >= 0 && target_y < height {
        // legal move
        neighbours_positions.push([target_x, target_y])
    }

    return neighbours_positions;
}

// no checks
fn move_to_position(current_x, current_y, dest_x, dest_y) {
    return [];
}

fn find_team_color() {
    return worker(0).color;
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

fn find_closest_colorable_tiles_to_corner_position(n, target_position, map, width, height, team_color) {
    /*
    It finds the first "n" closest colorable tiles to the target position (it only works for corner positions)

    A colorable tile is a tile that is empty or colored with an enemy color
    */

    // Visited matrix
    let visited_matrix = build_matrix(width, height);

    // I should run a bfs starting from the target position
    let closest_colorable_tiles = [];

    // Init bfs queue
    let queue = [target_position];
    let target_x = target_position[0];
    let target_y = target_position[1];
    visited_matrix[target_x][target_y] = 1;

    while len(queue) > 0 && len(closest_colorable_tiles) < n {
        let current_position = queue.remove(0);
        let x = current_position[0];
        let y = current_position[1];

        if map[x][y] != team_color {
            closest_colorable_tiles.push(current_position);
        }

        // Add neighbours to the queue
        let neighbours_positions = find_neighbours_positions(current_position, map, width, height);
        for neighbour_position in neighbours_positions {
            let neighbour_x = neighbour_position[0];
            let neighbour_y = neighbour_position[1];
            if visited_matrix[neighbour_x][neighbour_y] == 0 {
                // if not visited yet
                queue.push(neighbour_position);
                visited_matrix[neighbour_x][neighbour_y] = 1;
            }
        }
    }
    
    return closest_colorable_tiles;
}


fn dump_positions(positions) {
    for i in 0..len(positions) {
        let position = positions[i];
        let x = position[0];
        let y = position[1];

        info(`position ${i}: [${x}][${y}]`);
    }
}


let corner_position = find_corner_position(width, height);
let team_color = find_team_color();
info(`corner position: ${corner_position}`);
info(`team color: ${team_color}`);

let closest_colorable_tiles = find_closest_colorable_tiles_to_corner_position(8, corner_position, map, width, height, team_color);
dump_positions(closest_colorable_tiles);

for w in 0..8 {
    // get useful tiles
    // useful_tiles = get_useful_tiles();

    // choose randomly between useful tiles
    // let choice = (rand() % len(useful_tiles)).abs();
    // position = useful_tiles[choice];

    // move_to_position(worker(w).x, worker(w).y, po);

}