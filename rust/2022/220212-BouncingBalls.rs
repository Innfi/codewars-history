//https://www.codewars.com/kata/5544c7a5cb454edb3c000047

/* 
fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
        -1
    } else {
        (window / h).log(bounce).ceil() as i32 * 2 - 1
    }
}
*/

#[cfg(test)]
mod tests {
    fn to_bounce_count(h: f64, bounce: f64, window: f64) -> i32 {
        let mut bounce_count = 0;
        let mut starter = h;
        
        while starter > window {
            bounce_count += 1;
            starter = starter * bounce;

            if starter <= window { break; }

            bounce_count += 1;
        }

        bounce_count
    }

    #[test]
    fn bouncer() {
        let bounce = 0.66;
        let starter = 3.0;
        let limit = 1.5;

        assert_eq!(to_bounce_count(starter, bounce, limit), 3);
    }

    fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
        if h <= 0.0 { return -1; }
        if bounce <= 0.0 { return -1; }
        if bounce >= 1.0 { return -1; }
        if h <= window { return -1; }

        to_bounce_count(h, bounce, window)
    }

    fn testequal(h: f64,  bounce: f64,  window: f64, exp: i32) -> () {
        assert_eq!(bouncing_ball(h, bounce, window), exp)
    }
    
    #[test]
    fn tests_bouncing_ball() {
        testequal(3.0, 0.66, 1.5, 3);
        testequal(30.0, 0.66, 1.5, 15);
        testequal(40.0, 0.4, 10.0, 3);
        testequal(10.0, 0.6, 10.0, -1);
        testequal(2.0, 0.5, 1.0, 1);
        testequal(40.0, 1.0, 10.0, -1);
    }
}
