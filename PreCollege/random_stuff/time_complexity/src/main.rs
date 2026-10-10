fn main() {
    let x = 2;

    let mut total = 0;

    // x^2 or O(n^2)
    for i in x {
        for j in x {
            total += i + j;
        }
    }

    // 3x+2 or O(n)
    for k in 3 * x + 2 {
        println!("in loop")
    }

    println!("{}", total);

    // O(1)
    let n = 10;
    println!("{}", (n + n) / 3 + 20 * n);

    // O(n)
    // Sorting takes O(n log n)
    // Math takes O(1)
    let array = vec![3, 1, 5, 7, 8, 9]; // adding a number adds O(1)
    let search_para = 5;
    for n in array {
        if n == search_para {
            break;
        }
    }
}
