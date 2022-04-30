// version
let version = 1;
info(`version ${version}`);

//global variables
let width = 40;
let height = 40;
let n_workers = 8;
let collision_matrix = build_matrix(width, height);

//memory initialize ///////////////////////////////////////////////
if "tick" in memory == false {
    memory.tick = 0;
} else {
    memory.tick += 1;
}

if "strategy" in memory == false {
    memory.strategy = 0;
}

if "spawningCorner" in memory == false {
    let xPosition = map.workers[0].x;
    let yPosition = map.workers[0].y;
    
    memory.spawningCorner = "none";
    if(xPosition < width/2 && yPosition < height/2)
    {
        memory.spawningCorner = "bottom-left";
    }
    
    if(xPosition > width/2 && yPosition < height/2)
    {
        memory.spawningCorner = "bottom-right";
    }

    if(xPosition < width/2 && yPosition > height/2)
    {
        memory.spawningCorner = "top-left";
    }

    if(xPosition > width/2 && yPosition > height/2)
    {
        memory.spawningCorner = "top-right";
    }

}

if "map" in memory == false {
    let matrix = [];
    for i in 0..width {
        matrix.push([]);
        for j in 0..height {
            matrix[i].push("-");
        }
    }

    for w in map.workers {
        matrix[w.x][w.y] = "w"
    }    
    memory.map = matrix;
}
//////////////////////////////////////////////////////////////////////


//functions


/*

let occupied_positions = [];
let number_of_occupied_positions = 0;

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
*/

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

fn compute_distance(position_a, position_b) {
    let distance = abs(position_a[0] - position_b[0]) + abs(position_a[1] - position_b[1]);

    return distance;
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



//Main
/*
let collision_matrix = build_matrix(width, height);

for w in 0..n_workers {
    let x = worker(w).x;
    let y = worker(w).y;
    collision_matrix[x][y] = "w";
}
*/

fn lookOnlyForNodesAtTheFrontIfPossible(positions, worker_position, spawnCorner)
{
    let notPrioritaryNodesQueue = []; 
    switch spawnCorner {
        "top-right" => {
            for p in 0..positions.len()-1
            {
                if(positions[p][0] < worker_position[0])
                {
                    notPrioritaryNodesQueue.push(positions[p]);
                    positions.remove(p);
                }
            }
        },
        "top-left" => {
            for p in 0..positions.len()-1
            {
                if(positions[p][0] > worker_position[0])
                {
                    notPrioritaryNodesQueue.push(positions[p]);
                    positions.remove(p);
                }
            }
        },
        "bottom-right" => {
            for p in 0..positions.len()-1
            {
                if(positions[p][0] < worker_position[0])
                {
                    notPrioritaryNodesQueue.push(positions[p]);
                    positions.remove(p);
                }
            }
        },
        "bottom-left" => {
            for p in 0..positions.len()-1
            {
                if(positions[p][0] > worker_position[0])
                {
                    notPrioritaryNodesQueue.push(positions[p]);
                    positions.remove(p);
                }
            }
        },
    }

    return [positions, notPrioritaryNodesQueue];
}

fn lookOnlyForNodesAtTheBackIfPossible(positions, worker_position, spawnCorner)
{
    let notPrioritaryNodesQueue = []; 
    switch spawnCorner {
        "top-right" => {
            for p in 0..positions.len()-1
            {
                if(positions[p][0] < worker_position[0])
                {
                    notPrioritaryNodesQueue.push(positions[p]);
                    positions.remove(p);
                }
            }
        },
        "top-left" => {
            for p in 0..positions.len()-1
            {
                if(positions[p][0] > worker_position[0])
                {
                    notPrioritaryNodesQueue.push(positions[p]);
                    positions.remove(p);
                }
            }
        },
        "bottom-right" => {
            for p in 0..positions.len()-1
            {
                if(positions[p][0] < worker_position[0])
                {
                    notPrioritaryNodesQueue.push(positions[p]);
                    positions.remove(p);
                }
            }
        },
        "bottom-left" => {
            for p in 0..positions.len()-1
            {
                if(positions[p][0] > worker_position[0])
                {
                    notPrioritaryNodesQueue.push(positions[p]);
                    positions.remove(p);
                }
            }
        },
    }

    return [notPrioritaryNodesQueue,positions];
}

fn moveWorkers(n_workers, width, height, collision_matrix, spawnCorner, worker, map, workersToIgnore)
{
    for w in 0..n_workers
    {
        if !workersToIgnore.contains(w)
        {
            let worker = worker(w);
            let worker_position = [worker.x, worker.y];
            let positions = find_closest_colorable_tiles_to_position_equal_distance(worker_position, map, width, height, worker.color);
            positions = lookOnlyForNodesAtTheFrontIfPossible(positions, worker_position, spawnCorner);
            if(positions[0].len() != 0)
            {
                let index = (rand() % len(positions[0])).abs();
                collision_matrix = move_to_position(worker, positions[0][index], collision_matrix);
            }
            else
            {
                let index = (rand() % len(positions[1])).abs();
                collision_matrix = move_to_position(worker, positions[1][index], collision_matrix);
            }
            
        }
    }
    
    
    for w in workersToIgnore
    {
        let worker = worker(w);
            let worker_position = [worker.x, worker.y];
            let positions = find_closest_colorable_tiles_to_position_equal_distance(worker_position, map, width, height, worker.color);
            positions = lookOnlyForNodesAtTheBackIfPossible(positions, worker_position, spawnCorner);
            if(positions[0].len() != 0)
            {
                let index = (rand() % len(positions[0])).abs();
                collision_matrix = move_to_position(worker, positions[0][index], collision_matrix);
            }
            else
            {
                let index = (rand() % len(positions[1])).abs();
                collision_matrix = move_to_position(worker, positions[1][index], collision_matrix);
            }
    }    
}

info(`Tick ${memory.tick}`);
info(`SpawningCorner ${memory.spawningCorner}`);
info(`worker ${worker(0).color} color`);

if(memory.strategy == 0) //expand
{
    switch memory.spawningCorner {
        "top-right" => moveWorkers(n_workers, width, height, collision_matrix, memory.spawningCorner, map.workers, map, [6,5,3]),
        "top-left" => moveWorkers(n_workers, width, height, collision_matrix, memory.spawningCorner, map.workers, map, [7,6,4]),
        "bottom-right" => moveWorkers(n_workers, width, height, collision_matrix, memory.spawningCorner, map.workers, map, [3,0,1]),
        "bottom-left" => moveWorkers(n_workers, width, height, collision_matrix, memory.spawningCorner, map.workers, map, [4,2,1]),
    }
}

