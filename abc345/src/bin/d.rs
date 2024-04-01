use itertools::Itertools;
use proconio::input;

fn check(
    field_size: (usize, usize),
    tiles: Vec<(usize, usize)>,
    tile_order: Vec<(usize, usize)>,
) -> bool {
    let mut field = vec![vec![0; field_size.1]; field_size.0];
    let mut tile_order = tile_order.clone();

    for r in 0..field_size.0 {
        for c in 0..field_size.1 {
            if field[r][c] == 0 {
                let mut next_tile: (usize, usize);
                // println!("!!!");
                let Some((idx, side)) = tile_order.pop() else {
                    // println!("!!{:?}", field);
                    return false;
                };
                let mut next_tile: (usize, usize);
                if side == 1 {
                    let tile = tiles[idx];
                    next_tile = (tile.1, tile.0);
                } else {
                    let tile = tiles[idx];
                    next_tile = (tile.0, tile.1);
                }
                // println!("next: {:?}", next_tile);

                if r + next_tile.0 <= field_size.0 && c + next_tile.1 <= field_size.1 {
                    for rr in 0..(next_tile.0) {
                        for cc in 0..(next_tile.1) {
                            if field[r + rr][c + cc] == 1 {
                                // println!("!{:?}", field);
                                return false;
                            }
                            field[r + rr][c + cc] = 1;
                        }
                    }
                } else {
                    // println!(">{:?}", field);
                    return false;
                }
            }
        }
    }

    // println!(":{:?}", field);
    true
}

fn main() {
    input! {
        n: usize,
        (h, w): (usize, usize),
        ab: [(usize, usize); n],
    };

    let mut i = 0;
    for perms in (0..n).permutations(n).collect::<Vec<Vec<usize>>>() {
        for bit in 0..(1 << n) {
            i += 1;
            let sides = format!("{:b}", bit)
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();

            // println!("{:?} {:?}", perms, sides);

            let l = perms
                .iter()
                .zip(sides.iter().chain(std::iter::repeat(&0)))
                .map(|(x, y)| (*x, *y))
                .collect::<Vec<(usize, usize)>>();

            if check((h, w), ab.clone(), l) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");

    // let mut tile_order = Vec::new();
    // for i in 0..n {
    //     tile_order.push((i, 0));
    // }

    // if check((h, w), ab, tile_order) {
    //     println!("Yes");
    // } else {
    //     println!("No");
    // }

    // let mut field = vec![vec![0; w + 10]; h + 10];
    // println!("{:?}", field);

    // let mut tiles = ab.clone();
    // for r in 0..h {
    //     for c in 0..w {
    //         if field[r][c] == 0 {
    //             let tile = tiles.pop().unwrap();
    //             for i in 0..tile.0 {
    //                 for j in 0..tile.1 {
    //                     field[r + i][c + j] = 1;
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("{:?}", field);
    // println!("{:?}", tiles);
}
