
use Michilib::*;

pub fn run() -> () {
	
	let x: u128 = now();
	println!("{:?}",x );
	let block = Block::new(0,now(),"00000000".to_string(),0,"asdf".to_owned(), 0x0000ffffffffffff);
	
	let h = block.hash();
	
	let block : Block = change_hash(h,block);
	
	let block = mine(block);
	//println!{"{:?}",block};
	let mut chain = Blockchain::start_chain(block);
	for i in 0..10{
		chain = change_content(i.to_string(),chain);
	}
	chain = change_content("Hello World".to_string(), chain);
	// Still need to check verify() with wrong chains.
	
	println!("{:?}", chain.verify());
	
	println!("{:?}",chain );

	
	

}


