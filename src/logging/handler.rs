use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar};
use std::{
    fmt::Display,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use crate::{internal::utils::wrap_text, uwu};
use dialoguer::Confirm;

use super::Verbosity;
use parking_lot::RwLock;

const OK_SYMBOL: &str = "❖";
const ERR_SYMBOL: &str = "X";
const WARN_SYMBOL: &str = "!";
const DEBUG_SYMBOL: &str = "⌘";
const TRACE_SYMBOL: &str = "🗲";
const PROMPT_SYMBOL: &str = "?";

pub struct LogHandler {
    level: Arc<RwLock<Verbosity>>,
    output_type: Arc<RwLock<OutputType>>,
    uwu_enabled: Arc<AtomicBool>,
}

impl Default for LogHandler {
    fn default() -> Self {
        Self {
            level: Arc::new(RwLock::new(Verbosity::Info)),
            output_type: Arc::new(RwLock::new(OutputType::Stderr)),
            uwu_enabled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[allow(unused)]
pub enum OutputType {
    Stdout,
    Stderr,
    MultiProgress(Arc<MultiProgress>),
    Progress(Arc<ProgressBar>),
}

#[allow(unused)]
pub enum PromptDefault {
    Yes,
    No,
    None,
}

impl LogHandler {
    pub fn log_error(&self, msg: String) {
        if self.is_loggable(Verbosity::Error) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", ERR_SYMBOL.red().bold(), msg.bold().red());
            self.log(msg);
        }
    }

    pub fn log_warning(&self, msg: String) {
        if self.is_loggable(Verbosity::Warning) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", WARN_SYMBOL.yellow(), msg.yellow().bold());
            self.log(msg);
        }
    }

    pub fn log_info(&self, msg: String) {
        if self.is_loggable(Verbosity::Info) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", OK_SYMBOL.purple(), msg.bold());
            self.log(msg);
        }
    }

    pub fn log_debug(&self, msg: String) {
        if self.is_loggable(Verbosity::Debug) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", DEBUG_SYMBOL.blue(), msg);

            self.log(msg);
        }
    }

    pub fn log_trace(&self, msg: String) {
        if self.is_loggable(Verbosity::Trace) {
            let msg = self.preformat_msg(msg);
            let msg = format!("{} {}", TRACE_SYMBOL.cyan(), msg.dimmed());
            self.log(msg);
        }
    }

    /// Prompts the user with a question and a default selection
    pub fn prompt(&self, question: String, p_default: PromptDefault) -> bool {
        let question = self.preformat_msg(question);
        let question = format!("{} {}", PROMPT_SYMBOL.purple(), question.bold());
        let mut confirm = Confirm::new();
        confirm.with_prompt(question);

        match p_default {
            PromptDefault::Yes => {
                confirm.default(true);
            }
            PromptDefault::No => {
                confirm.default(false);
            }
            PromptDefault::None => {}
        }
        confirm.interact().unwrap()
    }

    pub fn print_list<I: IntoIterator<Item = T>, T: Display>(&self, list: I, separator: &str) {
        let lines = list
            .into_iter()
            .map(|l| self.preformat_msg(l.to_string()))
            .fold(String::new(), |acc, line| {
                format!("{}{}{}", acc, separator, line)
            });

        let lines = wrap_text(lines).join("\n");
        self.log(lines)
    }

    pub fn print_newline(&self) {
        self.log(String::from("\n"))
    }

    pub fn set_verbosity(&self, level: Verbosity) {
        (*self.level.write()) = level;
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn reset_output_type(&self) {
        self.set_output_type(OutputType::Stdout);
    }

    /// Creates a new progress spinner and registers it on the log handler
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn new_progress_spinner(&self) -> Arc<ProgressBar> {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(250));

        let mut output_type = self.output_type.write();

        if let OutputType::MultiProgress(mp) = &*output_type {
            Arc::new(mp.add(pb.clone()))
        } else {
            let pb = Arc::new(pb);
            *output_type = OutputType::Progress(pb.clone());

            pb
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn new_multi_progress(&self) -> Arc<MultiProgress> {
        let mp = Arc::new(MultiProgress::new());
        self.set_output_type(OutputType::MultiProgress(mp.clone()));

        mp
    }

    /// Registeres a progress bar on the log handler
    #[tracing::instrument(level = "trace", skip_all)]
    fn set_progress_bar(&self, pb: Arc<ProgressBar>) {
        self.set_output_type(OutputType::Progress(pb))
    }

    /// Sets the output type of the log handler to either stdout/stderr or a progress bar
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn set_output_type(&self, output: OutputType) {
        (*self.output_type.write()) = output;
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn set_uwu_enabled(&self, enabled: bool) {
        self.uwu_enabled
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
    }

    pub(crate) fn is_loggable(&self, level: Verbosity) -> bool {
        (*self.level.read()) >= level
    }

    fn preformat_msg(&self, msg: String) -> String {
        let msg = self.apply_uwu(msg);

        wrap_text(msg).join("\n")
    }

    fn apply_uwu(&self, msg: String) -> String {
        if self.uwu_enabled.load(std::sync::atomic::Ordering::Relaxed) {
            uwu!(msg)
        } else {
            msg
        }
    }

    fn log(&self, msg: String) {
        let output_type = self.output_type.read();
        match &*output_type {
            OutputType::Stdout => println!("{}", msg),
            OutputType::Stderr => eprintln!("{}", msg),
            OutputType::MultiProgress(m) => {
                let _ = m.println(msg);
            }
            OutputType::Progress(p) => p.println(msg),
        };
    }
}
