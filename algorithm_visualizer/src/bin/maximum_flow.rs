struct Graph{
    s: Vec<i64>, //sommet
    l: Vec<Vec<(usize,i64)>>,  //lien
}


fn dfs (g :&mut Graph,g2 :&mut Graph, p : usize, add : i64) -> (i8,i64)
{
    let mut pv = 0;
    let mut gp = 0;
    let mut max = 0;

    while pv < g.l[p].len()
    {
        if add < 0 && g2.l[p][pv].1 == 0
        {
            g2.l[p][pv].1 = g.l[p][pv].1;
            return (1,1)//to brake
        }
        else if g2.l[p][pv].1 + add <= g.l[p][pv].1
        {
                g2.l[p][pv].1 += add;
                return (1,add);//to brake
        }
        else if g.l[p][pv].1 - g2.l[p][pv].1 > max
        {
                gp = pv;
                max = g.l[p][pv].1 - g2.l[p][pv].1;
        }
        pv += 1;
    }
    let res = dfs( g, g2,pv,g.l[p][pv].1 - g2.l[p][pv].1);
        if res.0 == 1
        {
            g2.l[p][pv].1 +=res.1;
            return (1,res.1);
        }

    (0,0)
}


fn maximum_flow(g : &mut Graph,g2 :&mut Graph)
{
    dfs( g, g2,0,-1);

}

fn main()
{
}
