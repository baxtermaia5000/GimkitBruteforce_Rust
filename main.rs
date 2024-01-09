use reqwest;
use reqwest::StatusCode;
use fastrand;
#[tokio::main]
async fn join_code(code: &i32, client: &reqwest::Client) -> bool {
    let payload = [("code",&code.to_string())];
    let url = "https://www.gimkit.com/api/matchmaker/find-info-from-code";
    let response = client
        .post(url)
        .form(&payload)
        .send()
        .await
        .unwrap();
    if response.status() == StatusCode::OK {
        //println!("All clear at {code}");
        return true;
    }
    else {
        //println!("No good at {code}");
        return false;
    }
}

fn main() {
    println!("Press ctrl + c to stop");
    let client = reqwest::Client::new();
    loop {
        let code = fastrand::i32(100000..999999);
        //println!("checking {code}...");
        let code_bool = join_code(&code,&client);
        if code_bool == true {
            println!("Success at {code}");
        }
    };
}

