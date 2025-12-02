use anyhow::bail;
use std::io;

fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;

    println!("Part 1: {}", part_1(&input)?);
    println!("Part 2: {}", part_2(&input)?);

    Ok(())
}

fn part_1(inp: &str) -> anyhow::Result<i32> {
    let mut dial = 50;
    let mut z_count = 0;

    for line in inp.lines() {
        let (dir, mag_raw) = line.split_at(1);
        let mag: i32 = mag_raw.parse()?;

        match dir {
            "L" => {
                dial -= mag;
            }
            "R" => {
                dial += mag;
            }
            _ => bail!("Invalid rotation!"),
        }
        dial = dial.rem_euclid(100);

        if dial == 0 {
            z_count += 1;
        }
    }

    Ok(z_count)
}

fn part_2(inp: &str) -> anyhow::Result<i32> {
    let mut dial = 50;
    let mut z_count = 0;

    for line in inp.lines() {
        let (dir, mag_raw) = line.split_at(1);
        let mag: i32 = mag_raw.parse()?;

        z_count += mag / 100;

        match dir {
            "L" => {
                if mag % 100 >= dial && dial != 0 {
                    z_count += 1;
                }
                dial -= mag;
            }
            "R" => {
                if mag % 100 >= (100 - dial) {
                    z_count += 1;
                }
                dial += mag;
            }
            _ => bail!("Invalid rotation!"),
        }
        dial = dial.rem_euclid(100);
    }

    Ok(z_count)
}
