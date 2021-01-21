use rand::{thread_rng, Rng, rngs::ThreadRng};

fn main() {
    /*
    https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/rust
    
    Task:
    You will be given an array of numbers. You have to sort the odd numbers in ascending order while leaving the even numbers at their original positions.
    
    Examples:
    [7, 1]  =>  [1, 7]
    [5, 8, 6, 3, 4]  =>  [3, 8, 6, 5, 4]
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]  =>  [1, 8, 3, 6, 5, 4, 7, 2, 9, 0]
    */
    
    code_wars_tests();
}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut temp: Vec<i32> = arr.clone().to_vec();
    
    for (i, x) in arr.clone().iter().enumerate() {
        if x % 2 == 0 {
            continue;
        }
        
        for j in (i + 1)..arr.clone().iter().len() {
            let y = temp[j];
            let z = temp[i];
                
            if y % 2 == 0 || z < y {
               continue;
            }
            
            temp[i] = y;
            temp[j] = z;
        }
    }
    
    temp
}

fn code_wars_tests() {
    assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
    assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
    assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4, 11]), [1, 3, 2, 8, 5, 4, 11]);
    assert_eq!(sort_array(&[2, 22, 37, 11, 4, 1, 5, 0]), [2, 22, 1, 5, 4, 11, 37, 0]);
    assert_eq!(sort_array(&[1, 111, 11, 11, 2, 1, 5, 0]), [1, 1, 5, 11, 2, 11, 111, 0]);
    assert_eq!(sort_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), [1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    assert_eq!(sort_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(sort_array(&[0, 1, 2, 3, 4, 9, 8, 7, 6, 5]), [0, 1, 2, 3, 4, 5, 8, 7, 6, 9]);
    assert_eq!(sort_array(&[]), []);
    
    let mut rng = thread_rng();
    let gen_arr = |len: usize, rng: &mut ThreadRng| {
        (0..rng.gen_range(0..len)).map(|_| rng.gen_range(0..101)).collect::<Vec<_>>()
    };
        
    // Test slices of length 0–10
    for _ in 0..14 {
        let arr = gen_arr(10, &mut rng);
        assert_eq!(sort_array(&arr[..]), sort_array_solution(&arr[..]));
    }
        
    // Test slices of length 0–20
    for _ in 0..14 {
        let arr = gen_arr(20, &mut rng);
        assert_eq!(sort_array(&arr[..]), sort_array_solution(&arr[..]));
    }
        
    // Test slices of length 0–30
    for _ in 0..14 {
    let arr = gen_arr(30, &mut rng);
        assert_eq!(sort_array(&arr[..]), sort_array_solution(&arr[..]));
    }        
}
    
fn sort_array_solution(arr: &[i32]) -> Vec<i32> {
    let mut odd = arr
        .iter()
        .filter(|&x| x & 1 == 1)
        .copied()
        .collect::<Vec<_>>();
        odd.sort_by(|a, b| b.cmp(&a));
    
    arr.iter()
        .map(|&x| if x & 1 == 1 { odd.pop().unwrap() } else { x })
        .collect()
}
