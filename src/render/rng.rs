//! Independent random streams for gameplay outcomes and simulated throws.

use std::convert::Infallible;

use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, TryRng};

/// Random stream used only to generate gameplay outcomes.
#[derive(Resource)]
pub struct DiceRng {
    inner: Box<dyn Rng + Send + Sync>,
}

impl DiceRng {
    /// Wraps any existing [`Rng`] implementation.
    pub fn new<R: Rng + Send + Sync + 'static>(rng: R) -> Self {
        Self { inner: Box::new(rng) }
    }

    /// Deterministic RNG seeded with `seed` (uses [`StdRng`] internally).
    pub fn from_seed(seed: u64) -> Self {
        Self::new(StdRng::seed_from_u64(seed))
    }
}

impl Default for DiceRng {
    fn default() -> Self {
        Self::new(rand::make_rng::<StdRng>())
    }
}

impl TryRng for DiceRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        Ok(self.inner.next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        Ok(self.inner.next_u64())
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Infallible> {
        self.inner.fill_bytes(dest);
        Ok(())
    }
}

/// Independent random stream used only for background simulation seeds.
#[derive(Resource)]
pub struct DiceSimulationRng {
    inner: StdRng,
}

impl DiceSimulationRng {
    /// Seeds simulated trajectories without changing gameplay outcomes.
    pub fn from_seed(seed: u64) -> Self {
        Self { inner: StdRng::seed_from_u64(seed) }
    }

    pub fn next_seed(&mut self) -> u64 {
        self.inner.next_u64()
    }
}

impl Default for DiceSimulationRng {
    fn default() -> Self {
        Self { inner: rand::make_rng() }
    }
}
