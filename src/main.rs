use clap::Parser;
use clap::Subcommand;

use json_diff::enums::Error;
use json_diff::{
    ds::{key_node::KeyNode, mismatch::Mismatch},
    process::compare_jsons,
};

#[derive(Subcommand, Clone)]
/// Input selection
enum Mode {
    /// File input
    #[clap(short_flag = 'f')]
    File { file_1: String, file_2: String },
    /// Read from CLI
    #[clap(short_flag = 'd')]
    Direct { json_1: String, json_2: String },
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Mode,

    #[clap(short, long, default_value_t = 20)]
    /// truncate keys with more chars then this parameter
    truncation_length: usize,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let (json_1, json_2) = match args.cmd {
        Mode::Direct { json_2, json_1 } => (json_1, json_2),
        Mode::File { file_2, file_1 } => {
            let d1 = vg_errortools::fat_io_wrap_std(file_1, &std::fs::read_to_string)?;
            let d2 = vg_errortools::fat_io_wrap_std(file_2, &std::fs::read_to_string)?;
            (d1, d2)
        }
    };

    let mismatch = compare_jsons(&json_1, &json_2)?;

    let comparison_result = check_diffs(mismatch)?;
    if !comparison_result {
        std::process::exit(1);
    }
    Ok(())
}

pub fn check_diffs(result: Mismatch) -> Result<bool, Error> {
    let no_mismatch = Mismatch {
        left_only_keys: KeyNode::Nil,
        right_only_keys: KeyNode::Nil,
        keys_in_both: KeyNode::Nil,
    };

    if no_mismatch == result {
        println!("No mismatch");
        Ok(true)
    } else {
        let mismatches = result.all_diffs();
        for (d_type, key) in mismatches {
            println!("{d_type}: {key}");
        }
        Ok(false)
    }
}
