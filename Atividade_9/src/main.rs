fn main() {
    #[derive(Debug, PartialEq)]

    struct ParOrdenado<T>{
        x: T,
        y: T,
    }

    impl<T> ParOrdenado<T>{
        fn new(x: T, y: T) -> Self {
            Self{
                x,
                y
            }

        }
    }






}
