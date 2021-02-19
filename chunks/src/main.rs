/* You have to write a function that chunks an array into smaller arrays of specified size. For example, chunks([1,2,3,4,5] , 2) should return [[1,2],[3,4], [5]]. */

fn main() {
    for i in 0..5 {
        println!("{} => {:?}", i, chunks(vec![1, 2, 3, 4, 5], i));
    }
}

fn chunks<T>(arr: Vec<T>, size: usize) -> Vec<Vec<T>> where Vec<T>: Clone, T: Copy {
    if arr.len() <= 0 {
        return vec![];
    }

    if size <= 0 {
        return vec![arr];
    }

    let mut chunks: Vec<Vec<T>> = vec![];
    let mut chunk: Vec<T> = vec![];

    for i in 0..arr.len() {
        if i != 0 && i % size == 0 {
            chunks.push(chunk.clone());
            chunk.clear();
        }
        
        chunk.push(arr[i]);
    }

    if chunk.len() > 0 {
        chunks.push(chunk);
    }

    chunks
}
