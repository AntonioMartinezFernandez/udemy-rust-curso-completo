pub fn labeled_blocks() {
    /* LABELED BLOCKS */
    let some_number: Option<i32> = Some(1);

    'single_block: {
        let Some(num) = some_number else{
          break 'single_block;
        };

        println!("num from single block: {}", num)
    };

    'processor_block: {
        'loop_block: loop {
            let Some(num) = some_number else{
              break 'processor_block;
            };

            if num > 1 {
                println!("num from loop block is greather than 1");
                break 'loop_block;
            }

            println!("num from loop block: {}", num);
            break 'loop_block;
        }
    }

    let result_from_block: i32 = 'result_block: {
        'loop_block: loop {
            let Some(num) = some_number else{
          break 'result_block 0;
        };

            if num > 1 {
                break 'loop_block 1;
            }

            break 'loop_block num;
        }
    };
    println!("result from block: {result_from_block}")
}
