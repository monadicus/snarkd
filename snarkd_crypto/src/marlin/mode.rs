use core::fmt::Debug;

/// A trait to specify the Marlin mode.
pub trait MarlinMode: 'static + Copy + Clone + Debug + PartialEq + Eq + Sync + Send {
    const ZK: bool;
}

/// The Marlin hiding mode produces a hiding Marlin proof.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MarlinHidingMode;

impl MarlinMode for MarlinHidingMode {
    const ZK: bool = true;
}

/// The Marlin non-hiding mode produces a non-hiding Marlin proof.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MarlinNonHidingMode;

impl MarlinMode for MarlinNonHidingMode {
    const ZK: bool = false;
}
