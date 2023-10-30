

pub fn Michi_hash(a: &str) -> (){
	let  vect : &[u8] = a.as_bytes();
	let n = vect.len() as usize;
	let mut K :  [u32 ;8] = [0,0,0,0,0,0,0,0];
	for i in 0..n{
		let j : usize = i % 8 ;
		K[j] = (K[j]+ (vect[i] as u32 )) %256 ;

	}
	let mut P :  [u8 ;8] = [0,0,0,0,0,0,0,0];
	for i in 0 .. 8 {
		P[i] = K[i] as u8;
	}
	println!("{:?}",K );
	println!("{:?}",P );
	
	let mut U :  [char ;8] = ['0','0','0','0','0','0','0','0'];
	for i in 0 .. 8 {
		
		if P[i] >= 100{
			U[i] = 135 as char;
		} else {
			let q= P[i] +100;
			U[i] = q as char;
		}
	}
	
	println!("U: {:?}",U );
	println!("thîs is {}", 135 as char);
}