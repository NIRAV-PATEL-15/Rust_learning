#[derive(Debug)]
struct Intrest{
    principal:f32,
    rate:f32,
    time:f32,
    period:Option<f32>,
}

impl Intrest {
    fn simple_interest(&self) -> f32{
        self.principal * self.rate * self.time / 100.00
    }
    fn compound_interest(&self)-> f32{
        if let Some(period) = self.period {
            let n_times_t = self.time * period;
            let rate_per_period = self.rate / period;
            let amount = self.principal * f32::powf(1.0 + rate_per_period, n_times_t);
            amount - self.principal
        } else {
            
            0.0 
        }
    }
}
fn main() {
   let mut instance1 = Intrest{
    principal : 10000.0,
    rate:7.25,
    time:4.0,
    period:None
};
println!("Simple intrest of given values is : {}",instance1.simple_interest());
instance1.period = Some(2.0);
println!("compound intrest of given values is : {:?}",instance1.compound_interest());
}
