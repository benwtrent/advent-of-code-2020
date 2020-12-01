use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

static COOKIE: &str = "";

async fn input<T: Sized>(day: usize, parser: fn(String) -> T) -> Result<T> {
    let res = reqwest::Client::builder()
        .build()?
        .get(format!("https://adventofcode.com/2020/day/{}/input", day).as_str())
        .header("Cookie", COOKIE)
        .send()
        .await?;
    Ok(parser(res.text().await?))
}

fn sums_to<'a>(
    sum: &'_ usize,
    curr: &'a usize,
    rest: &'a [usize],
) -> Option<(&'a usize, &'a usize)> {
    if rest.len() == 0 {
        return Option::None;
    }
    for re in rest {
        if re + curr == *sum {
            return Option::Some((curr, re));
        }
    }
    sums_to(sum, &rest[0], &rest[1..])
}

fn tri_sums_to<'a>(
    sum: &'_ usize,
    x: &'a usize,
    rest: &'a [usize],
) -> Option<(&'a usize, &'a usize, &'a usize)> {
    if rest.len() == 0 {
        return Option::None;
    }
    if x < sum {
        let sub_sum = *sum - x;
        for i in 0..(rest.len() - 1) {
            if let Some((a, b)) = sums_to(&sub_sum, &rest[i], &rest[i + 1..]) {
                return Option::Some((x, a, b));
            }
        }
    }
    tri_sums_to(sum, &rest[0], &rest[1..])
}

async fn input_to_vec(day: usize) -> Result<Vec<usize>> {
    Ok(input::<Vec<usize>>(day, |s: String| {
        s.lines()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<usize>>()
    })
    .await?)
}

async fn day1_1() -> Result<usize> {
    let input = input_to_vec(1).await?;
    let (v1, v2) = sums_to(&2020, &input[0], &input[1..]).unwrap();
    Ok(v1 * v2)
}

async fn day1_2() -> Result<usize> {
    let input = input_to_vec(1).await?;
    let (v1, v2, v3) = tri_sums_to(&2020, &input[0], &input[1..]).unwrap();
    Ok(v1 * v2 * v3)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("answer:\n{:?}", day1_2().await?);
    Ok(())
}
