// Memory check
// Don't move (first move)
if "n_tickets" in memory == false {
    memory.n_tickets = 0;
    info(`first tick`);
    return;
} else {
    memory.n_tickets = memory.n_tickets + 1;
}

if memory.n_tickets == 10 {
    info(`10 tick`);
    return;
}

info(`not first tick`);

// I assume that the map is (40 by 40)
let WIDTH = 40;
let HEIGHT = 40;
let N_WORKERS = 8;

info(`after init workers`);


// I have to init the collision matrix with the positions of the workers
// The collision matrix keeps track of the actual or future positions of our workers
// and is used in order to avoid to move a worker in a tile that is or will be occipied by another worker during the next iteration
let COLLISION_MATRIX = build_matrix(WIDTH, HEIGHT);
info(`after build matrix`);

for w in 0..N_WORKERS {
    info(`worker w: $(w)`);
    info(`worker: ${worker(w).x}-${worker(w).y}`);

    let x = worker(w).x;
    let y = worker(w).y;

    COLLISION_MATRIX = matrix_set(COLLISION_MATRIX, x, y, HEIGHT, 1); // 1 means occupied by my worker - 0 means free
}

info(`after init first matrix`);

// Other workers matrix
let OTHER_WORKERS_MATRIX = build_matrix(WIDTH, HEIGHT);
for worker in map.workers {
    let x = worker.x;
    let y = worker.y;

    OTHER_WORKERS_MATRIX = matrix_set(OTHER_WORKERS_MATRIX, x, y, HEIGHT, 1); // 1 means occupied by other worke - 0 means free
}
for w in 0..N_WORKERS {
    let x = worker(w).x;
    let y = worker(w).y;

    OTHER_WORKERS_MATRIX = matrix_set(OTHER_WORKERS_MATRIX, x, y, HEIGHT, 0); // 1 means occupied by other worke - 0 means free
}

info(`after init other matrix`);

// full matrix
let FULL_MATRIX = build_matrix(WIDTH, HEIGHT);
for worker in map.workers {
    let x = worker.x;
    let y = worker.y;

    FULL_MATRIX = matrix_set(FULL_MATRIX, x, y, HEIGHT, 1); // 1 means occupied by other worke - 0 means free
}

info(`after init full matrix`);


fn build_matrix(width, height) {
    let matrix = [];

    for i in 0..width {
        for j in 0..height {
            matrix.push(0);
        }
    }

    return matrix;
}


fn matrix_get(matrix, x, y, height) {
    let index = x * height + y;

    return matrix[index];
}


fn matrix_set(matrix, x, y, height, value) {
    let index = x * height + y;

    matrix[index] = value;

    return matrix;
}


fn build_matrix_not_flat(width, height) {
    /*
        It builds a matrix and it initialize each element to 0
    */

    let matrix = [];
    info(`after allocate matrix`);
    for i in 0..width {
        matrix.push([]);
        for j in 0..height {
            matrix[i].push(0);
        }
    }
    info(`after matrix`);

    return matrix;
}


fn find_neighbours_positions(target_position, map, width, height) {
    /*
        It returns an array of positions that are adjacent (4-connectivity) to the target_position
        It returns only the legal positions, i.e. positions that are inside the map
    */
    let neighbours_positions = []; // array to return

    let target_x = target_position[0];
    let target_y = target_position[1];

    // left
    let candidate_x = target_x - 1;
    let candidate_y = target_y;

    if candidate_x >= 0 && candidate_x < width && candidate_y >= 0 && candidate_y < height {
        // legal move
        neighbours_positions.push([candidate_x, candidate_y])
    }

    // top
    candidate_x = target_x;
    candidate_y = target_y + 1;

    if candidate_x >= 0 && candidate_x < width && candidate_y >= 0 && candidate_y < height {
        // legal move
        neighbours_positions.push([candidate_x, candidate_y])
    }

    // right
    candidate_x = target_x + 1;
    candidate_y = target_y;

    if candidate_x >= 0 && candidate_x < width && candidate_y >= 0 && candidate_y < height {
        // legal move
        neighbours_positions.push([candidate_x, candidate_y])
    }

    // bottom
    candidate_x = target_x;
    candidate_y = target_y - 1;

    if candidate_x >= 0 && candidate_x < width && candidate_y >= 0 && candidate_y < height {
        // legal move
        neighbours_positions.push([candidate_x, candidate_y])
    }

    return neighbours_positions;
}


fn find_team_color() {
    /*
        It returns the color associated to the our team
    */
    return worker(0).color;
}


/*
fn find_corner_position(width, height) {
    /*
        It returns the position [x, y] of our spawning corner
        The computation is based on the workers, so you should call this function only once, in the beginning of the game
    */
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
*/


