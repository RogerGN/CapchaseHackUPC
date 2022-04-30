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
        memory.spawningCorner = "top-right";
    }
    
    if(xPosition > width/2 && yPosition < height/2)
    {
        memory.spawningCorner = "top-left";
    }

    if(xPosition < width/2 && yPosition > height/2)
    {
        memory.spawningCorner = "bottom-right";
    }

    if(xPosition > width/2 && yPosition > height/2)
    {
        memory.spawningCorner = "bottom-left";
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

fn moveWorkers(n_workers, width, height, collision_matrix, worker, map, workersToIgnore)
{
    for w in 0..n_workers
    {
        if !workersToIgnore.contains(w)
        {
            let worker = worker(w);
            info(`Workeraaaaa ${worker}`);
            let worker_position = [worker.x, worker.y];
            let positions = find_closest_colorable_tiles_to_position_equal_distance(worker_position, map, width, height, worker.color);
            let index = (rand() % len(positions)).abs();
            collision_matrix = move_to_position(worker, positions[index], collision_matrix);
        }
    }
    /*
    for w in workersToIgnore
    {

    }
    */
}

info(`Tick ${memory.tick}`);
info(`SpawningCorner ${memory.spawningCorner}`);
info(`worker ${worker(0).color} color`);

if(memory.strategy == 0) //expand
{
    switch memory.spawningCorner {
        "top-right" => moveWorkers(n_workers, width, height, collision_matrix, map.workers, map, [6,5,3]),
        "top-left" => moveWorkers(n_workers, width, height, collision_matrix, map.workers, map, [7,6,4]),
        "bottom-right" => moveWorkers(n_workers, width, height, collision_matrix, map.workers, map, [3,0,1]),
        "bottom-left" => moveWorkers(n_workers, width, height, collision_matrix, map.workers, map, [4,2,1]),
    }
}

