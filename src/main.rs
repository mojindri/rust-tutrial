mod enums_and_patterns;
mod traits_and_generics;

fn main() {
    println!("Hello, world!");
    //binary_tree_challange();
    let num_disks = 4; // You can change the number of disks here
    hanoi(num_disks, 'A', 'B', 'C');
}

fn hanoi(n: u32, source: char, auxiliary: char, target: char) {
    if n == 1 {
        println!("Move disk 1 from peg {} to peg {}", source, target);
        return;
    }

    hanoi(n - 1, source, target, auxiliary);
    println!("Move disk {} from peg {} to peg {}", n, source, target);
    hanoi(n - 1, auxiliary, source, target);
}