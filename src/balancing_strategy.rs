use crate::{BalancingStrategy, RandomStrategy, RoundRobinStrategy};
use rand::random_range;

impl BalancingStrategy for RandomStrategy {

    fn take_server(&mut self) -> &str {
        let random_ix = random_range(..self.servers.len());
        self.servers[random_ix].as_str()
    }

    fn get_server_back(&mut self, _: &str) {}
}

impl BalancingStrategy for RoundRobinStrategy {

    fn take_server(&mut self) -> &str {
        let server = self.servers.get(self.ix).unwrap();
        self.ix += 1;
        if self.ix >= self.servers.len(){
            self.ix = 0;
        }
        server
    }

    fn get_server_back(&mut self, _: &str) {}
}
