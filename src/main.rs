use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};
use colored::*;
use indoc::{indoc, indoc_impl};
use num_traits::{Num, Zero};
use piston_window::*;
use rand::distributions::Uniform;
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

struct Range<X> {
    low: X,
    high: X,
}

fn generate_tree<X>(n: usize, weight_range: Option<Range<X>>)
where
    X: Num + rand::distributions::uniform::SampleUniform + std::fmt::Display,
{
    let range = Uniform::new(1, n + 1);
    let binary = Uniform::new(0, 2);
    let mut rng = rand::thread_rng();

    let prufer: Vec<usize> = (0..n - 2).map(|_| rng.sample(range)).collect();
    let mut degree = vec![1; n + 1];
    for &v in &prufer {
        degree[v] += 1;
    }

    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(n - 1);

    for &v in &prufer {
        for other in 1..n + 1 {
            if degree[other] == 1 {
                edges.push((v, other));
                degree[v] -= 1;
                degree[other] -= 1;
                break;
            }
        }
    }

    for v in 1..n + 1 {
        if degree[v] == 1 {
            for u in v + 1..n + 1 {
                if degree[u] == 1 {
                    edges.push((v, u));
                    break;
                }
            }
            break;
        }
    }

    rng.shuffle(&mut edges);
    for e in &mut edges {
        if rng.sample(&binary) == 0 {
            std::mem::swap(&mut e.0, &mut e.1);
        }
    }

    println!("{}", n);
    match weight_range {
        Some(range) => {
            let dist = Uniform::new(range.low, range.high);
            for e in edges {
                let w = rng.sample(&dist);
                println!("{} {} {}", e.0, e.1, w);
            }
        }
        None => {
            for e in edges {
                println!("{} {}", e.0, e.1);
            }
        }
    }
}

// valtr algo
// http://cglab.ca/~sander/misc/ConvexGeneration/convex.html
fn generate_convex<X>(n: usize, coord_range: Range<X>)
where
    X: Num
        + rand::distributions::uniform::SampleUniform
        + std::fmt::Display
        + std::cmp::PartialOrd
        + num_traits::Signed
        + std::ops::AddAssign
        + num_traits::AsPrimitive<i8>,
{
    assert!(n >= 3);

    let coord_dist = Uniform::new(coord_range.low, coord_range.high);
    let mut rng = rand::thread_rng();

    // random points in square -> O(n^n) trials in average
    let generate_chains = &|| -> (X, Vec<X>) {
        let binary = Uniform::new(0, 2);
        let mut rng = rand::thread_rng();
        // return n vectors that sums up to 0
        let mut a: Vec<X> = (0..n).map(|_| rng.sample(&coord_dist)).collect();
        a.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let mut chains: Vec<X> = Vec::with_capacity(n);
        let mut last1 = 1;
        let mut last2 = 1;
        for i in 2..n {
            if rng.sample(binary) == 0 {
                chains.push(a[i] - a[last1]);
                last1 = i;
            } else {
                chains.push(a[last2] - a[i]);
                last2 = i;
            }
        }
        chains.push(a[n - 1] - a[last1]);
        chains.push(a[last2] - a[n - 1]);
        (a[1], chains)
    };

    let (minx, mut xs) = generate_chains();
    let (miny, ys) = generate_chains();

    rng.shuffle(&mut xs);
    // rng.shuffle(&mut ys);
    let mut vec: Vec<(X, X)> = (0..n).map(|i| (xs[i], ys[i])).collect();

    // if a vec is (0,0), it should be randomly inserted later

    // sort (xs[i], ys[i]) by angle
    vec.sort_unstable_by(|&(x1, y1), &(x2, y2)| {
        let sx1 = if x1.is_positive() {
            1
        } else if x1.is_negative() {
            3
        } else if y1.is_positive() {
            2
        } else {
            4
        };
        let sx2 = if x2.is_positive() {
            1
        } else if x2.is_negative() {
            3
        } else if y2.is_positive() {
            2
        } else {
            4
        };
        if sx1 != sx2 {
            return sx1.cmp(&sx2);
        }
        if sx1.is_zero() {
            return y2.partial_cmp(&y1).unwrap();
        }
        return (y1 * x2).partial_cmp(&(y2 * x1)).unwrap();
    });

    // how much shift y
    let mut y = X::zero();
    let mut my = X::zero();
    for i in 0..n {
        y += vec[i].1;
        if y < my {
            my = y;
        }
    }

    // shift my -> miny
    // let ret:Vec<(,)> = Vec::with_capacity(n);
    let mut x = minx;
    y = miny - my;
    println!("{}", n);
    for i in 0..n {
        // ret.push((x,y));
        println!("{} {}", x, y);
        // println!("+= {} {}", vec[i].0, vec[i].1);
        x += vec[i].0;
        y += vec[i].1;
    }

    // assert!((x-minx).is_zero(), "sum of vector x = {} {} ", x,minx);
    // assert!((y-(miny-my)).is_zero(), "sum of vector y = {} {} ", y,miny-my);
    // 3.0644979766624543
    // 3.0644979766624516
}

