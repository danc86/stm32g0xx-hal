use core::ops::{Add, Div};

/// A measurement of a monotonically nondecreasing clock
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Instant(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Bps(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Hertz(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct MicroSecond(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `Hertz`
    fn khz(self) -> Hertz;

    /// Wrap in `Hertz`
    fn mhz(self) -> Hertz;

    /// Wrap in `MicroSecond`
    fn us(self) -> MicroSecond;

    /// Wrap in `MicroSecond`
    fn ms(self) -> MicroSecond;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        assert!(self > 0);
        Bps(self)
    }

    fn hz(self) -> Hertz {
        assert!(self > 0);
        Hertz(self)
    }

    fn khz(self) -> Hertz {
        Hertz(self.saturating_mul(1_000))
    }

    fn mhz(self) -> Hertz {
        Hertz(self.saturating_mul(1_000_000))
    }

    fn ms(self) -> MicroSecond {
        MicroSecond(self.saturating_mul(1_000))
    }

    fn us(self) -> MicroSecond {
        MicroSecond(self)
    }
}

impl Hertz {
    pub fn duration(&self, cycles: u32) -> MicroSecond {
        let cycles = cycles as u64;
        let clk = self.0 as u64;
        let us = cycles.saturating_mul(1_000_000_u64) / clk;
        MicroSecond(us as u32)
    }
}

impl Add for Hertz {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Div for Hertz {
    type Output = u32;

    fn div(self, other: Self) -> Self::Output {
        self.0 / other.0
    }
}

impl MicroSecond {
    pub fn cycles(&self, clk: Hertz) -> u32 {
        assert!(self.0 > 0);
        let clk = clk.0 as u64;
        let period = self.0 as u64;
        let cycles = clk.saturating_mul(period) / 1_000_000_u64;
        cycles as u32
    }
}

impl Add for MicroSecond {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Into<MicroSecond> for Hertz {
    fn into(self) -> MicroSecond {
        assert!(self.0 <= 1_000_000);
        MicroSecond(1_000_000 / self.0)
    }
}

impl Into<Hertz> for MicroSecond {
    fn into(self) -> Hertz {
        let period = self.0;
        assert!(period > 0 && period <= 1_000_000);
        Hertz(1_000_000 / period)
    }
}
