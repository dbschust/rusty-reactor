fn main() {
    let x = Nuclide::new(4.5, (1.0,1.0), 3.0, 1);
    println!("x: {:?}", x);
}

#[derive(Debug, Clone)]
struct Atom {
    //electrons: Vec<Electron>,
    protons: Vec<Nuclide>,
    neutrons: Vec<Nuclide>,

}

impl Atom {
    fn new(self, p: u8, n: u8) {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Nuclide {
    mass: f32,
    dir: (f32, f32),
    vel: f32,
    charge: i8,

}
impl Nuclide {
    fn new(mass: f32, dir: (f32, f32), vel: f32, charge: i8) -> Nuclide {
        Nuclide{mass, dir, vel, charge}
    }
}


/*
#[derive(Debug, Clone)]
struct Electron {
    dir: [f32; 2],
    velocity: f32,
}
*/
