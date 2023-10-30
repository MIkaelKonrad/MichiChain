use std::time::{SystemTime,UNIX_EPOCH};
use std::fmt ;


pub fn now() -> u128 {
	let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
	duration.as_secs() as u128 *1000 + duration.as_millis() as u128
}

pub type BlockHash = String ;
#[derive(Clone)]
pub struct Block {
	pub index: u32,
	pub timestamp: u128,
	pub hash: BlockHash  ,
	pub prev_hash: BlockHash ,
	pub nonce: u128,
	pub content: String , 
	pub difficulty: u128,
}




impl Block {
	pub fn new(index:u32, timestamp: u128, prev_hash: BlockHash, nonce: u128, content:String, difficulty: u128 ) -> Self { Block {index,timestamp,hash: "0".to_string(), prev_hash, nonce, content,difficulty}}
}

impl Blockchain {
	pub fn start_chain(block : Block) -> Self{ Blockchain{blocks:[block].to_vec()}}

	pub fn verify(&self) -> bool{
		for (i,block) in self.blocks.iter().enumerate() {
			if block.index != i as u32 {
				println!("index missmatch: block {} indexed as {}",&i,&block.index);
				return false
			} else if !check_difficulty(&block, compute_difficulty(&block)) {
				println!("block # {} not mined properly",&i );
				return false
			} else if i >= 1 {
				if block.prev_hash != self.blocks[i-1].hash {
					println!("prev_hash of block # {}  is wrong",&i );
					return false
				} else if block.timestamp <= self.blocks[i-1].timestamp {
					println!("chronology impossible block # {} was created before or simultaniously block # {}", &i,&i -1 );
				} 
			}  
		}
		true
	}
}
impl fmt::Debug for Blockchain {
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		let n = &self.blocks.len() ;
		let mut  _blocks = "".to_string();
		for i in 0..*n {
			_blocks.push_str(&format!("{:?}", &self.blocks[i]))

		}
		write!(f,"{}",_blocks)
	}
}

pub fn change_hash(a: String, b:Block) -> Block {Block{index: b.index, timestamp: b.timestamp, hash:a,prev_hash : b.prev_hash, nonce: b.nonce, content: b.content, difficulty: b.difficulty} }

pub fn change_nonce(a: u128 , b:Block) ->  Block {Block{index: b.index, timestamp: b.timestamp, hash:b.hash ,prev_hash : b.prev_hash, nonce: a, content: b.content, difficulty: b.difficulty} }

pub fn extend_chain(b_c:  Blockchain) -> Blockchain { 
	let n = &b_c.blocks.len();
	let mut block : Block = Block::new(*n as u32, now(), b_c.blocks[n-1].hash.clone().to_string() , 0 , "a new Block".to_string() ,  b_c.blocks[n-1].difficulty );
	block = mine(block);
	let mut b_c = b_c;
	b_c.blocks.push(block);
	Blockchain{blocks: b_c.blocks}

}

pub fn change_content(a:String, b_c: Blockchain)-> Blockchain{ 
	let n = &b_c.blocks.len();
	let mut block : Block = Block::new(*n as u32, now(), b_c.blocks[n-1].hash.clone().to_string() , 0 , a ,  b_c.blocks[n-1].difficulty );
	block = mine(block);
	let mut b_c = b_c;
	b_c.blocks.push(block);
	Blockchain{blocks: b_c.blocks}
} 

impl fmt::Debug for Block {
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result { 
		write!(f, "#:{}, timestamp: {}, hash:{:?}, prev_hash:{:?}, nonce: {}, content: {}, difficulty: {} ", &self.index, &self.timestamp,&self.hash, &self.prev_hash, &self.nonce, &self.content , &self.difficulty)} 
}


use sha256::digest;

pub trait Hashable{

	fn bytes(&self)-> String ;

	fn hash(&self)-> String{ digest(&self.bytes() )}

}



impl Hashable for Block{
	fn bytes(&self)-> String{ 

		let _index = self.index.to_string();
		let _timestamp = self.timestamp.to_string();
		let _prev_hash = self.prev_hash.to_string();
		let _nonce = self.nonce.to_string();
		let _content = self.content.to_string();
		let _difficulty = self.difficulty.to_string();
		let block_string = [_index, _timestamp,_prev_hash,_nonce,_content,_difficulty];
		let mut _final = "".to_owned();
		for i in block_string {
			_final.push_str(&i);
		}
		return _final}
}


pub fn compute_difficulty(b: &Block) -> u128{
	let h = &b.hash;
	let mut blabla: u128 = 0;
	for i in 0 .. 16 {
		let mut ch = h.chars().nth(i).unwrap().to_string();
		let base: u128 = 16;
		if ch == "0".to_string() { 
		}else if ch == "1"{ blabla = blabla +1*(base.pow(15-i as u32))
		}else if ch == "2"{ blabla = blabla +2*(base.pow(15-i as u32))
		}else if ch == "3"{ blabla = blabla +3*(base.pow(15-i as u32))
		}else if ch == "4"{ blabla = blabla +4*(base.pow(15-i as u32))
		}else if ch == "5"{ blabla = blabla +5*(base.pow(15-i as u32))
		}else if ch == "6"{ blabla = blabla +6*(base.pow(15-i as u32))
		}else if ch == "7"{ blabla = blabla +7*(base.pow(15-i as u32))
		}else if ch == "8"{ blabla = blabla +8*(base.pow(15-i as u32))
		}else if ch == "9"{ blabla = blabla +9*(base.pow(15-i as u32))
		}else if ch == "a"{ blabla = blabla +10*(base.pow(15-i as u32))
		}else if ch == "b"{ blabla = blabla +11*(base.pow(15-i as u32))
		}else if ch == "c"{ blabla = blabla +12*(base.pow(15-i as u32))
		}else if ch == "d"{ blabla = blabla +13*(base.pow(15-i as u32))
		}else if ch == "e"{ blabla = blabla +14*(base.pow(15-i as u32))
		}else if ch == "f"{ blabla = blabla +15*(base.pow(15-i as u32))
		}
	};
	blabla

}

pub fn mine(block:Block)->Block {
	for j in 0..(u64::max_value() ) {
		let block = change_nonce(j as u128 ,block.clone());
		let block = change_hash(block.hash(),block.clone());
		let mut i = compute_difficulty(&block);
		if check_difficulty(&block,i ) {
				return block;
			}
		}
	block
}


pub fn check_difficulty(b: &Block, difficulty: u128)-> bool {
	b.difficulty > difficulty
}

pub struct Blockchain {
	pub blocks: Vec<Block>,
}