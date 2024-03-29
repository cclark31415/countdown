This is a Rust simple demonstration of how to a) implement a countdown timer, and b) display the text in the same line (overwrite what was previously displayed)

It also implements text colors just for funzies.  The colors are based on the ANSI colors here:
https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit

The way to read the color implementation is as follows:
Given this line `print!("\r\x1b[30;41m    {}    \x1b[0m", x),`
* `\r` is the newline character

* `\x` signifies a hexadecimal value

* `1b` is the ANSI value for `esc`, or escape character that "starts all the escape sequences"
* `[` is a delimiter that is known as a CSI (Control Sequence Introducer).  It begins the escape character sequence that will be a semi-colon separated list
* `30;41m` Black foreground, red background (see table at above link)
* `    {}    ` This is the text to be printed.  The curlies will not be printed in this case as they are formatting delimiters and will be replaced with the value in the variable `x`
* `\x1b[0m` the escape sequence that ends the above escape sequence
