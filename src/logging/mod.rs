//! Logging module

use fern::{ Dispatch, colors::{ Color, ColoredLevelConfig } };

pub fn init(filter: log::LevelFilter) {
	// Configuration of colors for the whole line
	let linecl = ColoredLevelConfig::new()
		.error(Color::BrightRed)
		.warn(Color::BrightYellow)
		.info(Color::White)
		.debug(Color::White)
		.trace(Color::BrightBlack);

	let levelcl = ColoredLevelConfig::new()
		.error(Color::BrightRed)
		.warn(Color::BrightYellow)
		.info(Color::Cyan)
		.debug(Color::BrightGreen)
		.trace(Color::BrightBlue);

	Dispatch::new()
		.format(move |out, message, record| {
			out.finish(format_args!(
				"[{date}]{cline}[{level}{cline}] {message}\x1B[0m",
				cline = format_args!("\x1B[{}m", linecl.get_color(&record.level()).to_fg_str()),
				date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
				level = levelcl.color(record.level()),
				message = message,
			));
		})
		// set the default log level. to filter out verbose log messages from dependencies, set
		// this to Warn and overwrite the log level for your crate.
		.level(filter)
		// change log levels for individual modules. Note: This looks for the record's target
		// field which defaults to the module path but can be overwritten with the `target`
		// parameter:
		// `info!(target="special_target", "This log message is about special_target");`
		.level_for("critical", log::LevelFilter::Trace)
		// output to stdout
		.chain(std::io::stdout())
		.apply().unwrap();
}