// Day 1 easy 
// 🟢 Problem: Find the Maximum Number in an Array 

// Input: [3, 7, 2, 9, 4]
// Output: 9

fn find_max(nums: &Vec<i32>) -> i32 {
    let mut max = nums[0];

    for &num in nums {
        if num > max {
            max = num;
        }
    }

    max
}



pub fn find_masx_num () {

    let numbers = vec![3, 7, 2, 9, 4];
    let result = find_max(&numbers);

    println!("Maximum number is: {}", result);



    let number = [3, 7, 2, 9, 4];

    println!("hello guys {:?}", number);


}