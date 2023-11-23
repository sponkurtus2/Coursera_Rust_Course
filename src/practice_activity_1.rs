pub fn practice_activity_1() {
    let my_array = vec![1, 2, 3, 7, 20];
    let result_int = find_max(&my_array);
    println!("Largest value: {}", result_int);

    let my_char = vec!['a', 'b', 'z', 'c'];
    let result_char = &my_char;
    println!("Largest char: {}", largest_char(result_char));

    let empty_vector: Vec<i32> = Vec::new();

    // The unwrap method is to eliminate the Some text
    println!("Largest Int: {:?}", find_largest(&my_array).unwrap());
    println!("Largest Char: {:?}", find_largest(&my_char).unwrap());
    print!("{:?}", &empty_vector);
}

// Find largest i32
fn find_max(list: &[i32]) -> &i32 {
    let mut max_value = &list[0];
        for value in list {
            if value > max_value {
                max_value = value;
            }
        }
    max_value
}

// Find largest char

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Find largest both data

fn find_largest<T: PartialOrd>(list: &[T]) -> Option<&T> {

    if list.is_empty() {
        return None;
    }

    let mut max = &list[0];
    for item in list.iter() {
        if item > max {
            max = item;
        }
    }
    Some(max)

}