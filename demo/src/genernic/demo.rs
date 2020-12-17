

pub fn largest(list: &[i32]) -> &i32 {
    let mut max = &list[0];

    for i in list {
        if i > max {
            max = i
        }
    }

    max
}