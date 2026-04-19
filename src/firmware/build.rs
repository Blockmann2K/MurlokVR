fn main() {
    linker_be_nice();

    // Make Sure 'linkall.x' Is the Last Linker Script (Otherwise Might Cause Problems With Flip-Link).
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn linker_be_nice() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let kind = &args[1];
        let what = &args[2];

        match kind.as_str() {
            "undefined-symbol" => match what.as_str() {
                what if what.starts_with("_defmt_") => {
                    eprintln!();
                    eprintln!("💡 `defmt` Not Found - Make Sure `defmt.x` Is Added as a Linker Script and You Have Included `use defmt_rtt as _;`.");
                    eprintln!();
                }

                "_stack_start" => {
                    eprintln!();
                    eprintln!("💡 Is the Linker Script `linkall.x` Missing?");
                    eprintln!();
                }

                what if what.starts_with("esp_rtos_") => {
                    eprintln!();
                    eprintln!("💡 `esp-radio` Has No Scheduler Enabled. Make Sure You Have Initialized `esp-rtos` or Provided an External Scheduler.");
                    eprintln!();
                }

                "embedded_test_linker_file_not_added_to_rustflags" => {
                    eprintln!();
                    eprintln!("💡 `embedded-test` Not Found - Make Sure `embedded-test.x` Is Added as a Linker Script for Tests.");
                    eprintln!();
                }

                "free" | "malloc" | "calloc" | "get_free_internal_heap_size" | "malloc_internal" | "realloc_internal" | "calloc_internal" | "free_internal" => {
                    eprintln!();
                    eprintln!("💡 Did You Forget the `esp-alloc` Dependency or Didn't Enable the `compat` Feature on It?");
                    eprintln!();
                }

                _ => (),
            },

            // We Don't Have Anything Helpful for "missing-lib" Yet...
            _ => {
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }

    println!("cargo:rustc-link-arg=--error-handling-script={}", std::env::current_exe().unwrap().display());
}
