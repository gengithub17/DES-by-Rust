use crate::values;

pub fn getval_sbox(input:u8, sboxnum:usize,output: &mut u8)->i32{
	let row:u8; let col:u8;
	row = ((input&0b100000)>>4)|(input&0b000001);
	col = (input&0b011110)>>1;
	*output = values::SBOX[sboxnum][row as usize][col as usize];

	return 0;
}

#[allow(dead_code)]
pub fn f(r:u32, k:u64, ret:&mut u32)->i32{
	let extended:u64 = k^{
		let mut _ret:u64 = 0;
		for i in 0..48 {
			_ret <<= 1;
			if r&(1u32 << values::E[i]) != 0 {
				_ret |= 1;
			}
		}
		#[cfg(feature="DEBUG_ROUND_F")]
		{
			println!("extended before = \t{:048b}",_ret);
		}
		_ret // return
	};
	#[cfg(feature="DEBUG_ROUND_F")]
	{
		println!("extended after = \t{:048b}",extended);
	}

	let bit32:u32 = {
		let mut _ret:u32 = 0;
		for i in 0..values::SBOXNUM {
			_ret <<= 4;
			let bit6 = (extended>>(6*(values::SBOXNUM-1-i)))&0b111111;
			let mut sboxret:u8 = 0;
			getval_sbox(bit6 as u8, i, &mut sboxret);
			_ret |= sboxret as u32;
			#[cfg(feature="DEBUG_ROUND_F")]
			{
				println!("{}th SBox",i);
				println!("\tbit6 = {:06b}",bit6);
				println!("\tbit4 = {:04b}",sboxret);
			}
		}
		_ret //return
	};
	#[cfg(feature="DEBUG_ROUND_F")]
	{
		println!("after sbox = \t\t{:032b}",bit32);
	}
	
	*ret = 0;
	for i in 0..32 {
		*ret <<= 1;
		if bit32&(1u32<<values::P[i]) != 0 {
			*ret |= 1;
		}
	}
	#[cfg(feature="DEBUG_ROUND_F")]
	{
		println!("\tf(r,k) = \t{:032b}\n",*ret);
	}
	return 0;
}

#[allow(dead_code)]
pub fn round(l:u32, r:u32, k:u64, newl:&mut u32, newr:&mut u32)->i32{
	#[cfg(feature="DEBUG_ROUND")]
	{
		println!("Round :\n");
		println!("\tl = \t\t{:032b}",l);
		println!("\tr = \t\t{:032b}",r);
		#[cfg(not(feature="DEBUG_KEY"))]
		{
			println!("\tk = \t\t{:048b}",k);
		}
	}
	*newl = r;

	let mut fret:u32 = 0;
	f(r,k,&mut fret);
	*newr = l^fret;
	#[cfg(feature="DEBUG_ROUND")]
	{
		#[cfg(not(feature="DEBUG_ROUND_F"))]
		{
			println!("\tf(r,k) = \t{:032b}",fret);
		}
		println!("\tnewl = \t\t{:032b}",*newl);
		println!("\tnewr = \t\t{:032b}",*newr);
		println!("");
	}
	return 0;
}

#[allow(dead_code)]
#[cfg(feature="TEST_MODE")]
pub fn test(l:u32, r:u32, k:u64)->i32{
	#[cfg(not(feature="DEBUG_ROUND_F"))]
	{
		println!("It is strongly recommended to enable DEBUG_ROUND for detail values!");
	}
	let mut newl:u32 = 0; let mut newr:u32 = 0;
	round(l,r,k,&mut newl, &mut newr);
	let mut retl:u32 = 0; let mut retr:u32 = 0;
	round(newr,newl,k,&mut retr, &mut retl);
	if retl == l && retr == r {
		println!("Success : Applying the 'Round' function twice will return to the original value");
		return values::TEST::SUCCESS as i32;
	}else {
		println!("Fail : The original values were not returned.");
		return values::TEST::FAIL as i32;
	}
}
