// Me trying to write minsweeper in Rust...

use std::fmt::Debug;

use rand::Rng;

#[derive(Debug)]
struct Board {
    row_size: usize,
    col_size: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Board {
    fn show_board(&self) {
        let mut res = "".to_string();

        for row in &self.tiles {
            let mut row_str = "".to_string();
            for tile in row {
                row_str.push_str(&tile.to_string());
            }
            row_str.push_str("\n");
            res.push_str(&row_str);
        }

        println!("{res}");
    }

    fn show_board_all(&self) {
        let mut res = "".to_string();

        for row in &self.tiles {
            let mut row_str = "".to_string();
            for tile in row {
                row_str.push_str(&tile.to_string_all());
            }
            row_str.push_str("\n");
            res.push_str(&row_str);
        }

        println!("{res}");
    }
}

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    content: TileValue,
    status: TileStatus,
    flagged: bool,
}

impl Tile {
    fn to_string(&self) -> String {
        match self.status {
            TileStatus::Hidden => "[/]".to_string(),
            TileStatus::Shown => {
                match self.content {
                    TileValue::Empty => "[ ]".to_string(),
                    TileValue::Bomb => "[!]".to_string(),
                    TileValue::Value(n) => format!("[{n}]"),
                }
            },
        }
    }

    fn to_string_all(&self) -> String {
        match self.content {
            TileValue::Empty => "[ ]".to_string(),
            TileValue::Bomb => "[!]".to_string(),
            TileValue::Value(n) => format!("[{n}]"),
        }
    }
}

#[derive(Debug)]
enum TileValue {
    Empty,
    Bomb,
    Value(usize),
}

#[derive(Debug)]
enum TileStatus {
    Hidden,
    Shown,
}

fn get_bomb_locations(x_len: &usize, y_len: &usize, bombs: &usize, start: &(usize, usize)) -> Vec<(usize, usize)> {
    assert!(*bombs < *x_len * *y_len, "Cannot instantiate more/as many bombs as there are tiles."); // should this be here?

    let mut rng = rand::thread_rng();

    let mut all_pairs: Vec<(usize,usize)> = (0..*x_len).flat_map(|x| (0..*y_len).map(move |y| (x,y))).collect();
    _ = all_pairs.remove((*x_len - 1) * start.0 + start.1); // this is the start, meaning we cannot place a bomb in/on it

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

fn update_bomb_counts(row_ind: usize, col_ind: usize, tiles: &mut Vec<Vec<Tile>>) -> () {
    let row_max = tiles.len();
    let col_max = tiles[0].len();

    for i in (row_ind.saturating_sub(1))..=(std::cmp::min(row_ind + 1, row_max - 1)) {
        for j in (col_ind.saturating_sub(1))..=(std::cmp::min(col_ind + 1, col_max - 1)) {
            println!("{i},{j}");
            if i != row_ind || j != col_ind {
                let mut tile = tiles[i].get_mut(j).unwrap();
                match tile.content {
                    TileValue::Bomb => (),
                    TileValue::Empty => tile.content = TileValue::Value(1),
                    TileValue::Value(n) => tile.content = TileValue::Value(n + 1),
                }
            }
        }
    }
}

fn init_board(row_size: usize, col_size: usize, bombs: usize, start: (usize, usize)) -> Board {
    // TODO: make this faster/smarter

    // initialize an empty board
    let mut tiles: Vec<Vec<Tile>> = vec![];

    for y in 0..row_size {
        let mut row: Vec<Tile> = vec![];
        for x in 0..col_size {
            row.push(Tile { x, y, content: TileValue::Empty, status: TileStatus::Hidden, flagged: false });
        }
        tiles.push(row);
    }
    tiles[start.0][start.1].status = TileStatus::Shown;

    // find and set bomb positions
    let pairs = get_bomb_locations(&row_size, &col_size, &bombs, &start);
    for (i, j) in pairs {
        println!("Bomb at {i},{j}");
        tiles[i][j].content = TileValue::Bomb;
        update_bomb_counts(i, j, &mut tiles);

        // update surrounding tile counts
        // for tile in get_mut_neighbors(x, y, &mut tiles) {
        //     match tile.content {
        //         TileValue::Empty | TileValue::Bomb => (),
        //         TileValue::Value(n) => tile.content = TileValue::Value(n + 1),
        //     }
        // }
        
    }

    // TODO: update tile values with correct number of bombs
    

    return Board { row_size, col_size, tiles };
}

fn main() {
    let mut board: Board = init_board(3, 3, 1, (0,0));

    println!("{board:#?}");
    board.show_board();
    board.show_board_all();
}
