//! # Art
//! 
//! A library for modelling artisitic concepts
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    //Primary colors acc to RYB Model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }

}

pub mod utils {
    use crate::kinds::*;

    //combines two primary colors in equal amounts to create
    //a secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
            let c3 = SecondaryColor::Green;
            c3
    }
}