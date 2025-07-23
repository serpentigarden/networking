mod dhcp;
use std::io::Error;

fn main() -> Result<(), Error> {
    dhcp::allocate_new_host()
}
