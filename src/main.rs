use std::collections::HashMap;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use swayipc::{Connection, Output};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Clone, Debug, Serialize, Subcommand, ValueEnum)]
enum Direction {
    Next,
    Prev,
}

#[derive(Subcommand, Debug)]
enum Mode {
    #[command(flatten)]
    Switch(Direction),
    Move {
        #[arg(value_enum)]
        direction: Direction,
    },
}

fn get_focused_output(con: &mut Connection) -> Result<Output> {
    let outputs = con.get_outputs()?;
    let focused = outputs
        .iter()
        .find(|o| o.focused)
        .ok_or_else(|| anyhow!("There is no focused output"))?;
    Ok(focused.clone())
}

fn get_workspace(con: &mut Connection, direction: Direction) -> Result<i32> {
    // Get focused workspace
    let workspaces = con.get_workspaces()?;
    let output = get_focused_output(con)?;
    let focused = workspaces
        .iter()
        .find(|w| w.output == output.name && w.focused)
        .ok_or_else(|| anyhow!("There is no focused workspace"))?;
    let current = std::cmp::max(focused.num, 1);

    // Find the closest one in the same output
    let mut map: HashMap<i32, &str> = HashMap::with_capacity(workspaces.len());
    let mut end = 1;
    for w in &workspaces {
        map.insert(w.num, w.output.as_str());
        if w.num > end {
            end = w.num
        };
    }
    let check = |n: &i32| -> bool {
        match map.get(n) {
            // Workspace exists on this output
            Some(&o) if o == output.name => true,
            // There is a gap
            None => true,
            _ => false,
        }
    };

    match direction {
        Direction::Next => {
            let mut it = (current + 1)..=end;
            Ok(it.find(check).unwrap_or(end + 1))
        }
        Direction::Prev => {
            let mut it = (1..current).rev();
            Ok(it.find(check).unwrap_or(current))
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut con = Connection::new()?;

    match args.mode {
        Mode::Switch(direction) => {
            let workspace = get_workspace(&mut con, direction)?;
            con.run_command(format!("workspace {}", workspace))?;
        }
        Mode::Move { direction } => {
            let workspace = get_workspace(&mut con, direction)?;
            con.run_command(format!("move container to workspace {}", workspace))?;
            con.run_command(format!("workspace {}", workspace))?;
        }
    }

    Ok(())
}
