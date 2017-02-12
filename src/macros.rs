// Description: Convenience macros for Assembunny-plus

/// Meant to be used in fn's returning `Result<_, String>`.
/// $todo expects an expression that returns a Result.
/// Macro, if $todo was Ok, returns the unwrapped item.
macro_rules! try_failsafe {
    ( $todo:expr, $err:expr ) => (match $todo {
        Ok(unwrapped) => unwrapped,
        Err(_) => return Err($err)
    })
}

/// Reads a certain file to String and returns that.
/// Same as try_failsafe!, this macro requires fn's calling it to return Result<(), String>.
macro_rules! file_to_string {
	( $filename:expr ) => ({
		let mut file = try_failsafe!(File::open($filename), format!("File not found for path {:?}", $filename));
		let mut fcontents = String::new();
		try_failsafe!(file.read_to_string(&mut fcontents), format!("Error reading file {:?}", $filename));
		fcontents
	})
}

/// Tries to do $todo,
/// If the Result is Err, this macro makes the parent function return Err containing a String, format(ted)! from $prefix + $todo's Err message.
///
/// Example:
/// try_err_fallthru!(read_file(), "File read failed: ") can make the parent function return Err("File read failed: ENOENT '/usr/sandwich/make.sh'")
macro_rules! try_err_fallthru {
    ( $todo:expr, $prefix:expr ) => (match $todo {
        Ok(unwrapped) => unwrapped,
        Err(errmsg) => return Err(format!("{}{}", $prefix, errmsg))
    })
}

/// Aborts the program with exit code 1.
macro_rules! abort {
    () => (std::process::exit(1))
}