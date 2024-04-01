use super::gtypes::{Axes, Subcommands};

pub fn print(axes: Option<Axes>, subcommands: Option<Vec<Subcommands>>) {
    if let Some(subcmds) = subcommands {
        for subcmd in subcmds {
            match subcmd {
                Subcommands::E(e_value) => print!(" E: {} |", e_value),
                Subcommands::F(f_value) => print!(" F: {} |", f_value),
                Subcommands::S(s_value) => print!(" S: {} |", s_value),
                Subcommands::P(p_value) => print!(" P: {} |", p_value),
            }
        }
    }
    if let Some(ax) = axes {
        if let Some(x_value) = ax.x {
            print!(" x: {} |", x_value);
        }
        if let Some(y_value) = ax.y {
            print!(" y: {} |", y_value);
        }
        if let Some(z_value) = ax.z {
            print!(" z: {} |", z_value);
        }
    }
    println!();
}
