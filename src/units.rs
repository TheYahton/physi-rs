use crate::SIUnit;

// Base units
pub const METER: SIUnit = SIUnit::meter(1.0);
pub const KILOGRAM: SIUnit = SIUnit::kilogram(1.0);
pub const SECOND: SIUnit = SIUnit::second(1.0);
pub const AMPERE: SIUnit = SIUnit::ampere(1.0);
pub const KELVIN: SIUnit = SIUnit::kelvin(1.0);
pub const MOLE: SIUnit = SIUnit::mole(1.0);
pub const CANDELA: SIUnit = SIUnit::candela(1.0);

// Other
pub const MINUTE: SIUnit = SIUnit::second(60.0);
pub const HOUR: SIUnit = SIUnit::second(3600.0);
pub const DAY: SIUnit = SIUnit::second(86400.0);

// TODO: CELCIUS
// 0°C + 0°C = 273.15K + 273.15K = 546.3K = 273.15°C
// pub const CELCIUS: SIUnit = SIUnit::kelvin(273.15);

pub const MILLIMETER: SIUnit = SIUnit::meter(0.001);
pub const CENTIMETER: SIUnit = SIUnit::meter(0.01);
pub const DECIMETER: SIUnit = SIUnit::meter(0.1);
pub const KILOMETER: SIUnit = SIUnit::meter(1000.0);

pub const LITRE: SIUnit = SIUnit::new(0.001, 3, 0, 0, 0, 0, 0, 0);
