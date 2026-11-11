# Connect to QEMU. 
set tcp auto-retry on
set tcp connect-timeout 5
target remote 127.0.0.1:3333

# Avoid stepping into printing helpers.
skip function firmware::uart::print_fmt
skip -rfunction ^core::fmt::
