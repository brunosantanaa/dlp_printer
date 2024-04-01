pub struct Axes {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
}

#[derive(Clone, Copy)]
pub enum Subcommands {
    E(f32),
    F(f32),
    S(f32),
    P(f32),
}
pub enum Commands {
    G0 {
        axes: Option<Axes>,
        subcommands: Option<Vec<Subcommands>>,
    }, // Linear move
    G1 {
        axes: Option<Axes>,
        subcommands: Option<Vec<Subcommands>>,
    }, // Linear move
    G4 {
        time: Subcommands
    }, // Dwell - sleep time
    /* Soon */
    /*
    G20, // Inch unit
    G21, // Millimeter Units
    G28 {
        axes: Axes
    }, // Auto home
    M18 {
        axes: Option<Axes>
    }, //  Disable steppers
    */
    M106 {
        pwm: Subcommands,
    }, // Light brightness
    M6054 {
        path: String
    } // Draw picture
}