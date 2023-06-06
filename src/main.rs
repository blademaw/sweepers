// Me trying to write minsweeper in Rust...
// FIXME: refactor file structure to split up board/tile/etc. implementation

use rand::Rng;
use std::{collections::HashSet, fmt::Debug};

enum GameStatus {
    Win,
    InPlay,
    Loss,
}

struct GameState {
    game_status: GameStatus,
    board: Board,
}

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

    fn get_orthogonal_neighbors(&self, row: usize, col: usize) -> Vec<&Tile> {
        let mut neighbors: Vec<&Tile> = Vec::with_capacity(4);

        if row > 0 {
            neighbors.push(&self.tiles[row - 1][col]);
        }

        if row < self.row_size - 1 {
            neighbors.push(&self.tiles[row + 1][col]);
        }

        if col > 0 {
            neighbors.push(&self.tiles[row][col - 1]);
        }

        if col < self.col_size - 1 {
            neighbors.push(&self.tiles[row][col + 1]);
        }

        return neighbors;
    }

    fn uncover_tile(&mut self, loc: (usize, usize)) -> Result<GameStatus, String> {
        // handles trying to uncover a tile
        let (row, col) = loc;

        if self.tiles[row][col].is_flagged {
            return Err("You can't uncover a flagged tile.".to_string());
        }

        match self.tiles[row][col].content {
            TileValue::Bomb => Ok(GameStatus::Loss),
            TileValue::Empty => {
                self.clear_single_tile(loc);
                Ok(GameStatus::InPlay)
            },
            TileValue::Value(_) => {
                self.clear_multiple_tiles(loc)
            }
        }
    }

    fn count_surrounding_bombs(&self, loc: (usize, usize)) -> usize {
        let (row, col) = loc;
        let mut bombs: usize = 0;

        for i in (row.saturating_sub(1))..=(std::cmp::min(row + 1, self.row_size - 1)) {
            for j in (col.saturating_sub(1))..=(std::cmp::min(col + 1, self.col_size - 1)) {
                match self.tiles[i][j].content {
                    TileValue::Bomb => bombs += 1,
                    TileValue::Empty | TileValue::Value(_) => (),
                }
            }
        }

        return bombs;
    }

    fn clear_multiple_tiles(&mut self, origin: (usize, usize)) -> Result<GameStatus, String> {
        let (row, col) = origin;

        let neighbors = self.count_surrounding_bombs(origin);
        todo!()
    }

    // TODO: refactor this into (&mut self, locs: Vec<(usize, usize)>) and
    // make clearing tiles based on a list of locations to clear. this means
    // we can use one function to clear X tiles rather than have to functions
    // to handle clearing multiple tiles.
    fn clear_single_tile(&mut self, loc: (usize, usize)) {
        // clear the selected position and all orthogonally connected free
        // neighbors exhaustively
        let (row, col) = loc;

        let mut to_show: Vec<(usize, usize)> = vec![];
        let mut closed: Vec<&Tile> = vec![];
        let mut open: Vec<&Tile> = vec![&self.tiles[row][col]];

        while let Some(cur) = open.pop() {
            closed.push(cur);

            match cur.content {
                TileValue::Bomb => continue,
                TileValue::Value(_) => to_show.push((cur.y, cur.x)),
                TileValue::Empty => {
                    to_show.push((cur.y, cur.x));
                    open.extend(
                        self.get_orthogonal_neighbors(cur.y, cur.x)
                            .iter()
                            .filter(|t| !closed.contains(t)),
                    );
                }
            }
        }

        for (row, col) in to_show {
            self.tiles[row][col].status = TileStatus::Shown;
        }
    }
}

#[derive(Debug, PartialEq)]
struct Tile {
    x: usize,
    y: usize,
    content: TileValue,
    status: TileStatus,
    is_flagged: bool,
}

impl Tile {
    fn to_string(&self) -> String {
        match self.status {
            TileStatus::Hidden => "[/]".to_string(),
            TileStatus::Shown => match self.content {
                TileValue::Empty => "[ ]".to_string(),
                TileValue::Bomb => "[!]".to_string(),
                TileValue::Value(n) => format!("[{n}]"),
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

#[derive(Debug, PartialEq)]
enum TileValue {
    Empty,
    Bomb,
    Value(usize),
}

#[derive(Debug, PartialEq)]
enum TileStatus {
    Hidden,
    Shown,
}

fn generate_bombs(
    x_len: &usize,
    y_len: &usize,
    bombs: &usize,
    start: &(usize, usize),
) -> Vec<(usize, usize)> {
    assert!(
        *bombs < *x_len * *y_len,
        "Cannot instantiate more/as many bombs as there are tiles."
    ); // should this be here?

    let mut rng = rand::thread_rng();

    let mut all_pairs: Vec<(usize, usize)> = (0..*x_len)
        .flat_map(|x| (0..*y_len).map(move |y| (x, y)))
        .collect();
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

fn make_empty_board(row_size: usize, col_size: usize) -> Board {
    // initialize an empty board
    let tiles: Vec<Vec<Tile>> = (0..row_size)
        .map(|y| {
            (0..col_size)
                .map(|x| Tile {
                    x,
                    y,
                    content: TileValue::Empty,
                    status: TileStatus::Hidden,
                    is_flagged: false,
                })
                .collect()
        })
        .collect();

    return Board {
        row_size,
        col_size,
        tiles,
    };
}

fn init_board(row_size: usize, col_size: usize, bombs: usize, start: (usize, usize)) -> Board {
    let mut board: Board = make_empty_board(row_size, col_size);

    // find and set bomb positions
    let pairs = generate_bombs(&row_size, &col_size, &bombs, &start);
    for (i, j) in pairs {
        println!("Bomb at {i},{j}");
        board.tiles[i][j].content = TileValue::Bomb;
        update_bomb_counts(i, j, &mut board.tiles);
    }

    // clear starting tile
    board.tiles[start.0][start.1].status = TileStatus::Shown;

    return board;
}

fn main() {
    let bombs: usize = 3;
    let board_dims: (usize, usize) = (6, 6);
    let start: (usize, usize) = (0, 0);

    let mut board: Board = init_board(board_dims.0, board_dims.1, bombs, start);

    board.show_board();
    board.show_board_all();

    board.clear_single_tile(start);
    board.show_board();
}
