use std::{fmt::{Debug,Display}, ops::{Add, AddAssign, Mul, MulAssign}};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Stats {
    attack: f64,
    life: f64
}
impl Stats {
    pub fn new(attack: f64, life: f64) -> Stats {
        Stats { attack, life }
    }
}
impl Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stats").field("attack", &self.attack.round()).field("life", &self.life.round()).finish()
    }
}
impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.attack.round(), self.life.round())
    }
}
impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        Stats { attack: self.attack + rhs.attack, life: self.life + rhs.life }
    }
}
impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.attack += rhs.attack;
        self.life += rhs.life;
    }
}
impl Mul<f64> for Stats {
    type Output = Stats;

    fn mul(self, rhs: f64) -> Self::Output {
        Stats { attack: self.attack * rhs, life: self.life * rhs }
    }
}
impl Mul<Stats> for f64 {
    type Output = Stats;

    fn mul(self, rhs: Stats) -> Self::Output {
        Stats { attack: self * rhs.attack, life: self * rhs.life }
    }
}
impl Mul<&Stats> for f64 {
    type Output = Stats;

    fn mul(self, rhs: &Stats) -> Self::Output {
        Stats { attack: self * rhs.attack, life: self * rhs.life }
    }
}
impl MulAssign<f64> for Stats {
    fn mul_assign(&mut self, rhs: f64) {
        self.attack *= rhs;
        self.life *= rhs;
    }
}
impl Mul<u32> for Stats {
    type Output = Stats;

    fn mul(self, rhs: u32) -> Self::Output {
        Stats { attack: self.attack * rhs as f64, life: self.life * rhs as f64 }
    }
}
impl Mul<Stats> for u32 {
    type Output = Stats;

    fn mul(self, rhs: Stats) -> Self::Output {
        Stats { attack: self as f64 * rhs.attack, life: self as f64 * rhs.life }
    }
}
impl Mul<&Stats> for u32 {
    type Output = Stats;

    fn mul(self, rhs: &Stats) -> Self::Output {
        Stats { attack: self as f64 * rhs.attack, life: self as f64 * rhs.life }
    }
}
impl MulAssign<u32> for Stats {
    fn mul_assign(&mut self, rhs: u32) {
        self.attack *= rhs as f64;
        self.life *= rhs as f64;
    }
}