// // use near_sdk_sim::{ExecutionResult};

// pub fn get_error_count(r: &ExecutionResult) -> u32 {
//     r.promise_errors().len() as u32
// }

// pub fn get_error_status(r: &ExecutionResult) -> String {
//     format!("{:?}", r.promise_errors()[0].as_ref().unwrap().status())
// }

// pub fn get_logs(r: &ExecutionResult) -> Vec<String> {
//     let mut logs: Vec<String> = vec![];
//     r.promise_results()
//         .iter()
//         .map(|ex| {
//             ex.as_ref()
//                 .unwrap()
//                 .logs()
//                 .iter()
//                 .map(|x| logs.push(x.clone()))
//                 .for_each(drop)
//         })
//         .for_each(drop);
//     logs
// }

// pub(crate) fn to_nano(timestamp: u32) -> u64 {
//     u64::from(timestamp) * 10u64.pow(9)
// }

use near_workspaces::{network::Sandbox, result::ExecutionFinalResult, Worker};

pub async fn wait_seconds(worker: &Worker<Sandbox>, seconds: u64) -> u64 {
    if seconds > 100 {
        panic!(
            "seconds is way too high. Max is: 100\nseconds is {}",
            seconds
        );
    }
    println!("Waiting {seconds} seconds");
    let start = worker
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec()
        / 10_u64.pow(9);
    let mut waited = 0;
    let mut timestamp = 0;
    while waited < seconds {
        worker.fast_forward(1).await.unwrap();
        let current = worker
            .view_block()
            .await
            .unwrap()
            .header()
            .timestamp_nanosec()
            / 10_u64.pow(9);
        waited = current - start;
        if waited > timestamp {
            timestamp = waited;
            println!("waiting ({timestamp})...");
        }
    }
    worker
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec()
        / 10_u64.pow(9)
}

pub async fn wait_until(worker: &Worker<Sandbox>, timestamp: u64) {
    let now: u64 = worker
        .view_block()
        .await
        .unwrap()
        .header()
        .timestamp_nanosec();
    if now > timestamp {
        println!(
            "No need to wait until {timestamp}. Currently {} nanoseconds ahead",
            now - timestamp
        );
    } else {
        let wait_time = (timestamp - now + 1) / 10_u64.pow(9);
        wait_seconds(worker, wait_time).await;
    }
}

pub async fn error_contains(res: &ExecutionFinalResult, msg: &str) {
    let receipt_failures = res.receipt_failures();
    assert_eq!(
        receipt_failures.len(),
        1,
        "receipt_failures is {:#?}",
        receipt_failures
    );
    let first = receipt_failures.first().unwrap();
    let first = format!("{first:#?}");
    assert!(first.contains(msg), "first is {:#?}", first);
}
