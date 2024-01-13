struct Graph{
    s: Vec<i64>, //sommet
    l: Vec<Vec<(usize,i64)>>,  //lien
}


fn dfs (g :&mut Graph,g2 :&mut Graph, p : usize, add : i64) -> i8
{
    let mut pv = 0;
    while pv < g.l[p].len() 
    {
        if g.l[p][pv].1 > g2.l[p][pv].1
        {
            if add < 0 && g2.l[p][pv].1 == 0
            {
                g2.l[p][pv].1 = g.l[p][pv].1;
                return 1;//to brake
            }
            else if g2.l[p][pv].1 + add <= g.l[p][pv].1
            {
                g2.l[p][pv].1 += add;
                return 1;//to brake
            }
            else
            {
                if dfs(g,g2,pv,add) == 1
                {
                    return 1;
                }

            }
        }
        pv += 1;

    }
    0


}


fn maximum_flow(g : Graph, g2 : Graph)
{
    
}

fn main()
{
}
