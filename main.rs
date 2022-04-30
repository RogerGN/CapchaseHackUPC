
fn checkPositionInMap(positions) {
    return positions[0].x;
}


for w in map.workers {
    // Logic to check worker placement
    info(`Placement x ${w.x} and y ${w.y}`);
}


for x in 0..50 {
    for y in 0..50 {
        if map[x][y] == Tile::EMPTY {
            // more logic
            let test = 5;
        }
        // other logic
        let test1 = 5;
    }
}

info(`functvalue ${checkPositionInMap(map.workers)}`);


let version = 0;

info(`Version 0`);

for w in 0..8 {
    let r = (rand() % 4).abs();
    switch r {
        0 => worker(w).move_up(),
        1 => worker(w).move_down(),
        2 => worker(w).move_right(),
        3 => worker(w).move_left(),
    }
    
    //info(`worker ${w} finished`);
    //info(`worker ${worker(w).color} color`);
}