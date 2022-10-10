// https://leetcode.com/problems/number-of-recent-calls/
// How to think?
// The problem need comparing the new values with old values (pings) to check if exceeded time limit,
// The low time limit is 3000 millisecond , so we set the time_span_limit to t-3000 and check every new ping
// We push every ping to last_ping Queue and with each ping we pop_front the queue to check its content
// if the content is lower than the time_span_limit , the value is popped from the queue,
// if the content is not lower the time_span_limit , we push the new value and return the queue length.

use std::collections::VecDeque;

struct RecentCounter {
    pings: VecDeque<i32>,

}

impl RecentCounter {

    fn new() -> Self {
        RecentCounter{
            pings: VecDeque::new()
        }    }

    fn ping(&mut self, t: i32) -> i32 {
        let time_span_limit = t - 3000;
        while let Some(ping)= self.pings.front(){
            match *ping <time_span_limit {
                true => self.pings.pop_front(),
                false=> break,
            };

        }
        self.pings.push_back(t);
        self.pings.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_recent_calls() {
        let test_cases = vec![0,1,100,3001,3002, 3005, 3007,4008, 9000];
        let mut obj = RecentCounter::new();
        // let t = 2000;
        for &i in test_cases.iter(){
            let ret = obj.ping(i);
            println!("{}",ret);
        }
    }
}

