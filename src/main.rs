use std::io;

fn main() {
    println!("Enter in some random numbers");
    
    let mut vec: Vec<i32> = Vec::new();
    
    // Keep grabbing inputs from the terminal
    loop {
        let mut num = String::new();
        
        io::stdin().read_line(&mut num)
            .expect("DOH");
            
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => break, // This gets triggered if you don't put a num
        };
        
        vec.push(num);
    }
    
    // Actually start the sort
    vec = merge_sort(vec);
    
    for sorted_num in vec {
        println!("{}", sorted_num);
    }
}

// Helper function to get the minimum size
fn min(x: usize, y: usize) -> usize {
    if x < y {
        return x;
    }
    
    return y;
}

fn merge_sort(mut vec: Vec<i32>) -> Vec<i32>{
    let elements = vec.len();
    
    // We grow this size by doubling it over each iteration
    let mut current_size: usize = 1;
    
    // Iterate over each size of elements
    while current_size <= (elements - 1) {
        // Reset the start on each iteration
        let mut left_start: usize = 0;
        
        while left_start < (elements - 1) {
            // Get the middle of the size of elements in our slice
            let mid = left_start + current_size - 1;
            
            // Get the right edge of our current slice
            let right_end = min(left_start + (2 * current_size) - 1, elements - 1);
            
            // Do the merge
            vec = merge(vec, left_start, mid, right_end);
            
            left_start += 2 * current_size;
        }
        
        current_size *= 2;
    }
    
    return vec;
}

fn merge(mut vec: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    // We make copies of the left and right hand side of the vector slice we are after
    let left_parts: Vec<i32> = vec[left..mid + 1].iter().map(|&x| x).collect();
    let right_parts: Vec<i32> = vec[(mid+1)..(right+1)].iter().map(|&x| x).collect();
    
    // Cursors to iterate over the elements
    let mut left_cursor = 0;
    let mut right_cursor = 0;
    let mut insert_cursor = left;
    
    // Do the actual comparison sort on the provided slice
    while left_cursor < (mid - left + 1) && right_cursor < (right - mid) {
        if left_parts[left_cursor] <= right_parts[right_cursor] {
            vec[insert_cursor] = left_parts[left_cursor];
            left_cursor += 1;
        } else {
            vec[insert_cursor] = right_parts[right_cursor];
            right_cursor += 1;
        }
        
        insert_cursor += 1;
    }
    
    // Place the remaining left-side elements
    while left_cursor < (mid - left + 1) {
        vec[insert_cursor] = left_parts[left_cursor];
        
        left_cursor += 1;
        insert_cursor += 1;
    }
    
    // Place the remaining right-side elements
    while right_cursor < (right - mid) {
        vec[insert_cursor] = right_parts[right_cursor];
        
        right_cursor += 1;
        insert_cursor += 1;
    }
    
    // We return the sub-sorted vector
    return vec;
}
