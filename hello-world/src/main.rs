use core::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    println!("{} days", 31);
    println!("As a {0}, we can print {0} twice", "Test");
    println!("Binary number: {:b}", 9);
    println!(
        "Let's print a number: {content:>0min_width$}",
        content = 42,
        min_width = 3
    );
    println!(
        "..or indent text: {content:>min_width$}",
        content = "Test",
        min_width = 10
    );

    #[derive(Debug)]
    struct Structure(i32, i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    println!("Let's print a struct: {}", Structure(5, 12));

    #[derive(Debug)]
    struct Deep(Structure);

    println!("Here's a deep struct: {:?}", Deep(Structure(1, 2)));

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    let num = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display: {}", num);
    println!("Debug: {:?}", num);
}
