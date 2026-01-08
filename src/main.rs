use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    
    // i locked stdout to ensure nothing else's written while this writes
    // this prevents other threads from writing concurrently
    let mut handle = stdout.lock();
    
    // i pre-allocated a String buffer with initial capacity (4000 gotta be enough)
    let mut output = String::with_capacity(3100); // Pre-allocated memory
    
    for n in 1..=100 {
        match (n % 3 == 0, n % 5 == 0) {
            (true, true) => output.push_str("FizzBuzz\n"),
            (true, false) => output.push_str("Fizz\n"),
            (false, true) => output.push_str("Buzz\n"),
            (false, false) => {
                output.push_str(&n.to_string());  
                output.push('\n');                
            }
        }
    }
    
    // write the entire accumulated output in one call. faster than multiple writes yk..
    handle.write_all(output.as_bytes())?;
    
    Ok(())
} 