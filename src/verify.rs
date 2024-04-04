use anyhow::{bail, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    env,
    io::{stdout, Write},
    process::Output,
    time::Duration,
};

use crate::exercise::{Exercise, Mode, State};

pub enum VerifyState<'a> {
    AllExercisesDone,
    Failed(&'a Exercise),
}

// Verify that the provided container of Exercise objects
// can be compiled and run without any failures.
// Any such failures will be reported to the end user.
// If the Exercise being verified is a test, the verbose boolean
// determines whether or not the test harness outputs are displayed.
pub fn verify<'a>(
    pending_exercises: impl IntoIterator<Item = &'a Exercise>,
    progress: (usize, usize),
    verbose: bool,
    success_hints: bool,
) -> Result<VerifyState<'a>> {
    let (num_done, total) = progress;
    let bar = ProgressBar::new(total as u64);
    let mut percentage = num_done as f32 / total as f32 * 100.0;
    bar.set_style(
        ProgressStyle::default_bar()
            .template("Progress: [{bar:60.green/red}] {pos}/{len} {msg}")
            .expect("Progressbar template should be valid!")
            .progress_chars("#>-"),
    );
    bar.set_position(num_done as u64);
    bar.set_message(format!("({percentage:.1} %)"));

    for exercise in pending_exercises {
        let compile_result = match exercise.mode {
            Mode::Test => compile_and_test(exercise, RunMode::Interactive, verbose, success_hints)?,
            Mode::Compile => compile_and_run_interactively(exercise, success_hints)?,
            Mode::Clippy => compile_only(exercise, success_hints)?,
        };
        if !compile_result {
            return Ok(VerifyState::Failed(exercise));
        }
        percentage += 100.0 / total as f32;
        bar.inc(1);
        bar.set_message(format!("({percentage:.1} %)"));
    }

    bar.finish();
    println!("You completed all exercises!");

    Ok(VerifyState::AllExercisesDone)
}

#[derive(PartialEq, Eq)]
enum RunMode {
    Interactive,
    NonInteractive,
}

// Compile and run the resulting test harness of the given Exercise
pub fn test(exercise: &Exercise, verbose: bool) -> Result<()> {
    compile_and_test(exercise, RunMode::NonInteractive, verbose, false)?;
    Ok(())
}

// Invoke the rust compiler without running the resulting binary
fn compile_only(exercise: &Exercise, success_hints: bool) -> Result<bool> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {exercise}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let _ = exercise.run()?;
    progress_bar.finish_and_clear();

    prompt_for_completion(exercise, None, success_hints)
}

// Compile the given Exercise and run the resulting binary in an interactive mode
fn compile_and_run_interactively(exercise: &Exercise, success_hints: bool) -> Result<bool> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Running {exercise}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let output = exercise.run()?;
    progress_bar.finish_and_clear();

    if !output.status.success() {
        warn!("Ran {} with errors", exercise);
        {
            let mut stdout = stdout().lock();
            stdout.write_all(&output.stdout)?;
            stdout.write_all(&output.stderr)?;
            stdout.flush()?;
        }
        bail!("TODO");
    }

    prompt_for_completion(exercise, Some(output), success_hints)
}

// Compile the given Exercise as a test harness and display
// the output if verbose is set to true
fn compile_and_test(
    exercise: &Exercise,
    run_mode: RunMode,
    verbose: bool,
    success_hints: bool,
) -> Result<bool> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Testing {exercise}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let output = exercise.run()?;
    progress_bar.finish_and_clear();

    if !output.status.success() {
        warn!(
            "Testing of {} failed! Please try again. Here's the output:",
            exercise
        );
        {
            let mut stdout = stdout().lock();
            stdout.write_all(&output.stdout)?;
            stdout.write_all(&output.stderr)?;
            stdout.flush()?;
        }
        bail!("TODO");
    }

    if verbose {
        stdout().write_all(&output.stdout)?;
    }

    if run_mode == RunMode::Interactive {
        prompt_for_completion(exercise, None, success_hints)
    } else {
        Ok(true)
    }
}

fn prompt_for_completion(
    exercise: &Exercise,
    prompt_output: Option<Output>,
    success_hints: bool,
) -> Result<bool> {
    let context = match exercise.state()? {
        State::Done => return Ok(true),
        State::Pending(context) => context,
    };
    match exercise.mode {
        Mode::Compile => success!("Successfully ran {}!", exercise),
        Mode::Test => success!("Successfully tested {}!", exercise),
        Mode::Clippy => success!("Successfully compiled {}!", exercise),
    }

    let no_emoji = env::var("NO_EMOJI").is_ok();

    let clippy_success_msg = if no_emoji {
        "The code is compiling, and Clippy is happy!"
    } else {
        "The code is compiling, and 📎 Clippy 📎 is happy!"
    };

    let success_msg = match exercise.mode {
        Mode::Compile => "The code is compiling!",
        Mode::Test => "The code is compiling, and the tests pass!",
        Mode::Clippy => clippy_success_msg,
    };

    if no_emoji {
        println!("\n~*~ {success_msg} ~*~\n");
    } else {
        println!("\n🎉 🎉 {success_msg} 🎉 🎉\n");
    }

    if let Some(output) = prompt_output {
        let separator = separator();
        println!("Output:\n{separator}");
        stdout().write_all(&output.stdout).unwrap();
        println!("\n{separator}\n");
    }
    if success_hints {
        println!(
            "Hints:\n{separator}\n{}\n{separator}\n",
            exercise.hint,
            separator = separator(),
        );
    }

    println!("You can keep working on this exercise,");
    println!(
        "or jump into the next one by removing the {} comment:",
        style("`I AM NOT DONE`").bold()
    );
    println!();
    for context_line in context {
        let formatted_line = if context_line.important {
            format!("{}", style(context_line.line).bold())
        } else {
            context_line.line
        };

        println!(
            "{:>2} {}  {}",
            style(context_line.number).blue().bold(),
            style("|").blue(),
            formatted_line,
        );
    }

    Ok(false)
}

fn separator() -> console::StyledObject<&'static str> {
    style("====================").bold()
}
