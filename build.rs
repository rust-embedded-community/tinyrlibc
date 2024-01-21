fn main() {
	if cfg!(feature = "snprintf") {
		// Build our snprintf substitute (which has to be C as Rust doesn't do varargs)
		let mut build = cc::Build::new();
		
        build
            .warnings(true)
			.extra_warnings(true)
			.flag("-std=c99")
			.file("./src/snprintf.c");

		#[cfg(not(feature = "itoa"))]
		{
			build.define("itoa", "tinyrlibc_itoa");
		}
		#[cfg(not(feature = "utoa"))]
		{
			build.define("utoa", "tinyrlibc_utoa");
		}
		#[cfg(not(feature = "strtoul"))]
		{
			build.define("strtoul", "tinyrlibc_strtoul");
		}

		build.compile("clocal");
	}

	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=src/snprintf.c");
}
