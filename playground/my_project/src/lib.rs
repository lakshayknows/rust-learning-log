pub mod timings {

    use std::{thread::sleep, time::{Duration, SystemTime}};

    pub fn time1() -> SystemTime{
        let now = SystemTime::now();
        sleep(Duration::new(2,0));
        match now.elapsed() {
            Ok(elapsed) => {
                println!("{}",elapsed.as_secs());
    }
        Err(e)=> {
            println!("Great Scott!{e:?}");
        }
        }
        now
    }
    pub fn time2(n:Duration) -> SystemTime {
        let past = SystemTime::UNIX_EPOCH;
        println!("{:?}",past - n);
        past
    }
  }