/*
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
*/


fn is_reacheable(position, other_workers_matrix, map, width, height) {
    let neighbours = find_neighbours_positions(position, map, width, height);
    for neighbour in neighbours {
        if matrix_get(other_workers_matrix, neighbour[0], neighbour[1], height) == 0 {
            return true;
        }
    }

    return false;
}


fn is_position_valid(target_position, width, height) {
    if target_position[0] < 0 || target_position[0] >= width || target_position[1] < 0 || target_position[1] >= height {
        return false;
    }

    return true;
}


fn find_closest_colorable_tiles_to_position_equal_distance(target_position, map, width, height, team_color, other_workers_matrix) {
    /*
    It returns the closest colorable tiles to the target position. If some tiles has the same distance, it returns all of them.
    A colorable tile is a tile that is empty or colored with an enemy color
    */

    if !is_position_valid(target_position, width, height) {
        info(`fcc pos not valid: ${target_position}`);
        return [];
    }

    // Visited matrix
    let visited_matrix = build_matrix(width, height);

    // I should run a bfs starting from the target position
    let closest_colorable_tiles = [];

    // Init bfs queue
    let level = 0;
    let queue = [[target_position, level]]; // position, level
    let target_x = target_position[0];
    let target_y = target_position[1];
    visited_matrix = matrix_set(visited_matrix, target_x, target_y, height, 1);
    let stop_level = -1;

    while len(queue) > 0 {
        level = level + 1;
        let current_position_level = queue.remove(0);
        let current_position = current_position_level[0];
        let current_level = current_position_level[1];
        let x = current_position[0];
        let y = current_position[1];

        if map[x][y] != team_color && is_reacheable([x, y], other_workers_matrix, map, width, height) {
            if stop_level == -1 || stop_level == current_level {
                closest_colorable_tiles.push(current_position);
                stop_level = current_level;
            }
            if current_level > stop_level {
                return closest_colorable_tiles;
            }
        }

        // Add neighbours to the queue
        let neighbours_positions = find_neighbours_positions(current_position, map, width, height);
        for neighbour_position in neighbours_positions {
            let neighbour_x = neighbour_position[0];
            let neighbour_y = neighbour_position[1];
            if matrix_get(visited_matrix, neighbour_x, neighbour_y, height) == 0 {
                // if it has not been visited yet
                queue.push([neighbour_position, level]);
                visited_matrix = matrix_set(visited_matrix, neighbour_x, neighbour_y, height, 1);
            }
        }
    }

    info(`fcc empty array`);
    
    return closest_colorable_tiles;
}


/*
fn dump_positions(positions) {
    for i in 0..len(positions) {
        let position = positions[i];
        let x = position[0];
        let y = position[1];
        info(`position ${i}: [${x}][${y}]`);
    }
}
*/


/*
fn compute_position_bfs(worker_position, target_position, collision_matrix, width, height) {
    // compute path from worker position to target position
    // I want to compute the next from worker position to target position
    // Visited matrix
    let visited_matrix = build_matrix(width, height);
    // Init bfs queue
    // I start from the destination
    let queue = [target_position];
    let target_x = target_position[0];
    let target_y = target_position[1];
    visited_matrix[target_x][target_y] = 1;
    while len(queue) > 0 {
        let current_position = queue.remove(0);
        if current_position == worker_position {
            // return neighbour in visited matrix
            let neighbours_positions = find_neighbours_positions(current_position, map, width, height);
            for neighbour in neighbours {
                if visited_matrix[neighbour[0]][neighbour[1]] == 1 {
                    return neighbour;
                }
            }
        }
        let x = current_position[0];
        let y = current_position[1];
        // Add neighbours to the queue
        let neighbours_positions = find_neighbours_positions(current_position, map, width, height);
        for neighbour_position in neighbours_positions {
            if collision_matrix[neighbour_position[0]][neighbour_position[1]] == 0 {
                let neighbour_x = neighbour_position[0];
                let neighbour_y = neighbour_position[1];
                if visited_matrix[neighbour_x][neighbour_y] == 0 {
                    // if it has not been visited yet
                    queue.push(neighbour_position);
                    visited_matrix[neighbour_x][neighbour_y] = 1;
                }
            }
        }
    }
    
    return [-1, -1];
}
*/


/*
fn move_to_position_2(worker_position, target_position, collision_matrix, full_matrix, width, height) {
    let position = compute_position_bfs(worker_position, target_position, full_matrix, width, height);
    if position[0] == -1 {
        return;
    }
    move_to_position(worker, target_position, collision_matrix);
}
*/


