use crate::internal::commands::ShellCommand;
use crate::internal::error::SilentUnwrap;
use crate::internal::exit_code::AppExitCode;
use crate::{prompt, warn, spinner};
use std::env;

pub fn detect() {
    // Start spinner
    let sp = spinner!("Scanning for pacnew files");

    let mut pacnew = vec![];

    // Run `find` to find pacnew files and split by lines into a vec
    let find = ShellCommand::pacdiff()
        .args(&["-o", "-f"])
        .elevated()
        .wait_with_output()
        .silent_unwrap(AppExitCode::PacmanError);
    let find_lines = find.stdout.split('\n');
    for line in find_lines {
        if !line.is_empty() {
            pacnew.push(line.to_string());
        }
    }

    // If pacnew files are found, warn the user and prompt to pacdiff
    if !pacnew.is_empty() {
        sp.stop_bold("It appears that at least one program you have installed / upgraded has installed a .pacnew/.pacsave config file. These are created when you have modified a program's configuration, and a package upgrade could not automatically merge the new file.");

        let choice = prompt!(default false, "Would you like to run pacdiff to deal with this? You can always deal with this later by running `sudo pacdiff`");
        if choice {
            if env::var("PACDIFF_WARNING").unwrap_or_else(|_| "1".to_string()) != "0" {
                warn!("Pacdiff uses `vimdiff` by default to edit files for merging. Make sure you know how to exit vim before proceeding!");
                warn!("You can surpress this warning in the future by setting the `PACDIFF_WARNING` environment variable to `0`");
                let cont = prompt!(default false, "Continue?");
                if cont {
                    ShellCommand::pacdiff()
                        .elevated()
                        .wait()
                        .silent_unwrap(AppExitCode::PacmanError);
                }
            } else {
                ShellCommand::pacdiff()
                    .elevated()
                    .wait()
                    .silent_unwrap(AppExitCode::PacmanError);
            }
        }
    } else {
        sp.stop_bold("No pacnew files found");
    }
}
