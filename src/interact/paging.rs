use std::env;
use std::fmt::Write;

use crate::internal::error::AppError;
use crate::internal::error::AppResult;
use crate::with_suspended_output;

use minus::Pager;

pub fn page_string<S: ToString>(content: S) -> AppResult<()> {
    let mut pager = Pager::new();
    pager.set_prompt(
        env::args().collect::<Vec<String>>().join(" ")
            + " | Q: quit | /: Search | n: next result | p: previous result",
    )?;
    writeln!(pager, "{}", content.to_string())?;
    with_suspended_output!({ minus::page_all(pager).map_err(AppError::from) })
}
