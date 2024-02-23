#[derive(PartialEq)]
#[derive(Debug, Clone)]
pub struct Graph {
    key: i32,
    adjlists: Vec<Vec<u8>>,
    order: i32,
}

pub fn floyd_warshall(g: Graph) -> Vec<Vec<u8>> {
    let mut adjlists = g.adjlists.clone();
    let n = g.order as usize;
    for k in 1..n {
        for i in 1..n {
            for j in 1..n {
                adjlists[i][j] = adjlists[i][j] | (adjlists[i][k] & adjlists[k][j])
            }
        }
    }
    adjlists
}

fn display(adjlists: Vec<Vec<u8>>) {
    for i in 0..adjlists.len() {
        print!("[");
        for j in 0..adjlists[i].len() {
            print!("{:?} ", adjlists[i][j]);
        }
        print!("]");
        println!();
    }
}

fn main() {
    let inf = i32::MAX;
    let adjlists = vec![vec![0, 0, 1, 0],
                        vec![0, 0, 1, 1],
                        vec![1, 1, 0, 0],
                        vec![0, 1, 0, 0]];
    let g: Graph = Graph { key: 0, adjlists, order: 4 };
    let result = floyd_warshall(g);
    display(result);
}