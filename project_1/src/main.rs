const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let fav_season: &str = "Winter";

    let mut points_scored: i32 = 28;

    points_scored = 5;

    //Used _ to silence
    let _event_time: &str = "06:00";

    let event_time: i32 = 6;

    //Used compile directive to silence
    #[allow(unused_variables)]
    let fav_bev: &str = "Shirley Temple";

    println!(
        "Const Value: {TOUCHDOWN_POINTS}, Favorite Season: {fav_season}, Points Scored: {points_scored}, Event Time (Current): {event_time}"
    )
}
