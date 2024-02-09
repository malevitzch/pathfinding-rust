use rand::Rng;
use std::collections::VecDeque;
use inline_colorization::*;
use std::thread;
use array2d::Array2D;

//function that takes a float value from 0 to 100
//returns an Rgb color from red to green based on the value
//TODO: change to option, handle values outside the [0.0, 100.0] range
fn percent_to_rgb(percent: f32) -> image::Rgb<u8> {
    let k: u8 = ((255.0 * percent) / 100.0).round() as u8; //TODO: check if k belongs to [0, 255] 
    image::Rgb([255 - k, k, 0u8])
}
//function that takes an immutable reference to a vector of tuples of element and weight
//randomly returns a copy of one of the elements, with chances based on the weights
//in case all elements have weight 0, returns the default element of type T
//TODO: perhaps change the syntax to return and option and 
fn weighted_rand<T: std::marker::Copy + Default>(elements: &Vec<(T, u32)>) -> T {
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
    Default::default()
}
//auxilary function that converts an integer tuple to usize tuple
fn utuple(val: (i32, i32)) -> (usize, usize) {
    (val.0 as usize, val.1 as usize)
}
//function that checks whether the coords are inside a square of set size (indexed from 0)
fn in_bounds(coords: (i32, i32), bounds: i32) -> bool {
    coords.0 >= 0 && coords.1 >= 0 && coords.0 < bounds && coords.1 < bounds
}
//function that generates a random map and returns whether there exists a path from top left corner
//to bottom right corner
//map size and the vector of types and weights is configurable, albeit the types are not to be
//changed as 0 and 1 are hard-coded as passable and non-passable so calling the run function with a
//types vec that has other values will result in undefined behavior
//if display is set to true, the map is printed to console with red X as unreachable tiles,
//reachable tiles as blue O, and tiles that belong to the optimal path as green O
//TODO: allow for m x n maps rather than just m x m
fn run(size: usize, display: bool, types: &Vec<(i32, u32)>) -> bool {
    let n: usize = size;
    let mut dist: Array2D<i32> = Array2D::filled_with(-1, n, n);
    let mut map: Array2D<i32> = Array2D::filled_with(0, n, n);  
    for i in 0..n {
        for j in 0..n {
            map[(i, j)] = weighted_rand(&types);
        }
    }
    map[(n - 1, n - 1)] = 0;
    map[(0, 0)] = 0;
    dist[(0, 0)] = 0;
    let mut deq: VecDeque<(i32, i32, i32)> = VecDeque::new();
    deq.push_back((0, 0, 0));
    while !deq.is_empty() {
        let cur = deq.front().unwrap().clone();
        deq.pop_front();
        for i in [(cur.0 - 1, cur.1), (cur.0 + 1, cur.1), (cur.0, cur.1 - 1), (cur.0, cur.1 + 1)] {
            if in_bounds(i, n as i32) && dist[utuple(i)] == -1 && map[utuple(i)] != 1 {
                dist[utuple(i)] =  dist[utuple((cur.0, cur.1))] + 1;
                deq.push_back((i.0, i.1, dist[utuple((cur.0, cur.1))] + 1));
            }
        }
    }
    let mut cur: (i32, i32) = ((n - 1) as i32, (n - 1) as i32);
    let mut better_spot: bool = true;
    if dist[utuple(cur)] != -1
    {
        while better_spot {
            map[utuple(cur)] = 2; 
            better_spot = false;
            for nxt in [(cur.0 - 1, cur.1), (cur.0 + 1, cur.1), (cur.0, cur.1 - 1), (cur.0, cur.1 + 1)] {
                if in_bounds(nxt, n as i32) && dist[utuple(nxt)] != -1 && dist[utuple(nxt)] < dist[utuple(cur)] {
                    cur = nxt;
                    better_spot = true;
                    break;
                }
            }
        }
    }
    if !display {
        return dist[(n-1, n-1)] != -1;
    }
    for i in 0..n {
        for j in 0..n {
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
    return dist[(n-1, n-1)] != -1;
}
#[allow(dead_code)]
fn coords_to_index(x: usize, y: usize, width: usize) -> usize
{
    x + width * y
}
fn index_to_coords(index: usize, width: usize) -> (usize, usize)
{
    ((index % width), index / width)
}
fn get_color(size: usize, display: bool, types: Vec<(i32, u32)>, iterations: u32) -> image::Rgb<u8> {
    let mut successful : i32 = 0;
    for _ in 0..iterations
    {
        successful += run(size, display, &types) as i32;
    }
    let percent: f32 = 100.0 * successful as f32 / iterations as f32;
    return percent_to_rgb(percent)
}
fn main() {
    let now = std::time::Instant::now();
    let mut types: Vec<(i32, u32)> = [(0, 0), (1, 0)].to_vec();
    let precision: u32 = 10000;
    let mut img: image::RgbImage = image::RgbImage::new(20*50 as u32, 10*50 as u32);
    let mut threads : Vec<thread::JoinHandle<image::Rgb<u8>>> = Vec::new();
    for weight in 0..10usize {
        for map_size in 1..=20usize {
            types[0].1 = 10 - weight as u32;
            types[1].1 = weight as u32;
            let type_clone = types.clone();
            threads.push(thread::spawn(move || get_color(map_size.clone(), false, type_clone, precision.clone()))); 
        }
    }
    let mut i: usize = 0;
    for cur in threads.into_iter() {
        let color: image::Rgb<u8> = (cur.join().unwrap()).clone();
        let coords = index_to_coords(i, 20);
        for x in 50*coords.0..50*(coords.0+1) {
            for y in (50*coords.1)..50*(coords.1+1) {
                img.put_pixel(x as u32, y as u32, color.clone());
            }
        }
        i+=1;        
    }
    let elapsed = now.elapsed();
    img.save("Output_Graph.png").expect("Failed to save image");
    print!("{color_white}{}s", elapsed.as_secs());
}