fn sanitize_file<P: AsRef<Path>>(path: P, confirmed: bool) -> Result<bool, Box<Error>> {
    // 1. already good
    // 2. sanitized
    // 3. error

    // lowercase ext
    // - line ending to LF (remove any CR)
    // - make sure newline at EOF
    // - make sure ascii
    let path: &Path = path.as_ref();

    //    if let Some(ext) = path.extension() {
    //        if !ext.to_str()?.bytes().all(|b| b>=b'a' && b<=b'z') {
    //
    //            println!("  Renamed: {} => {}", path.display(),);
    //        }
    //    }

    // 1. check
    {
        let f = File::open(&path)?;
        let reader = BufReader::new(f);
        let mut it = reader.bytes();
        let mut cr = false;
        let mut last: u8 = b'\n';
        while let Some(b) = it.next() {
            let b = b?;
            // ascii range? 127=DEL
            // non-control? 32=SPACE, 9=TAB
            if !(b < 127 && (b >= 32 || b == 10 || b == 13)) {
                return Err(format!("non-ascii byte: {}", b).into());
            }
            if b == 13 {
                cr = true;
            }
            last = b;
        }

        if !cr && last == b'\n' {
            return Ok(false); // already good
        }
    }

    if confirmed {
        let f = File::open(&path)?;
        let reader = BufReader::new(f);
        let mut it = reader.bytes();

        let tmp_path = &path.with_extension("tmp");
        let tmp_file = File::create(&tmp_path)?;
        let mut bw = BufWriter::new(tmp_file);

        // let last_cr = false; // last_cr == skip_lf
        let mut last: u8 = 0;
        while let Some(b) = it.next() {
            let b = b?;
            // it == CR
            if last == b'\r' && b == b'\n' {
                // pass
            } else if b == b'\r' {
                bw.write(&[b'\n'])?;
            } else {
                bw.write(&[b])?;
            }
            last = b;
        }
        if last != b'\r' && last != b'\n' {
            bw.write(&[b'\n'])?;
        }
        std::fs::rename(tmp_path, path)?;
    }

    println!("Converted: {}", path.display());
    Ok(true)
}

fn sanitize<P: AsRef<Path>>(path: P, exts: Vec<&str>, confirmed: bool) {
    println!("   Exts: {:?}", exts);

    let mut good = 0;
    let mut changed = 0;
    let mut error = 0;

    for entry in WalkDir::new(path) {
        match entry {
            Ok(e) => {
                let path = e.path();
                if let Some(ext) = path.extension() {
                    if exts.contains(&ext.to_str().unwrap()) {
                        match sanitize_file(path, confirmed) {
                            Ok(true) => {
                                changed += 1;
                            }
                            Ok(false) => {
                                good += 1;
                            }
                            Err(err) => {
                                eprintln!("[Error] {}", err);
                                eprintln!("\t=> {}", path.display());
                            }
                        }
                    }
                }
            }
            Err(err) => {
                error += 1;
                println!("[File Error] {:?}", err);
            }
        }
    }
    println!("   {}: {}", "Good".green(), good);
    println!("{}: {}", "Changed".yellow(), changed);
    println!("  {}: {}", "Error".red(), error);
    if !confirmed && changed > 0 {
        eprintln!("\n{}", "Run with --confirmed to make actual change".red());
    }
}

