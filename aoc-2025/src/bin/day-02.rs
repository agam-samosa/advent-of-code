use anyhow::Context as _;
use std::fmt::Write as _;
use std::{io, ops::RangeInclusive};

fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;

    println!("Part 1: {}", part_1(&input)?);

    Ok(())
}

fn part_1(inp: &str) -> anyhow::Result<i64> {
    let mut sum_invalids = 0;

    let mut buf = String::new();
    for range_raw in inp.split(',') {
        let (start_raw, end_raw) = range_raw.split_once('-').context("Invalid Range!")?;
        let (start, end) = (start_raw.parse()?, end_raw.parse()?);
        let range: RangeInclusive<i64> = start..=end;

        for id in range {
            write!(&mut buf, "{id}")?;
            let (l, r) = buf.split_at(buf.len() / 2);

            if l == r {
                sum_invalids += id;
            }

            buf.clear();
        }
    }

    Ok(sum_invalids)
}
