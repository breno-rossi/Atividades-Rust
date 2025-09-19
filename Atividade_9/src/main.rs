#[derive(Debug, PartialEq)]

pub struct ParOrdenado<T> {
    x: T,
    y: T,
}

impl<T> ParOrdenado<T> {
    pub fn new(x: T, y: T) -> Self {
        Self{ x, y }

    }

    pub fn x(&self) -> &T{
        &self.x
    }

    pub fn y(&self) -> &T{
        &self.y
    }

}

fn main() {


    let par_int = ParOrdenado::new(10,20);
    println!("Par de inteiro: {}, {}", par_int.x(), par_int.y());
    println!("Instância completa:{:?}",par_int);

    assert_eq!(*par_int.x(), 10);
    assert_eq!(*par_int.y(), 20);

    println!();

    let par_str = ParOrdenado::new("vinte e um","vinte e dois");
    println!("Par de str: {}, {}", par_str.x(), par_str.y());
    println!("Instância completa:{:?}",par_str);


    assert_eq!(*par_str.x(), "vinte e um");
    assert_eq!(*par_str.y(), "vinte e dois");


    println!();

    let par_f64 = ParOrdenado::new(444.4,8888.8);
    println!("Par de f: {}, {}", par_f64.x(), par_f64.y());
    println!("Instância completa:{:?}",par_f64);


    assert_eq!(*par_f64.x(), 444.4);
    assert_eq!(*par_f64.y(),8888.8);


    // let par_erro_misto = ParOrdenado::new(5,"vai_dar_erro");
    //
    //   |     let par_erro_misto = ParOrdenado::new(5,"vai_dar_erro");
    // |                          ----------------   ^^^^^^^^^^^^^^ expected integer, found `&str`
    // |


}
