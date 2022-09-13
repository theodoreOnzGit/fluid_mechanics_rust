use fluid_mechanics_rust;
fn main() {
    println!("Hello, world!");
    hello2();
    test_friction_factor();
}

fn hello2(){
    println!("hello world!2");
}

fn test_friction_factor(){
    let darcy_friction_factor = 
        fluid_mechanics_rust::darcy(1800.0,0.0015);

    println!("{}", darcy_friction_factor);
    
    let fldk = 
        fluid_mechanics_rust::fldk(
            15000.0,0.00014,10.0,5.0);

    println!("{}", fldk);

    let bejan_d = 
        fluid_mechanics_rust::get_bejan_d(
            15000.0,0.00014,10.0,5.0);

    println!("{}", bejan_d);

}




