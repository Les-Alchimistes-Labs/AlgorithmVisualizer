pub fn counting_sort(array: &mut Vec<i64>, max: usize)
{
    let mut occ: Vec<usize> = vec![0; max + 1];
    for &i in array.iter()
    {
        occ[i as usize] += 1;
    }
    let mut index = 0;
    for (i,&j) in occ.iter().enumerate()
    {
        for _k in 0..j
        {
            array[index] = i as i64;
            index +=1;
        }
    }
}

//pub fn main()
//{
//    let mut arr1: Vec<i64> = vec![4, 3, 12, 1, 5, 5, 3, 9];
//    println!("arr1 = {:?}",arr1);
//    counting_sort(&mut arr1,12);
//    println!("Application counting_sort: arr1 = {:?}",arr1);

//    let mut arr2: Vec<i64> = vec![14, 5, 13, 13, 0, 512, 42, 6, 0, 1, 12, 1000, 80, 17, 7];
//    println!("arr1 = {:?}",arr2);
//    counting_sort(&mut arr2,1000);
//    println!("Application counting_sort: arr1 = {:?}",arr2);
//}
