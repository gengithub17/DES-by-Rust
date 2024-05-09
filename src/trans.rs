use crate::values;

#[allow(dead_code,non_snake_case)]
pub fn trans_IP(m:u64, l:&mut u32, r:&mut u32)->i32{
	let transm:u64 = {
		let mut _ret:u64 = 0;
		for i in 0..64 {
			_ret <<= 1;
			if (m&(1u64<<values::IP[i])) != 0 {
				_ret |= 1;
			}
		}

		_ret // return
	};
	*l = (transm>>32) as u32;
	*r = (transm&0xFFFFFFFF) as u32;
	#[cfg(feature="DEBUG_TRANS")]
	{
		println!("trans_IP");
		println!("before =\t{:064b}",m);
		println!("after =\t{:032b} {:032b}\n",*l,*r);
	}
	return 0;
}

#[allow(dead_code,non_snake_case)]
pub fn trans_inv_IP(l:u32, r:u32, ret:&mut u64)->i32{
	let tmp:u64 = ((l as u64)<<32) | (r as u64);
	*ret = {
		let mut _ret:u64 = 0;
		for i in 0..64 {
			_ret <<= 1;
			if (tmp&(1u64<<values::IP_1[i])) != 0 {
				_ret |= 1;
			}
		}

		_ret //return
	};
	#[cfg(feature="DEBUG_TRANS")]
	{
		println!("trans_inv_IP");
		println!("before =\t{:032b} {:032b}",l,r);
		println!("after =\t{:064b}\n",*ret);
	}
	return 0;
}

#[allow(dead_code)]
#[cfg(feature="TEST_MODE")]
pub fn test(m:u64)->i32{
	let mut l:u32 = 0; let mut r:u32 = 0;
	println!("*****Trans Test*****");
	println!("m = \t{:064b}",m);
	println!("--trans_IP--");
	trans_IP(m, &mut l, &mut r);
	println!("mid = \t{:032b}{:032b}", l, r);
	let mut newm:u64 = 0;
	println!("--inv_trans_IP--");
	trans_inv_IP(l, r, &mut newm);
	println!("newm = \t{:064b}",newm);
	if m == newm {
		println!("Success : Transposed 'm' matched original 'm'!");
		return values::TEST::SUCCESS as i32;
	}else {
		println!("Fail : Transposed 'm' did not match original 'm'!");
		return values::TEST::FAIL as i32;
	}
}