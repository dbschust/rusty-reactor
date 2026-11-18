//! Rusty Reactor
//! Daniel Schuster

use std::io::Write;

fn main() {
    let nuclide = Nuclide::new(4.5, Dir::N, 3.0, Charge::POS);
    let mut area = Area::new(5, 5);
    area.add(nuclide);
    area.display();
}

/// A Nuclide represents a proton or neutron, with a mass, velocity, and direction.
/// A Nuclide with a POS charge represents a proton, ZERO charge a neutron.
#[derive(Debug, Clone)]
struct Nuclide {
    mass: f32,
    dir: Dir,
    vel: f32,
    charge: Charge,
}

impl Nuclide {
    fn new(mass: f32, dir: Dir, vel: f32, charge: Charge) -> Nuclide {
        Nuclide {
            mass,
            dir,
            vel,
            charge,
        }
    }
    fn display(&self) {
        match self.charge {
            Charge::POS => print!("p"),
            Charge::ZERO => print!("n"),
            Charge::NEG => print!("e"),
        }
    }
}

/// Area represents the region in which particles will interact
/// with each other.
#[derive(Debug, Clone)]
struct Area {
    height: usize,
    width: usize,
    board: Vec<Option<Nuclide>>,
}
impl Area {
    fn new(height: usize, width: usize) -> Area {
        let squares = vec![None; height * width];
        Area {
            height,
            width,
            board: squares,
        }
    }

    /// moves an existing nuclide into the board area
    fn add(&mut self, nuclide: Nuclide /* x: usize, y: usize*/) {
        self.board.push(Some(nuclide));
    }

    /// displays the current state of the board, with . for empty locations,
    /// p for a proton, n for a neutron
    fn display(&self) {
        for square in &self.board {
            match square {
                None => print!("."),
                Some(x) => x.display(),
            }
            std::io::stdout().flush().unwrap();
        }
        println!();
    }

    /// function to update the current status of the board one iteration
    fn update(&self) {
        todo!();
    }
}

/// Represents the "direction" a particle is traveling, with up on the
/// screen corresponding to North.
#[derive(Debug, Clone)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

/// Represents the charge of a single particle
#[derive(Debug, Clone)]
enum Charge {
    NEG = -1,
    ZERO = 0,
    POS = 1,
}

/*
#[derive(Debug, Clone)]
struct Electron {
    dir: Dir,
    velocity: f32,
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
*/

