fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    // let a = [
    //     1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    //     1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 21, 21, 2, 1, 2, 1, 2, 1, 21, 21, 2, 1, 21, 2, 1,
    //     22, 1, 2, 1, 21, 1, 2, 12, 1, 2, 12, 1, 2, 1, 2, 12, 1, 21, 2, 1, 2, 12, 1, 2, 12, 21, 2,
    //     12, 21, 21, 21, 1, 21, 2, 12, 21, 2, 2, 12, 21, 2, 21, 2, 12, 1, 2, 1, 2, 12, 12, 12, 2,
    //     12, 21, 121,
    // ]; NOTE: bara testa
    // let a = ["a"; 100];
    let a = 1..111;

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
