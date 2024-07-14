fn part(tab: &mut Vec<usize>, little: i32, big: i32) -> i32
{
    let piv = tab[big as usize];
    let mut pos = little - 1;
    for i in little..big
    {
        if tab[i as usize] <= piv
        {
            pos += 1;
            (tab[pos as usize], tab[i as usize]) = (tab[i as usize], tab[pos as usize]);
        }
    }
    (tab[pos as usize + 1], tab[big as usize]) = (tab[big as usize], tab[pos as usize + 1]);
    return pos + 1;
}

pub fn quick_sort(tab: &mut Vec<usize>, little: i32, big: i32)
{
    if little < big
    {
        let pos = part(tab, little, big);
        quick_sort(tab, little, pos - 1);
        quick_sort(tab, pos + 1, big);
    }
}

fn main()
{
    let mut tab = vec![5, 1, 2, 3, 4, 6, 88, 65, 974, 620, 742, 7, 648];
    let leng = (tab.len()) as i32;
    println!("{:?}", tab);
    quick_sort(&mut tab, 0, leng - 1);
    println!("{:?}", tab);
}
