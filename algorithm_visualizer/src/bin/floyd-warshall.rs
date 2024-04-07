#[derive(PartialEq)]
#[derive(Debug, Clone)]
pub struct Graph {
    key: i32,
    adjlists: Vec<Vec<i32>>,
    costs: Vec<Vec<i32>>,
    order: i32,
}

pub fn floyd_warshall(g: Graph) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let order: usize = g.order as usize;
    let mut dist: Vec<Vec<i64>> = vec![vec![i64::MAX; order]; order];
    let mut pred = vec![vec![0; order]; order];

    for x in 0..order {
        for y in 0..order {
            pred[x][y] = x as i64;
            if g.costs.len() > 0 {
                dist[x][y] = g.costs[x][y] as i64;
            } else {
                dist[x][y] = if g.adjlists[x][y] == 1 { 1 } else { i64::MAX };
            }
        }
    }

    for k in 0..order {
        for i in 0..order {
            for j in 0..order {
                if let Some(sum) = dist[i][k].checked_add(dist[k][j]) {
                    if sum < dist[i][j] {
                        dist[i][j] = sum;
                        pred[i][j] = pred[k][j];
                    }
                }
            }
        }
    }

    (dist, pred)
}



fn display(adjlists: Vec<Vec<i64>>) {
    for i in 0..adjlists.len() {
        for j in 0..adjlists[i].len() {
            print!("{} ", adjlists[i][j]);
        }
        println!();
    }
}

fn main() {
    let adjlists = vec![vec![0, 1, 1, 0, 0],
                        vec![1, 0, 1, 1, 0],
                        vec![0, 0, 0, 1, 1],
                        vec![0, 1, 0, 0, 1],
                        vec![0, 0, 0, 1, 0]];
    let costs = vec![vec![0, 10, 5, 0, 0],
                     vec![10, 0, 2, 3, 0],
                     vec![5, 2, 0, 8, 4],
                     vec![0, 3, 8, 0, 7],
                     vec![0, 0, 4, 7, 0]];


    let g: Graph = Graph { key: 0, adjlists, costs, order: 5 };

    let (dist, pred) = floyd_warshall(g);

    println!("Matrix de distances :");
    display(dist);
    println!("Matrix de prédécesseurs :");
    display(pred);

    let adjlists2 = vec![vec![0, 1, 1, 0, 0],
                         vec![1, 0, 1, 1, 0],
                         vec![0, 0, 0, 1, 1],
                         vec![0, 1, 0, 0, 1],
                         vec![0, 0, 0, 1, 0]];
    let pred2 = vec![vec![0, 2, 2, 0, 0],
                     vec![0, 0, 2, 2, 0],
                     vec![0, 0, 0, 2, 2],
                     vec![0, -3, 2, 0, 2],
                     vec![0, 0, 0, 3, 0]];
    let g2: Graph = Graph { key: 0, adjlists: adjlists2, costs: vec![], order: 5 };

    let (dist, pred) = floyd_warshall(g2);

    println!("Matrix de distances 2 :");
    display(dist);
    println!("Matrix de prédécesseurs 2 :");
    display(pred);
}
