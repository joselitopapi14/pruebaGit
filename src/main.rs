use std::io;

fn primera_vez(){
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Ingrese el primer número");
    io::stdin()
        .read_line(&mut a)
        .expect("Locochon");
    let a: u32 = a.trim().parse().expect("ingrese un numero correcto");

    println!("Ingrese el segundo número");
    io::stdin()
        .read_line(&mut b)
        .expect("Locochon");
    let b: u32 = b.trim().parse().expect("ingrese un numero correcto");

    println!("Ingrese el tercer número");
    io::stdin()
        .read_line(&mut c)
        .expect("Locochon");
    let c: u32 = c.trim().parse().expect("ingrese un numero correcto");

    println!("{}",a*b*c);
}

fn main() {
    primera_vez();
}
