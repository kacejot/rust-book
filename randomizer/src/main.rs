use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

fn main() {
    let mut randomizer = Randomizer::new(StdRng::from_seed([2; 32]), Uniform::new(-1.0, 1.0));
    println!("{}", randomizer.gen());
    println!("{}", randomizer.gen());
    println!("{}", randomizer.gen());
    println!("{}", randomizer.gen());
    println!("{}", randomizer.gen());
}

// randomizer accepts seed to predefine the sequence of random numbers
// randomizer accepts distribution to sample random numbers
// randomizer has gen() method that generates new random number
// randomizer - instance that lives several calls

struct Randomizer<R, D, T>
where
    R: SeedableRng + Rng,
    D: Distribution<T>,
{
    rng: R,
    distribution: D,
    phantom: ::core::marker::PhantomData<T>,
}

impl<R, D, T> Randomizer<R, D, T>
where
    R: SeedableRng + Rng,
    D: Distribution<T>,
{
    pub fn new(rng: R, distribution: D) -> Self {
        Self {
            rng,
            distribution,
            phantom: ::core::marker::PhantomData,
        }
    }

    pub fn gen(&mut self) -> T {
        self.distribution.sample(&mut self.rng)
    }
}
