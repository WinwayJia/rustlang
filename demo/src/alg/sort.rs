pub fn swap(arr: &mut [i32], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp
}

pub fn partion(arr: &mut [i32]) -> usize {
    let mut j: usize = 1;
    for n in 1..arr.len() {
        if arr[n] < arr[0] {
            swap(arr, n, j);
            j += 1;
        }
    }

    swap(arr, 0, j - 1);

    j - 1
}

pub fn quick_sort(arr: &mut [i32], times: &mut i32) {
    if *times >= 4 {
        return;
    }
    *times += 1;

    if arr.len() <= 1 {
        return;
    }

    let pos = partion(arr);
    println!("pos={}", pos);

    if pos > 1 {
        quick_sort(&mut arr[..pos], times);
    }
    quick_sort(&mut arr[pos..], times);
}

#[test]
fn test_quick_sort() {
    let mut arr: [i32; 10] = [8, 3, 2, 5, 6, 1, 9, 7, 0, 4];
    let mut times = 0;
    quick_sort(&mut arr[0..10], &mut times);

    for value in arr.iter() {
        print!("{} ", value);
    }
    println!("");
}
