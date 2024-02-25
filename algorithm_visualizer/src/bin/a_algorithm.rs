#[derive(PartialEq)]
#[derive(Debug, Clone)]
pub struct Graph {
    key: i32,
    adjlists: Vec<Vec<i32>>,
    order: i32,
}

impl Graph {
    fn new(key: i32, adjlists: Vec<Vec<i32>>, order: i32) -> Self {
        Graph { key, adjlists, order }
    }
}

fn heuristic_estimate_of_distance(start: i32, end: i32) -> i32 {
    (end - start).abs()
}

fn reconstruct_path(came_from: &Vec<i32>, current: i32) -> Vec<i32> {
    if came_from[current as usize] != -1 {
        let mut path = reconstruct_path(&came_from, came_from[current as usize]);
        path.push(current);
        return path;
    }
    vec![current]
}


pub fn a_algorithm(g: Graph, start: i32, end: i32) -> Vec<i32> {
    let mut closed_vec: Vec<i32> = vec![];
    let mut open_vec: Vec<i32> = vec![start];
    let mut came_from: Vec<i32> = vec![-1; g.order as usize];
    let mut g_score: Vec<i32> = vec![i32::MAX; g.order as usize];
    let mut h_score: Vec<i32> = vec![i32::MAX; g.order as usize];
    let mut f_score: Vec<i32> = vec![i32::MAX; g.order as usize];
    g_score[start as usize] = 0;
    h_score[start as usize] = heuristic_estimate_of_distance(start, end);
    f_score[start as usize] = h_score[start as usize];
    while !open_vec.is_empty() {
        let current = *open_vec.iter().min_by_key(|&x| f_score[*x as usize]).unwrap();
        if current == end {
            return reconstruct_path(&came_from, current);
        }
        open_vec.retain(|&x| x != current);

        closed_vec.push(current);

        for y in 0..g.order {

            if g.adjlists[current as usize][y as usize] == 0{
                continue;
            }
            let tentative_g_score = g_score[current as usize] + g.adjlists[current as usize][y as usize];

            let tentative_is_better: bool;
            if !open_vec.contains(&y){
                tentative_is_better = true;
            } else if tentative_g_score < g_score[y as usize] {
                tentative_is_better = true;
            } else {
                tentative_is_better = false;
            }

            if tentative_is_better{
                came_from[y as usize] = current;
                g_score[y as usize] = tentative_g_score;
                h_score[y as usize] = heuristic_estimate_of_distance(y, end);
                f_score[y as usize] = g_score[y as usize] + h_score[y as usize];
                if !open_vec.contains(&y){
                    open_vec.push(y);
                }
            }
        }
    }

    vec![]
}



fn main() {
    let _g: Graph = Graph::new(0, vec![vec![0, 1, 1, 0, 0],
                                       vec![1, 0, 1, 1, 0],
                                       vec![0, 0, 0, 1, 1],
                                       vec![0, 1, 0, 0, 1],
                                       vec![0, 0, 0, 1, 0]], 5);
    let start = 0;
    let end = 4;
    let result = a_algorithm(_g, start, end);
    println!("The shortest path from {} to {} is : {:?}", start, end, result);
}


