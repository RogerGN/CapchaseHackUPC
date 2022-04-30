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

let n_workers = 8;

// I have to init collision matrix with positions of workers
let collision_matrix = build_matrix(width, height);

for w in 0..n_workers {
    let x = worker(w).x;
    let y = worker(w).y;

    collision_matrix[x][y] = "w";
}

info(`collision matrix ${collision_matrix[34][33]}`);

// version
let version = 1;
info(`version ${version}`);

let occupied_positions = [];
let number_of_occupied_positions = 0;

fn build_matrix(width, height) {
    /*
    It builds a matrix initializing it to 0
    */

    let matrix = [];
    for i in 0..width {
        matrix.push([]);
        for j in 0..height {
            matrix[i].push("-");
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

fn find_closest_colorable_tiles_to_position(n, target_position, map, width, height, team_color) {
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
    visited_matrix[target_x][target_y] = "w";

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
            if visited_matrix[neighbour_x][neighbour_y] != "w" {
                // if not visited yet
                queue.push(neighbour_position);
                visited_matrix[neighbour_x][neighbour_y] = "w";
            }
        }
    }
    
    return closest_colorable_tiles;
}


fn find_closest_colorable_tiles_to_position_equal_distance(target_position, map, width, height, team_color) {
    /*
    It finds the first "n" closest colorable tiles to the target position (it only works for corner positions)
    It finds all tiles at the same distance

    A colorable tile is a tile that is empty or colored with an enemy color
    */

    // Visited matrix
    let visited_matrix = build_matrix(width, height);

    // I should run a bfs starting from the target position
    let closest_colorable_tiles = [];

    // Init bfs queue
    let level = 0;
    let queue = [[target_position, level]]; // position, level
    let target_x = target_position[0];
    let target_y = target_position[1];
    visited_matrix[target_x][target_y] = "w";
    let found_level = -1;

    while len(queue) > 0 {
        level = level + 1;
        let current_position_level = queue.remove(0);
        let current_position = current_position_level[0];
        let current_level = current_position_level[1];
        let x = current_position[0];
        let y = current_position[1];

        if map[x][y] != team_color {
            if found_level == -1 || found_level == current_level {
                closest_colorable_tiles.push(current_position);
                found_level = current_level;
            }
            if current_level > found_level {
                return closest_colorable_tiles;
            }
        }

        // Add neighbours to the queue
        let neighbours_positions = find_neighbours_positions(current_position, map, width, height);
        for neighbour_position in neighbours_positions {
            let neighbour_x = neighbour_position[0];
            let neighbour_y = neighbour_position[1];
            if visited_matrix[neighbour_x][neighbour_y] != "w" {
                // if not visited yet
                queue.push([neighbour_position, level]);
                visited_matrix[neighbour_x][neighbour_y] = "w";
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

fn move_to_position(worker, position, collision_matrix) {
    // move towards the position if possible

    // just move to position for now

    let target_x = position[0];
    let target_y = position[1];

    let local_target_x = 0;
    let local_target_y = 0;

    // x
    let x_difference = worker.x - target_x;
    if x_difference < 0 {
        // go left
        local_target_x = worker.x + 1;
        local_target_y = worker.y;
        if collision_matrix[local_target_x][local_target_y] != "w" {
            worker.move_right();
            collision_matrix[local_target_x][local_target_y] = "w";
            collision_matrix[worker.x][worker.y] = "c";
            return collision_matrix;
        }
    }
    if x_difference > 0 {
        // go right
        local_target_x = worker.x - 1;
        local_target_y = worker.y;
        if collision_matrix[local_target_x][local_target_y] != "w" {
            worker.move_left();
            collision_matrix[local_target_x][local_target_y] = "w";
            collision_matrix[worker.x][worker.y] = "c";
            return collision_matrix;
        }
    }


    // y
    let y_difference = worker.y - target_y;
    if y_difference < 0 {
        // go up
        local_target_x = worker.x;
        local_target_y = worker.y + 1;
        if collision_matrix[local_target_x][local_target_y] != "w" {
            worker.move_up();
            collision_matrix[local_target_x][local_target_y] = "w";
            collision_matrix[worker.x][worker.y] = "c";
            return collision_matrix;
        }
    }
    if y_difference > 0 {
        // go down
        local_target_x = worker.x;
        local_target_y = worker.y - 1;
        if collision_matrix[local_target_x][local_target_y] != "w" {
            worker.move_down();
            collision_matrix[local_target_x][local_target_y] = "w";
            collision_matrix[worker.x][worker.y] = "c";
            return collision_matrix;
        }
    }

    // otherwise stay still
    return collision_matrix;

}

fn compute_distance(position_a, position_b) {
    let distance = abs(position_a[0] - position_b[0]) + abs(position_a[1] - position_b[1]);

    return distance;
}

// I should do this only once
if memory.registered_corner_position != `` {
    memory.corner_position = find_corner_position(width, height);
    memory.registered_corner_position = ``;
}
let team_color = find_team_color();
info(`corner position: ${memory.corner_position}`);
info(`team color: ${team_color}`);

let closest_colorable_tiles = find_closest_colorable_tiles_to_position(n_workers, memory.corner_position, map, width, height, team_color);
// dump_positions(closest_colorable_tiles);

if len(closest_colorable_tiles) < n_workers {
    // add random positions
    let remaining_positions_number = n_workers - len(closest_colorable_tiles);
    for i in 0..remaining_positions_number {
        let random_x = (rand() % width).abs();
        let random_y = (rand() % height).abs();
        closest_colorable_tiles.push([random_x, random_y]);
    }
}

/*
for w in 0..n_workers {
    let min_distance = 1000;
    let position_index = 1000;
    let worker = worker(w);
    for i in 0..len(closest_colorable_tiles) {

        // I should not change the target once I chose the it.
        let position = closest_colorable_tiles[i];

        let distance = compute_distance(position, [worker.x, worker.y]);
        if distance < min_distance {
            min_distance = distance;
            position_index = i;
        }
    }

    collision_matrix = move_to_position(worker, closest_colorable_tiles[position_index], collision_matrix);
    info(`${closest_colorable_tiles[position_index]} at ${worker.x} - ${worker.y}`);
}
*/

// for each worker find the closest empty or enemy tile
for w in 0..n_workers {
    let worker = worker(w);
    let worker_position = [worker.x, worker.y];
    let positions = find_closest_colorable_tiles_to_position_equal_distance(worker_position, map, width, height, team_color);
    // chose a random position
    // if positions is Null I have to chose random I think
    let index = (rand() % len(positions)).abs();
    collision_matrix = move_to_position(worker, positions[index], collision_matrix);
    info(`${positions[0]} at ${worker.x} - ${worker.y}`);
}


// dump corner color
//info(`corner color: ${map[39][39]}`);