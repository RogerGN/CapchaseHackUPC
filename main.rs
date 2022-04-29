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
    
    info(`worker ${w} finished`);
}