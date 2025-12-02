use anyhow::Context as _;
use std::fmt::Write as _;
use std::io;

fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;

    println!("Part 1: {}", part_1(&input)?);

    Ok(())
}

fn part_1(inp: &str) -> anyhow::Result<i64> {
    let mut sum_invalids = 0;

    let mut buf = String::new();
    for range in inp.split(',') {
        let (start, end) = range.split_once('-').context("Invalid Range!")?;
        let (start, end): (i64, _) = (start.parse()?, end.parse()?);

        for id in start..=end {
            buf.clear();
            write!(&mut buf, "{id}")?;

            let (l, r) = buf.split_at(buf.len() / 2);
            if l == r {
                sum_invalids += id;
            }
        }
    }

    Ok(sum_invalids)
}

