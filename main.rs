fn main () {
    println!("Hello, World!");
    pruebavariable();
    pruebalen();
    let ent = compound();
    println!("Prueba reacción {}", ent);
}

fn pruebavariable() {
    let mut numero: i32 = -1;
    println!("el valor de x es: {}", numero);
    numero = 2;
    println!("Esta línea no se muestra");
    println!("variable nueva es: {}", numero);
}

fn pruebalen() {
    let random = "adia90dvhvsv";
    let caracteres = random.len();
    println!{"{}", caracteres}
}

fn compound() -> i32 {
    let mut ent: i32 = 0;

    while ent<10 {
        ent = ent + 1;
    }

    loop {
        ent = ent - 5;
    break;
    }
    return ent;
}