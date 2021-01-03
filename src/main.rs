use ureq;
// use serde::{Deserialize};
// use std::{fs, error,
//     io::{
//     BufReader, Read,
// }};

fn main() {

    let apikey = football_one::config::get_config();
    hureq(apikey);
}

fn hureq(apikey: String) {

    let mut req = get_team("290");
    // let mut req = get_team_transfers("290");
    // let mut req = get_leagues();
    // let mut req = get_standings("98", "2020");

    req = api_header_set(apikey,req);

    let resp = req.call();
    let hoge = resp.into_string();

    match hoge {
        Ok(s) => println!("{}", s),
        _ => println!("ERR"),
    };
}

fn get_team(team_id: &str) -> ureq::Request {
    let query = format!("?id={}", team_id);
    ureq::get("https://v3.football.api-sports.io/teams")
        .query_str(&query)
        .build()
    
    // api_header_set(req)
}

fn get_leagues() -> ureq::Request {
    ureq::get("https://v3.football.api-sports.io/leagues")
    .build()

    // api_header_set(req)
}

fn get_standings(league_id: &str, year: &str) -> ureq::Request {
    let query = format!("league={}&season={}", league_id, year);
    // println!("{}", query);
    ureq::get("https://v3.football.api-sports.io/teams")
        .query_str(&query)
        .build()
    
    // api_header_set(req)
}

fn get_team_transfers(team_id: &str) -> ureq::Request{
    let query = format!("team={}", team_id);
    ureq::get("https://v3.football.api-sports.io/transfers")
        .query_str(&query)
        .build()
    
    // api_header_set(req)
}

fn api_header_set(apikey : String, mut req: ureq::Request) -> ureq::Request {
    req.set("x-rapidapi-host", "v3.football.api-sports.io")
    .set("x-rapidapi-key", &apikey)
    .build()
}





/*
{"id":102,"name":"Emperor Cup","type":"Cup","logo":"https://media.api-sports.io/football/leagues/102.png"}
{"id":101,"name":"J-League Cup","type":"Cup","logo":"https://media.api-sports.io/football/leagues/101.png"}
{"id":98,"name":"J. League Div.1","type":"League","logo":"https://media.api-sports.io/football/leagues/98.png"}
{"id":99,"name":"J. League Div.2","type":"League","logo":"https://media.api-sports.io/football/leagues/99.png"}
{"id":100,"name":"J. League Div.3","type":"League","logo":"https://media.api-sports.io/football/leagues/100.png"}
{"id":497,"name":"Japan Football League","type":"League","logo":"https://media.api-sports.io/football/leagues/497.png"}
{"id":548,"name":"Super Cup","type":"Cup","logo":"https://media.api-sports.io/football/leagues/548.png"}

{"team":{"id":290,"name":"Kashima","country":"Japan","founded":1991,"national":false,"logo":"https://media.api-sports.io/football/teams/290.png"},
"venue":{"id":964,"name":"Kashima Soccer Stadium","address":"26-2 Ushiroyama, Jinkoji","city":"Kashima","capacity":40728,"surface":"grass","image":"https://media.api-sports.io/football/venues/964.png"}}
*/
