use longitude::{Direction, Distance, Location};
use rand::Rng;

pub fn get_rand_cord(center: &Location, radius: &Distance) -> Location {
    let mut rand = rand::rng();
    let angle = rand.random::<f64>() * std::f64::consts::PI;
    let dist = rand.random::<f64>();

    let distance_north = Distance::from_kilometers(radius.kilometers() * dist * dist * angle.sin());
    let distance_east = Distance::from_kilometers(radius.kilometers() * dist * dist * angle.cos());

    center
        .add(&distance_north, Direction::North)
        .add(&distance_east, Direction::East)
}