fn show() {
    let mut window: PistonWindow = WindowSettings::new("", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}

mod vendor {
    trait Judge {
        fn overview() {}
        fn read() {}
        fn submit() {}
    }
}

mod codeforces {
    use chrono::{DateTime, Utc};

    enum ContestStatus {
        Scheduled,
        Running,
        Finished,
    }

    struct Codeforces {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    }

    impl Codeforces {
        fn submit() {}
        fn download() {}
    }
}

mod topcoder {}

fn main() {
    let args = App::new("psutil")
        .version("prealpha")
        .author("elbaro@github")
        .about("data util for algo ps")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("generate")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("tree")
                        .arg(Arg::with_name("n").required(true).index(1))
                        .arg(Arg::with_name("int-weight").short("i").number_of_values(2))
                        .arg(
                            Arg::with_name("float-weight")
                                .short("f")
                                .number_of_values(2),
                        )
                        .group(ArgGroup::with_name("weight").args(&["int-weight", "float-weight"]))
                        .arg(Arg::with_name("directed").long("directed")),
                )
                .subcommand(
                    SubCommand::with_name("convex")
                        .arg(Arg::with_name("n").required(true).index(1))
                        .arg(Arg::with_name("int-range").short("i").number_of_values(2))
                        .arg(Arg::with_name("float-range").short("f").number_of_values(2))
                        .group(
                            ArgGroup::with_name("range")
                                .args(&["int-range", "float-range"])
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("sanitize")
                .arg(Arg::with_name("path").index(1))
                .arg(
                    Arg::with_name("ext")
                        .long("ext")
                        .takes_value(true)
                        .multiple(true)
                        .min_values(1)
                        .use_delimiter(true)
                        .required(true),
                )
                .arg(Arg::with_name("confirmed").long("confirmed"))
                .about("psutil sanitize data/A --ext txt,in,out"),
        )
        .subcommand(
            SubCommand::with_name("contest").about("Overview of upcoming or recent contests"),
        )
        .subcommand(SubCommand::with_name("download").about(indoc!(
            "
                102/A/A.cpp
                102/A/problem.txt
                102/A/input1.txt
                102/A/input2.txt
                102/A/output1.txt
                102/A/output2.txt
                "
        )))
        .subcommand(
            SubCommand::with_name("submit")
                .about("submit a single code")
                .arg(Arg::with_name("vendor").index(1))
                .arg(Arg::with_name("prob").index(2))
                .arg(Arg::with_name("code").index(3)),
        )
        .subcommand(SubCommand::with_name("show"))
        .get_matches();

    // app - matches - subcommand(app) - matches - subcommand(tree)
    let sub = args.subcommand.unwrap();
    match sub.name.as_ref() {
        "generate" => {
            let sub = sub.matches.subcommand.unwrap();
            match sub.name.as_ref() {
                "tree" => {
                    let n = sub.matches.value_of("n").unwrap().parse().unwrap();
                    if let Some(mut w) = sub.matches.values_of("int-weight") {
                        let low: i64 = w.next().unwrap().parse().unwrap();
                        let high: i64 = w.next().unwrap().parse().unwrap();
                        generate_tree(n, Some(Range { low, high }));
                    } else if let Some(mut w) = sub.matches.values_of("float-weight") {
                        let low: f64 = w.next().unwrap().parse().unwrap();
                        let high: f64 = w.next().unwrap().parse().unwrap();
                        generate_tree(n, Some(Range { low, high }));
                    } else {
                        generate_tree::<i8>(n, None);
                    }
                }
                "convex" => {
                    let n = sub.matches.value_of("n").unwrap().parse().unwrap();
                    if let Some(mut w) = sub.matches.values_of("int-range") {
                        let low: i64 = w.next().unwrap().parse().unwrap();
                        let high: i64 = w.next().unwrap().parse().unwrap();
                        generate_convex(n, Range { low, high });
                    } else if let Some(mut w) = sub.matches.values_of("float-range") {
                        let low: f64 = w.next().unwrap().parse().unwrap();
                        let high: f64 = w.next().unwrap().parse().unwrap();
                        generate_convex(n, Range { low, high });
                    }
                }
                _ => unreachable!(),
            }
        }
        "sanitize" => {
            let path = sub.matches.value_of("path").unwrap_or(".");
            let exts: Vec<&str> = sub.matches.values_of("ext").unwrap().collect();
            let confirmed = sub.matches.is_present("confirmed");
            sanitize(path, exts, confirmed);
        }
        "download" => {
            unimplemented!();
        }
        "acmicpc" => {
            unimplemented!();
        }
        "codeforces" => {
            unimplemented!();
        }
        "topcoder" => {
            unimplemented!();
        }
        "prepare" => {
            unimplemented!();
        }
        "visualize" => {
            unimplemented!();
            show();
        }
        _ => unreachable!(),
    };
}
