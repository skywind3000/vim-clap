pub mod dynamic;

use std::path::Path;

use anyhow::Result;
use fuzzy_filter::{fuzzy_filter_and_rank, truncate_long_matched_lines, Algo, Source};

use icon::prepend_icon;

/// Return the info of the truncated top items ranked by the filtering score.
fn print_top_items<T>(
    total: usize,
    top_size: usize,
    top_list: impl IntoIterator<Item = (String, T, Vec<usize>)>,
    winwidth: usize,
    enable_icon: bool,
) {
    let (truncated_lines, truncated_map) = truncate_long_matched_lines(top_list, winwidth, None);
    let mut lines = Vec::with_capacity(top_size);
    let mut indices = Vec::with_capacity(top_size);
    if enable_icon {
        for (text, _, idxs) in truncated_lines {
            lines.push(prepend_icon(&text));
            indices.push(idxs);
        }
    } else {
        for (text, _, idxs) in truncated_lines {
            lines.push(text);
            indices.push(idxs);
        }
    }
    if truncated_map.is_empty() {
        println_json!(total, lines, indices);
    } else {
        println_json!(total, lines, indices, truncated_map);
    }
}

pub fn run<I: Iterator<Item = String>>(
    query: &str,
    source: Source<I>,
    algo: Option<Algo>,
    number: Option<usize>,
    enable_icon: bool,
    winwidth: Option<usize>,
) -> Result<()> {
    let ranked = fuzzy_filter_and_rank(query, source, algo.unwrap_or(Algo::Fzy))?;

    if let Some(number) = number {
        let total = ranked.len();
        print_top_items(
            total,
            number,
            ranked.into_iter().take(number),
            winwidth.unwrap_or(62),
            enable_icon,
        );
    } else {
        for (text, _, indices) in ranked.iter() {
            println_json!(text, indices);
        }
    }

    Ok(())
}

/// Looks for matches of `query` in lines of the current vim buffer.
pub fn blines(
    query: &str,
    input: &Path,
    number: Option<usize>,
    winwidth: Option<usize>,
) -> Result<()> {
    run(
        query,
        Source::List(
            std::fs::read_to_string(&input)?
                .lines()
                .enumerate()
                .map(|(idx, item)| format!("{} {}", idx + 1, item)),
        ),
        None,
        number,
        false,
        winwidth,
    )
}
