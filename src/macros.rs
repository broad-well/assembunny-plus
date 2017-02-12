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

/// Just like try_failsafe, except for Option instead of Result regarding token $todo.
macro_rules! try_opt {
    ( $todo:expr, $err:expr ) => (match $todo {
        Some(val) => val,
        None => return Err($err)
    })
}

/// Reads a certain file to String and returns that.
/// Same as try_failsafe!, this macro requires fn's calling it to return Result<_, String>.
macro_rules! file_to_string {
	( $filename:expr ) => ({
		let mut file = try_failsafe!(File::open($filename), format!("File not found for path {:?}", $filename));
		let mut fcontents = String::new();
		try_failsafe!(file.read_to_string(&mut fcontents), format!("Error reading file {:?}", $filename));
		fcontents
	})
}

/// Reads a certain file to Vec of u8 (bytes) and returns that.
/// Same as try_failsafe!, this macro requires fn's calling it to return Result<_, String>.
macro_rules! file_to_bytevec {
    ( $filename:expr ) => ({
        let mut file = try_failsafe!(File::open($filename), format!("File not found for path {:?}", $filename));
        let mut bytes: Vec<u8> = Vec::new();
        try_failsafe!(file.read_to_end(&mut bytes), format!("Error reading file {:?}", $filename));
        bytes
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