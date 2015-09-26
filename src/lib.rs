#![feature(negate_unsigned)]
extern crate libc;
mod bcm_host;

#[cfg(test)]
mod tests {

	#[test]
	fn init_bcm() {
	   bcm_host_init();
	}
}
