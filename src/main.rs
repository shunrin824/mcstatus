use chrono::Local;
use mc_rcon::RconClient;
use regex;
use std::{env, thread, time::Duration};

fn net_send(content: String, send_type: String) {
    todo!();
}

fn rcon_tps(rcon_client: &RconClient, args: &Vec<String>) -> f32 {
    //10秒間使ってtpsを計測し、f32で返す関数
    let mut rcon_result: String;
    let mut tps: f32 = 0.0;
    match rcon_client.send_command("debug start") {
        Ok(_) => (),
        Err(e) => {
            println!("debug startコマンドに返答がありませんでした。\n{}", e);
            return tps;
        }
    }
    thread::sleep(Duration::from_secs(10));
    match rcon_client.send_command("debug stop") {
        Ok(rcon_result_stop) => rcon_result = rcon_result_stop,
        Err(e) => {
            println!("debug stopコマンドに返答がありませんでした。\n{}", e);
            return tps;
        }
    };

    //rconからの返答をパースする。
    let re = if (&args[2] == &"ex") {
        match regex::Regex::new(r"\((\d+\.\d+) tick(s) per second\)") {
            Ok(re) => re,
            Err(e) => {
                println!("サーバーがtpsを応答していません。\n{}", e);
                return tps;
            }
        }
    } else {
        match regex::Regex::new(r"\((\d+\.\d+) ticks per second\)") {
            Ok(re) => re,
            Err(e) => {
                println!("サーバーがtpsを応答していません。\n{}", e);
                return tps;
            }
        }
    };

    if let Some(caps) = re.captures(&rcon_result) {
        if let Some(cap) = caps.get(1) {
            let Ok(tps) = cap.as_str().to_string().parse::<f32>() else {
                print!("tpsをパースできません！");
                return tps;
            };
            {
                return tps;
            }
        }
    }
    return tps;
    //Stopped tick profiling after 10.03 seconds and 205 ticks (20.45 ticks per second)
}

fn rcon_number_of_players(rcon_client: &RconClient) -> u16 {
    let number_of_players: u16 = 0;
    let rcon_result = match rcon_client.send_command("list") {
        Ok(rcon_result) => rcon_result,
        Err(e) => {
            println!("listコマンドに返答がありませんでした。\n{}", e);
            return number_of_players;
        }
    };

    let re = match regex::Regex::new(r"There are (\d+) of a max of") {
        Ok(re) => re,
        Err(e) => {
            println!("サーバーがlistを応答していません。\n{}", e);
            return number_of_players;
        }
    };

    if let Some(caps) = re.captures(&rcon_result) {
        if let Some(cap) = caps.get(1) {
            let Ok(number_of_players) = cap.as_str().to_string().parse::<u16>() else {
                print!("listの結果をパースできません！");
                return number_of_players;
            };
            {
                return number_of_players;
            }
        }
    }
    return number_of_players;
    //There are 1 of a max of 20 players online: shunrin824
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let client_result: Result<RconClient, std::io::Error> = RconClient::connect(&args[2]);
    match client_result {
        Ok(client) => {
            let _ = client.log_in(&args[3]);
            let number_of_players: u16 = rcon_number_of_players(&client); //ワールド人数の計測
            let tps: f32 = rcon_tps(&client, &args); //tpsの計測
            if (&args[1] == &"io_csv") {
                println!(
                    //一旦標準出力にcsv出力する。
                    "{},{},{}",
                    Local::now().format("%Y%m%d%H%M%S"),
                    number_of_players,
                    tps
                )
            } else if (&args[1] == &"human") {
                println!(
                    "現在オンラインのプレイヤーは{}人、tpsは{}です。",
                    number_of_players, tps
                )
            } else {
                println!("引数を指定してください。io_csv, human");
            }
        }
        Err(_) => {
            println!("Minecraftサーバーにログイン出来ません。\nアプリケーションを終了します。")
        }
    }
}
