fn main() {
	if cfg!(feature = "snprintf") {
		// Build our snprintf substitute (which has to be C as Rust doesn't do varargs)
		let mut build = cc::Build::new()
			.warnings(true)
			.extra_warnings(true)
			.flag("-std=c99")
			.file("./src/snprintf.c")
			.clone();

		#[cfg(feature = "itoa")]
		{
			build = build.define("itoa", "itoa").clone();
		}
		#[cfg(feature = "utoa")]
		{
			build = build.define("utoa", "utoa").clone();
		}
		#[cfg(feature = "strtoul")]
		{
			build = build.define("strtoul", "strtoul").clone();
		}
		#[cfg(not(feature = "itoa"))]
		{
			build = build.define("itoa", "tinyrlibc_itoa").clone();
		}
		#[cfg(not(feature = "utoa"))]
		{
			build = build.define("utoa", "tinyrlibc_utoa").clone();
		}
		#[cfg(not(feature = "strtoul"))]
		{
			build = build.define("strtoul", "tinyrlibc_strtoul").clone();
		}

		build.compile("clocal");
	}

	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=src/snprintf.c");
}