fn move_to_position(worker, target_position, collision_matrix, width, height) {
    // Move towards the target_position if possible.
    // Otherwise if there is an obstacle (a worker with the same color) it try to change direction otherwise it stops.

    if !is_position_valid(target_position, width, height) {
        info(`mtp invalid pos: ${target_position}`);
    }

    let target_x = target_position[0];
    let target_y = target_position[1];

    let local_target_x = 0;
    let local_target_y = 0;

    // x
    let x_difference = worker.x - target_x;
    if x_difference < 0 {
        // go right
        local_target_x = worker.x + 1;
        local_target_y = worker.y;
        if matrix_get(collision_matrix, local_target_x, local_target_y, height) == 0 {
            info(`old worker x: ${worker.x}`);
            worker.move_right();
            info(`new worker x: ${worker.x}`);
            collision_matrix = matrix_set(collision_matrix, local_target_x, local_target_y, height, 1);
            collision_matrix = matrix_set(collision_matrix, worker.x, worker.y, height, 0);
            return collision_matrix;
        }
    }
    if x_difference > 0 {
        // go left
        local_target_x = worker.x - 1;
        local_target_y = worker.y;
        if matrix_get(collision_matrix, local_target_x, local_target_y, height) == 0 {
            worker.move_left();
            collision_matrix = matrix_set(collision_matrix, local_target_x, local_target_y, height, 1);
            collision_matrix = matrix_set(collision_matrix, worker.x, worker.y, height, 0);
            return collision_matrix;
        }
    }

    // y
    let y_difference = worker.y - target_y;
    if y_difference < 0 {
        // go up
        local_target_x = worker.x;
        local_target_y = worker.y + 1;
        if matrix_get(collision_matrix, local_target_x, local_target_y, height) == 0 {
            worker.move_up();
            collision_matrix = matrix_set(collision_matrix, local_target_x, local_target_y, height, 1);
            collision_matrix = matrix_set(collision_matrix, worker.x, worker.y, height, 0);
            return collision_matrix;
        }
    }
    if y_difference > 0 {
        // go down
        local_target_x = worker.x;
        local_target_y = worker.y - 1;
        if matrix_get(collision_matrix, local_target_x, local_target_y, height) == 0 {
            worker.move_down();
            collision_matrix = matrix_set(collision_matrix, local_target_x, local_target_y, height, 1);
            collision_matrix = matrix_set(collision_matrix, worker.x, worker.y, height, 0);
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


fn evaluate_position_nearness(position, map, target_color, width, height) {
    // It counts the number of adjacent tiles (4-connected) between position and the tiles in map with the same color
    let count = 0;

    let neighbours = find_neighbours_positions(position, map, width, height);
    for neighbour in neighbours {
        if map[neighbour[0]][neighbour[1]] == target_color {
            count = count + 1;
        }
    }

    return count;
}


fn select_closest_position_from_connected_components(candidate_positions, map, target_color, width, height) {
    let max_nearness = -1;
    let max_nearness_index = -1;

    for i in 0..len(candidate_positions) {
        let current_position = candidate_positions[i];
        let nearness = evaluate_position_nearness(current_position, map, target_color, width, height);
        if max_nearness == -1 || nearness > max_nearness {
            max_nearness = nearness;
            max_nearness_index = i;
        }
    }

    return candidate_positions[max_nearness_index];
}


// I should do this only once
/*
if "corner_position" in memory == false {
    memory.corner_position = find_corner_position(WIDTH, HEIGHT);
}
*/
let team_color = find_team_color();

info(`after find color`);

info(`corner position: ${memory.corner_position}`);
info(`team color: ${team_color}`);

// For each worker find the closest empty or enemy tile
for w in 0..N_WORKERS {
    let worker = worker(w);
    let worker_position = [worker.x, worker.y];
    let positions = find_closest_colorable_tiles_to_position_equal_distance(worker_position, map, WIDTH, HEIGHT, team_color, OTHER_WORKERS_MATRIX);
    info(`${worker.x} - ${worker.y} - ${positions}`);
    if len(positions) > 0 {
        let position = [0, 0];

        // choose a random strategy
        let strategy = (rand() % 2).abs();

        // Select a random position from the proposed ones
        if strategy == 0 {
            let index = (rand() % len(positions)).abs();
            position = positions[index];
        } else {
            position = select_closest_position_from_connected_components(positions, map, team_color, WIDTH, HEIGHT);
        }

        COLLISION_MATRIX = move_to_position(worker, position, COLLISION_MATRIX, WIDTH, HEIGHT);
        // info(`${worker.x} - ${worker.y} moves to ${positions[0]}`);
    }
}
