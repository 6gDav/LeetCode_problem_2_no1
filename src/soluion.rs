pub struct Solution;

impl Solution 
{
    pub fn add_two_numbers(mut list1: Vec<i32>, mut list2: Vec<i32>)
    {
        list1.reverse();
        list2.reverse();

        let total1: i32 = list1.iter().map(|i| i.to_string()).collect::<String>().trim().parse().expect("Error");
        let total2: i32 = list2.iter().map(|i| i.to_string()).collect::<String>().trim().parse().expect("Error");
        
        let new_resoult_list: Vec<char> = (total1 + total2).to_string().chars().rev().map(|s| s).collect();
        println!("The reoult: {:?}", new_resoult_list);
    }
}