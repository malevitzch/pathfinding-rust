use rand::Rng;
use std::collections::VecDeque;
use inline_colorization::*;
use array2d::Array2D;
fn weighted_rand<T: std::marker::Copy>(elements: &Vec<(T, u32)>) -> T {
    let mut cur_sum: u32 = 0;
    let mut total_sum: u32 = 0;
    for i in 0..elements.len() {
        total_sum += elements[i].1;
    }
    let rng_val = rand::thread_rng().gen_range(1..=total_sum);
    for i in 0..elements.len() {
        cur_sum += elements[i].1;
        if cur_sum >= rng_val {
            return elements[i].0.clone();
        }
    }
    elements[elements.len() - 1].0.clone()
}
fn utuple(val: (i32, i32)) -> (usize, usize) {
    (val.0 as usize, val.1 as usize)
}
fn in_bounds(coords: (i32, i32), bounds: i32) -> bool {
    coords.0 >= 0 && coords.1 >= 0 && coords.0 < bounds && coords.1 < bounds
}
fn run(size: usize, display: bool) -> bool {
    let N: usize = size;
    let types: Vec<(i32, u32)> = [(0, 10), (1, 5)].to_vec();
    let mut dist: Array2D<i32> = Array2D::filled_with(-1, N, N);
    let mut map: Array2D<i32> = Array2D::filled_with(0, N, N);  
    for i in 0..N {
        for j in 0..N {
            map[(i, j)] = weighted_rand(&types);
        }
    }
    map[(N - 1, N - 1)] = 0;
    map[(0, 0)] = 0;
    dist[(0, 0)] = 0;
    let mut deq: VecDeque<(i32, i32, i32)> = VecDeque::new();
    deq.push_back((0, 0, 0));
    while !deq.is_empty() {
        let cur = deq.front().unwrap().clone();
        deq.pop_front();
        for i in [(cur.0 - 1, cur.1), (cur.0 + 1, cur.1), (cur.0, cur.1 - 1), (cur.0, cur.1 + 1)] {
            if in_bounds(i, N as i32) && dist[utuple(i)] == -1 && map[utuple(i)] != 1 {
                dist[utuple(i)] =  dist[utuple((cur.0, cur.1))] + 1;
                deq.push_back((i.0, i.1, dist[utuple((cur.0, cur.1))] + 1));
            }
        }
    }
    let mut cur: (i32, i32) = ((N - 1) as i32, (N - 1) as i32);
    let mut better_spot: bool = true;
    if dist[utuple(cur)] != -1
    {
        while better_spot {
            map[utuple(cur)] = 2; 
            better_spot = false;
            for nxt in [(cur.0 - 1, cur.1), (cur.0 + 1, cur.1), (cur.0, cur.1 - 1), (cur.0, cur.1 + 1)] {
                if in_bounds(nxt, N as i32) && dist[utuple(nxt)] != -1 && dist[utuple(nxt)] < dist[utuple(cur)] {
                    cur = nxt;
                    better_spot = true;
                    break;
                }
            }
        }
    }
    if !display {
        return dist[(N-1, N-1)] != -1;
    }
    for i in 0..N {
        for j in 0..N {
            print!("{} ",
                match (map[(i, j)], dist[(i, j)]) {
                    (1, _) | (0, -1) => {
                        print!("{color_red}");
                        "X".to_string()
                        },
                    (0, _) => {
                        print!("{color_blue}");
                        "O".to_string()
                        },
                    (2, _) => {
                        print!("{color_green}");
                        "O".to_string()
                        }
                    (_, _) => "ERR".to_string()
                    }
                ); 
        }
        print!("\n");
    }
    return dist[(N-1, N-1)] != -1;
}

fn main() {
    let mut valid : i32 = 0;
    let total : i32 = 10000;
    for _ in 0..total {
        valid += run(20, false) as i32;
        }
    println!("{:.2}", (100.0 * valid as f32 / total as f32));
}
