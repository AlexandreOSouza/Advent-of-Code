use std::fs;

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] >= arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn find_max(value: &String) {
    let elfs: Vec<Vec<u32>> = value
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut max: (u32, usize) = (0, 0);
    let mut sum_vec: Vec<u32> = Vec::new();
    for (idx, elf) in elfs.iter().enumerate() {
        let sum: u32 = elf.iter().sum();
        sum_vec.push(sum);
        if max.0 < sum {
            max.0 = sum;
            max.1 = idx;
        }
    }

    bubble_sort(&mut sum_vec);

    let last_sum =
        sum_vec[sum_vec.len() - 1] + sum_vec[sum_vec.len() - 2] + sum_vec[sum_vec.len() - 3];
    println!("{:?}", { max });
    println!("{}", last_sum);
}

fn main() {
    let filepath: &str = "../input.txt";
    let content = fs::read_to_string(filepath).expect("Error trying to read the file");
    find_max(&content);
}
