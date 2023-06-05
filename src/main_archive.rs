// Me trying to write minsweeper in Rust...

use rand::Rng;

#[derive(Debug)]
struct Board {
    row_size: usize,
    col_size: usize,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    content: TileValue,
}

#[derive(Debug)]
enum TileValue {
    Empty,
    Bomb,
    Value(usize),
}

fn get_bomb_locations(x_len: &usize, y_len: &usize, bombs: &usize) -> Vec<(usize, usize)> {
    assert!(*bombs <= *x_len * *y_len, "Cannot instantiate more bombs than there are tiles."); // should this be here?

    let mut rng = rand::thread_rng();

    let mut all_pairs: Vec<(usize,usize)> = (0..*x_len).flat_map(|x| (0..*y_len).map(move |y| (x,y))).collect();
    let mut num_pairs = all_pairs.len();
    let mut assigned_bombs = 0;

    let mut pairs: Vec<(usize, usize)> = vec![];

    while assigned_bombs < *bombs {
        let rand_pair_ind = rng.gen_range(0..num_pairs);

        pairs.push(all_pairs.remove(rand_pair_ind));

        num_pairs -= 1;
        assigned_bombs += 1;
    }

    return pairs;
}

fn init_board(row_size: usize, col_size: usize, bombs: usize) -> Board {
    // TODO: make this faster/smarter

    // initialize an empty board
    let mut tiles: Vec<Vec<Tile>> = vec![];

    for y in 0..row_size {
        tiles.push((0..col_size).map(|x| Tile { x, y, content: TileValue::Empty }).collect());
    }

    // determine where the bombs are going
    let pairs = get_bomb_locations(&row_size, &col_size, &bombs);

    for (x, y) in pairs {
        tiles[x][y].content = TileValue::Bomb;
    }

    // TODO: update tile values with correct number of bombs

    Board { row_size, col_size, tiles }
}

fn main() {
    let mut board: Board = init_board(2, 2, 5);

    println!("{board:#?}");
}
