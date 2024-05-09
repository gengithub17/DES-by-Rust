use crate::values;

#[allow(dead_code)]
pub fn init_key(k:u64, c:&mut u32, d:&mut u32)->i32{
	let transk:u64 = {
		let mut _ret:u64 = 0;
		for i in 0..56 {
			_ret <<= 1;
			if k&(1u64<<values::PC_1[i]) != 0 {
				_ret |= 1;
			}
		}

		_ret // return
	};
	*c = (transk>>28) as u32;
	*d = (transk&0xFFFFFFF) as u32;
	return 0;
}

#[allow(dead_code)]
pub fn rotate_shift(val:u32, len:usize, shift:usize, ret:&mut u32)->i32{
	let lsb:u32 = val>>(len-shift);
	*ret = ((val<<shift)|lsb) & ((1u32<<len)-1);
	return 0;
}

#[allow(dead_code)]
pub fn key_manage(round:usize, c:u32, d:u32, newk:&mut u64, newc:&mut u32, newd:&mut u32)->i32{
	rotate_shift(c, 28, values::Key_Shift[round] as usize, newc);
	rotate_shift(d, 28, values::Key_Shift[round] as usize, newd);
	*newk = {
		let mut _ret:u64 = 0;
		let bit56:u64 = ((*newc as u64)<<28) | (*newd as u64);
		for i in 0..48 {
			_ret <<= 1;
			if bit56&(1u64<<values::PC_2[i]) != 0 {
				_ret |= 1;
			}
		}
		_ret //return
	};
	
	return 0;
}

#[allow(dead_code)]
pub fn inv_key_manage(round:usize, c:u32, d:u32, newk:&mut u64, newc:&mut u32, newd:&mut u32)->i32{
	*newk = {
		let mut _ret:u64 = 0;
		let bit56:u64 = ((c as u64)<<28) | (d as u64);
		for i in 0..48 {
			_ret <<= 1;
			if bit56&(1u64<<values::PC_2[i]) != 0 {
				_ret |= 1;
			}
		}
		_ret //return
	};
	rotate_shift(c, 28, 28-values::Key_Shift[round] as usize, newc);
	rotate_shift(d, 28, 28-values::Key_Shift[round] as usize, newd);
	return 0;
}