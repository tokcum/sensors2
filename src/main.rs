use std::time::{Duration, Instant};

/*
Idea for integration with failcheck:
- Create an Agent as part of the State of the Monitor thread.
- Spawn a new thread for each check (a check might have multiple URLs) and pass a clone of Agent to it.
- Thread returns result which is then used to update the state.
*/

fn main() -> Result<(), ureq::Error> {
    let client = ureq::builder()
        .timeout(Duration::from_millis(500))
        .redirects(2)
        .https_only(true)
        .user_agent("ferris/1.0")
        .build();

    let start = Instant::now();

    let x = client.head("http://kodiak.polarlabs.io/")
        .call();

    let stop = start.elapsed().as_millis();
    println!("request took: {}ms", start.elapsed().as_millis());

    match x {
        Ok(r) => {
            println!("{}, {}: {}", r.get_url(), r.status(), r.status_text());
        },
        Err(ureq::Error::Status(_code, _response)) => {
            println!("Status");
        },
        Err(ureq::Error::Transport(trans_e)) => {
            println!("Error: {}", trans_e.to_string());
        },
        Err(_e) => {},
    }

    Ok(())
}
