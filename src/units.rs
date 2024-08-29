use std::f64::consts::PI;


// CODATA 2018 taken from
// https://physics.nist.gov/cuu/Constants/index.html
// Fundamental physical constants as public constants
pub const C: f64 = 299792458.0;          // Speed of light in vacuum (m/s)          Exact
pub const MU0: f64 = 4.0e-7 * PI;        // Vacuum permeability (N/A^2)             Exact
pub const GRAV: f64 = 6.67430e-11;       // Gravitational constant (m^3 kg^-1 s^-2) +/- 0.000_15e-11
pub const HPLANCK: f64 = 6.62607015e-34; // Planck constant (J s)                   Exact
pub const E: f64 = 1.602176634e-19;      // Elementary charge (C)                   Exact
pub const ME: f64 = 9.10938356e-31;      // Electron mass (kg)                      +/- 0.000_000_0028e-31
pub const MP: f64 = 1.67262192369e-27;   // Proton mass (kg)                        +/- 0.000_000_000_51e-27
pub const NAV: f64 = 6.02214076e23;      // Avogadro's number (mol^-1)              Exact
pub const K: f64 = 1.380649e-23;         // Boltzmann constant (J/K)                Exact
pub const AMU: f64 = 1.66053906660e-27;  // Atomic mass unit (kg)                   +/- 0.000_000_000_50e-27


pub const EPS0: f64 = 1.0 / (MU0) / C / C; // Vacuum permittivity (F/m)
pub const HBAR: f64 = HPLANCK / (2.0 * PI); // Reduced Planck constant (J s)
pub const MOL: f64 = NAV; // Mole

// ATOMIC UNITS (AU)
pub const BOHR: f64 = 1.0; // Bohr radius
pub const ANG: f64 = 4e10 * PI * EPS0 * HBAR * HBAR/ ME/ E / E; // Angstrom
pub const NM: f64 = ANG * 1e-1; // Nanometer
pub const M: f64 = ANG * 1e-10; // Meter

pub const SECOND: f64 = M * M * ME/ HBAR; // Second
pub const FS: f64 = SECOND * 1e15; // Femtosecond

pub const V:f64 = M/SECOND; // Velocity

pub const HARTREE: f64 = 1.0; // Hartree energy
pub const E_V: f64 = ME *E*E*E / 16.0 / PI/PI/ EPS0/EPS0/ HBAR/ HBAR; // Electron Volt
pub const J: f64 = E * E_V; // Joule
pub const KJ: f64 = J * 1e-3; // KiloJoule
pub const KCAL: f64 =  KJ/4.184; // KiloCalorie
pub const RYDBERG : f64 = 0.5 * HARTREE; // Rydberg energy
pub const RY: f64 = RYDBERG;
pub const HA: f64 = HARTREE;

pub const KG: f64 = ME; // Kilogram

pub const N: f64 = J / M; // Force

pub const PASCAL: f64 =  J / M/ M /M ; // Pascal
pub const BAR: f64 = 1e-5 * PASCAL; // Bar 
pub const GPA: f64 = 1e-9 * PASCAL; // GPa

pub const KB: f64 = K / J ; // Boltzmann constant (Hartree/K)
pub const C_AU: f64 =  C/(M/SECOND) ; // Speed of light in vacuum (a.u.)
pub const ALPHA:f64 = 1.0 / C_AU; // Fine structure constant
