use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};
use indoc::{indoc, indoc_impl};

pub fn build_cli() -> App<'static, 'static> {
	App::new("psutil")
		.version("prealpha")
		.author("github.com/elbaro/psutil")
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
		.subcommand(
			SubCommand::with_name("validate")
				.arg(Arg::with_name("validator").index(1).required(true))
				.arg(
					Arg::with_name("paths")
						.index(2)
						.required(true)
						.takes_value(true)
						.multiple(true),
				)
				.arg(Arg::with_name("filter").long("filter").takes_value(true)),
		)
		.subcommand(
			SubCommand::with_name("eval")
				.arg(Arg::with_name("solution").index(1).required(true))
				.arg(Arg::with_name("data_dir").index(2).required(true))
				.arg(Arg::with_name("eval"))
				.arg(Arg::with_name("time-limit").long("time").takes_value(true))
				.arg(
					Arg::with_name("memory-limit")
						.long("memory")
						.takes_value(true),
				),
		)
		.subcommand(
			SubCommand::with_name("new")
				.arg(Arg::with_name("path").index(1).required(true))
				.arg(
					Arg::with_name("from")
						.long("from")
						.takes_value(true)
						.multiple(true),
				)
				.arg(Arg::with_name("c"))
				.arg(Arg::with_name("cpp"))
				.arg(Arg::with_name("java"))
				.arg(Arg::with_name("rust"))
				.arg(Arg::with_name("python"))
				.group(ArgGroup::with_name("lang").args(&["c", "cpp", "java", "rust", "python"]))
				.usage("psutil new dir1/dir2 --from cf 1032H --cpp")
				.about(indoc!(
					"
					102/A/A.cpp
					102/A/problem.txt
					102/A/input1.txt
					102/A/input2.txt
					102/A/output1.txt
					102/A/output2.txt
					"
				)),
		)
		.subcommand(
			SubCommand::with_name("submit")
				.about("submit a single code")
				.arg(Arg::with_name("vendor").index(1).required(true))
				.arg(Arg::with_name("prob").index(2).required(true))
				.arg(Arg::with_name("code").index(3).required(true)),
		)
		.subcommand(SubCommand::with_name("show"))
}
