use markdown::tokenize;

fn main() {
    let input = r"Hello, **world**!
        ```rs
        fn main() {
            println!(\Hello, world!\);
        }
        ```

        $\frac{2}{3}$
        ";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}
