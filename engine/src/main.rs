use dispersal_engine::Engine;
fn main(){let mut e=Engine::new(101);for i in 0..101{e.cell(i,2000.);if i>0{e.edge(i-1,i,55.6);}}e.seed(50,1000.);for _ in 0..1000{e.step(0.008,0.1);}println!("{{\"kind\":\"synthetic-corridor\",\"years\":{},\"population\":{},\"occupied_cells\":{}}}",e.year,e.population.iter().sum::<f32>(),e.population.iter().filter(|p|**p>=25.).count());}
