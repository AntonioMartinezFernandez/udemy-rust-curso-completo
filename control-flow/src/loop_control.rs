pub fn loop_control() {
    /* LOOP */
    let mut loop_times = 3;
    loop {
        println!("loopiiiing...");
        loop_times -= 1;

        if loop_times == 0 {
            break;
        }
    }

    let loop_result = loop {
        loop_times += 1;
        if loop_times == 5 {
            break loop_times;
        }
    };
    println!("'loop' result: {}", loop_result);
}
