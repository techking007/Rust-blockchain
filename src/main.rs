
mod block_models;
fn main() {
   let difficulty = 1;
   let mut blockchain = block_models::blockchain::Blockchain::new(difficulty);
   block_models::blockchain::Blockchain::add_block(&mut blockchain);
}