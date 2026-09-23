//! Deterministic, continuous population mechanics. Not calibrated historical inference.
use std::cell::RefCell;
#[derive(Clone)]
pub struct Engine { pub population: Vec<f32>, capacity: Vec<f32>, edges: Vec<(usize,usize,f32)>, next: Vec<f32>, pub year:u32 }
impl Engine {
 pub fn new(n:usize)->Self { Self{population:vec![0.;n],capacity:vec![0.;n],edges:Vec::new(),next:vec![0.;n],year:0} }
 pub fn cell(&mut self,i:usize,k:f32) { if i<self.capacity.len() && k.is_finite(){self.capacity[i]=k.max(0.);} }
 pub fn edge(&mut self,a:usize,b:usize,km:f32){if a<self.capacity.len() && b<self.capacity.len() && a!=b && km.is_finite() && km>0. {self.edges.push((a,b,km));}}
 pub fn seed(&mut self,i:usize,n:f32){ if i<self.population.len() && self.capacity[i]>0. && n.is_finite(){self.population[i]=n.max(0.);} }
 pub fn step(&mut self,growth:f32,mobility:f32){
  let g=if growth.is_finite(){growth.clamp(0.,0.05)}else{0.};
  let m=if mobility.is_finite(){mobility.clamp(0.,0.2)}else{0.};
  // Exact logistic growth over one year, avoids Euler overshoot above capacity.
  for i in 0..self.population.len(){ let p=self.population[i];let k=self.capacity[i];
   self.next[i]=if k<=0.||p<=0.{0.}else{k*p/(p+(k-p)*(-g).exp())};
  }
  self.population.copy_from_slice(&self.next);
  // Synchronous conservative diffusion; max edge coefficient .05, four-neighbor grid.
  for &(a,b,km) in &self.edges {if self.capacity[a]<=0.||self.capacity[b]<=0.{continue;}
   let cost=(55.6/km).min(1.5);let habitat=(self.capacity[a].min(self.capacity[b])/2000.).clamp(0.1,1.);
   let flow=(self.population[a]-self.population[b])*m*0.25*cost*habitat;
   self.next[a]-=flow; self.next[b]+=flow;
  }
  self.population.copy_from_slice(&self.next); self.year+=1;
 }
}
thread_local!{static STATE:RefCell<Engine>=RefCell::new(Engine::new(0));}
#[no_mangle] pub extern "C" fn init(n:usize){STATE.with(|s|*s.borrow_mut()=Engine::new(n.min(100000)));}
#[no_mangle] pub extern "C" fn set_cell(i:usize,k:f32){STATE.with(|s|s.borrow_mut().cell(i,k));}
#[no_mangle] pub extern "C" fn add_edge(a:usize,b:usize,km:f32){STATE.with(|s|s.borrow_mut().edge(a,b,km));}
#[no_mangle] pub extern "C" fn seed(i:usize,n:f32){STATE.with(|s|s.borrow_mut().seed(i,n));}
#[no_mangle] pub extern "C" fn advance(years:u32,g:f32,m:f32){STATE.with(|s|{let mut e=s.borrow_mut();for _ in 0..years.min(10000){e.step(g,m);}});}
#[no_mangle] pub extern "C" fn population_ptr()->*const f32{STATE.with(|s|s.borrow().population.as_ptr())}
#[cfg(test)]mod tests{
 use super::*;
 fn pair()->Engine{let mut e=Engine::new(3);e.cell(0,2000.);e.cell(1,2000.);e.edge(0,1,55.6);e.seed(0,1000.);e}
 #[test]fn migration_conserves_population(){let mut e=pair();for _ in 0..1000{e.step(0.,0.1);}assert!((e.population.iter().sum::<f32>()-1000.).abs()<0.1);assert_eq!(e.population[2],0.);}
 #[test]fn zero_mobility_blocks_spread(){let mut e=pair();for _ in 0..100{e.step(0.01,0.);}assert_eq!(e.population[1],0.);}
 #[test]fn barriers_block_even_if_edge_supplied(){let mut e=pair();e.edge(1,2,55.6);for _ in 0..100{e.step(0.01,0.2);}assert_eq!(e.population[2],0.);}
 #[test]fn repeatable_and_nonnegative(){let mut a=pair();let mut b=a.clone();for _ in 0..3000{a.step(0.015,0.2);b.step(0.015,0.2);}assert_eq!(a.population,b.population);assert!(a.population.iter().all(|p|p.is_finite()&&*p>=0.));}
 #[test]fn symmetry(){let mut e=Engine::new(3);for i in 0..3{e.cell(i,2000.);}e.edge(0,1,55.6);e.edge(1,2,55.6);e.seed(1,1000.);for _ in 0..100{e.step(0.01,0.1);}assert!((e.population[0]-e.population[2]).abs()<0.001);}
}

/// Least-cost expansion front. Edge weights are kilometers times dimensionless resistance.
#[derive(Clone)]
struct Front { graph:Vec<Vec<(usize,f32)>>, arrivals:Vec<f32> }
impl Front {
 fn new(n:usize)->Self {Self{graph:vec![vec![];n],arrivals:vec![f32::INFINITY;n]}}
 fn solve(&mut self,source:usize,speed:f32){
  self.arrivals.fill(f32::INFINITY);
  if source>=self.graph.len() || !speed.is_finite() || speed<=0. {return;}
  // O(V²) deterministic minimum scan: bounded 8,100-cell interactive regional grid.
  let mut done=vec![false;self.graph.len()];self.arrivals[source]=0.;
  for _ in 0..self.graph.len(){
   let mut u=usize::MAX;let mut best=f32::INFINITY;
   for i in 0..self.graph.len(){if !done[i] && self.arrivals[i]<best {best=self.arrivals[i];u=i;}}
   if u==usize::MAX {break;} done[u]=true;
   for &(v,cost) in &self.graph[u]{let next=best+cost/speed;if next<self.arrivals[v]{self.arrivals[v]=next;}}
  }
 }
}
thread_local!{static FRONT:RefCell<Front>=RefCell::new(Front::new(0));}
#[no_mangle]pub extern "C" fn front_init(n:usize){FRONT.with(|f|*f.borrow_mut()=Front::new(n.min(10000)));}
#[no_mangle]pub extern "C" fn front_edge(a:usize,b:usize,cost:f32){FRONT.with(|f|{let mut f=f.borrow_mut();if a<f.graph.len()&&b<f.graph.len()&&cost.is_finite()&&cost>0.{f.graph[a].push((b,cost));f.graph[b].push((a,cost));}});}
#[no_mangle]pub extern "C" fn front_solve(source:usize,speed:f32){FRONT.with(|f|f.borrow_mut().solve(source,speed));}
#[no_mangle]pub extern "C" fn front_ptr()->*const f32{FRONT.with(|f|f.borrow().arrivals.as_ptr())}
#[cfg(test)]mod front_tests{
 use super::*;
 #[test]fn weighted_shortest_route_and_unreachable(){let mut f=Front::new(4);f.graph[0]=vec![(1,100.),(2,300.)];f.graph[1]=vec![(2,50.)];f.solve(0,0.5);assert_eq!(f.arrivals[2],300.);assert!(f.arrivals[3].is_infinite());}
 #[test]fn zero_speed_is_not_instant_arrival(){let mut f=Front::new(1);f.solve(0,0.);assert!(f.arrivals[0].is_infinite());}
}

pub mod population;
