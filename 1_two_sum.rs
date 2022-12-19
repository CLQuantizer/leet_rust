use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();
        for i in 0..nums.len(){
            let j = target - nums[i];
            if map.contains_key(&j){
                return vec![i as i32, *map.get(&j).unwrap() as i32];
            }
            map.insert(nums[i],i as i32);
            }
        vec![]
    }
}