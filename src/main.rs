use std::error::Error;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use models::ChampionData;
use serde_json::Value;
use shaco::rest::RESTClient;

mod models;

struct PlayerInfo {
    champion_name: String,
    team: u64,
}

struct PlayerState {
    level: u64,
    total_gold: u64,
    minions_killed: u64,
    jungle_minions_killed: u64,
    position: (f64, f64),
}

fn create_file_writer(game_id: u64) -> Result<BufWriter<File>, Box<dyn Error>> {
    let dir_path = Path::new("games");
    let file_name = format!("game_{}_history.txt", game_id);
    let file_path = dir_path.join(file_name);

    // Create the "games" directory if it doesn't exist
    fs::create_dir_all(dir_path)?;

    let file = File::create(file_path)?;
    Ok(BufWriter::new(file))
}

async fn get_game_history(client: &RESTClient, game_id: u64) -> Result<(), Box<dyn Error>> {
    let history_endpoint = format!("/lol-match-history/v1/games/{}", game_id);
    let history_response = client.get(history_endpoint).await?;
    let game_info: Value = serde_json::from_value(history_response.clone())?;

    let mut writer = create_file_writer(game_id)?;
    writeln!(writer, "Game Timeline:\n")?;

    let timeline_endpoint = format!("/lol-match-history/v1/game-timelines/{}", game_id);
    let timeline_response = client.get(timeline_endpoint).await?;
    let timeline_data: Value = serde_json::from_value(timeline_response)?;
    
    let champion_data = ChampionData::new();
    let player_info = create_player_info(&game_info, &champion_data)?;
    
    process_timeline(&timeline_data, &player_info, &mut writer)?;

    writer.flush()?;
    Ok(())
}

fn create_player_info(game_info: &Value, champion_data: &ChampionData) -> Result<HashMap<u64, PlayerInfo>, Box<dyn Error>> {
    let mut player_info = HashMap::new();
    
    if let Some(participants) = game_info["participants"].as_array() {
        for participant in participants {
            let participant_id = participant["participantId"].as_u64().unwrap_or(0);
            let champion_id = participant["championId"].as_u64().unwrap_or(0);
            let champion_name = champion_data.get_name(champion_id);
            let team = participant["teamId"].as_u64().unwrap_or(0);

            player_info.insert(participant_id, PlayerInfo { champion_name, team });
        }
    }

    Ok(player_info)
}

fn process_timeline(timeline_data: &Value, player_info: &HashMap<u64, PlayerInfo>, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut previous_states: HashMap<u64, PlayerState> = HashMap::new();

    if let Some(frames) = timeline_data["frames"].as_array() {
        for (index, frame) in frames.iter().enumerate() {
            let timestamp = frame["timestamp"].as_u64().unwrap_or(0);
            writeln!(writer, "## {} Minute Mark ({} ms):", timestamp / 60000, timestamp)?;

            process_player_states(frame, player_info, &mut previous_states, writer)?;
            process_events(frame, player_info, writer)?;

            if index < frames.len() - 1 {
                writeln!(writer)?;
            }
        }
    }

    print_map_legend(writer)?;
    Ok(())
}

fn process_player_states(frame: &Value, player_info: &HashMap<u64, PlayerInfo>, previous_states: &mut HashMap<u64, PlayerState>, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    if let Some(participant_frames) = frame["participantFrames"].as_object() {
        for (id, data) in participant_frames {
            let player_id = id.parse::<u64>().unwrap();
            let player = player_info.get(&player_id).unwrap();
            
            let current_state = PlayerState {
                level: data["level"].as_u64().unwrap_or(0),
                total_gold: data["totalGold"].as_u64().unwrap_or(0),
                minions_killed: data["minionsKilled"].as_u64().unwrap_or(0),
                jungle_minions_killed: data["jungleMinionsKilled"].as_u64().unwrap_or(0),
                position: (
                    data["position"]["x"].as_f64().unwrap_or(0.0),
                    data["position"]["y"].as_f64().unwrap_or(0.0),
                ),
            };

            if let Some(previous_state) = previous_states.get(&player_id) {
                print_player_changes(&player.champion_name, team_to_color(player.team), &current_state, previous_state, writer)?;
            } else {
                writeln!(writer, "- {} (Team {}): Level {}, Total Gold {}, Position ({:.0}, {:.0})",
                    player.champion_name, team_to_color(player.team), current_state.level, current_state.total_gold,
                    current_state.position.0, current_state.position.1)?;
            }

            previous_states.insert(player_id, current_state);
        }
    }
    Ok(())
}

fn print_player_changes(champion_name: &str, team: &str, current: &PlayerState, previous: &PlayerState, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut changes = Vec::new();

    if current.level > previous.level {
        changes.push(format!("leveled up to {}", current.level));
    }

    let gold_change = current.total_gold as i64 - previous.total_gold as i64;
    if gold_change.abs() > 100 {
        changes.push(format!("gold changed by {}", gold_change));
    }

    let minion_change = current.minions_killed as i64 - previous.minions_killed as i64;
    if minion_change > 0 {
        changes.push(format!("killed {} minions", minion_change));
    }

    let jungle_change = current.jungle_minions_killed as i64 - previous.jungle_minions_killed as i64;
    if jungle_change > 0 {
        changes.push(format!("killed {} jungle monsters", jungle_change));
    }

    let distance_moved = ((current.position.0 - previous.position.0).powi(2) +
                          (current.position.1 - previous.position.1).powi(2)).sqrt();
    if distance_moved > 1000.0 {
        changes.push(format!("moved to ({:.0}, {:.0})", current.position.0, current.position.1));
    }

    if !changes.is_empty() {
        writeln!(writer, "- {} (Team {}): {}", champion_name, team, changes.join(", "))?;
    }
    Ok(())
}

fn process_events(frame: &Value, player_info: &HashMap<u64, PlayerInfo>, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    if let Some(events) = frame["events"].as_array() {
        for event in events {
            let event_type = event["type"].as_str().unwrap_or("Unknown");
            let timestamp = event["timestamp"].as_u64().unwrap_or(0);
            match event_type {
                "CHAMPION_KILL" => print_champion_kill(event, player_info, timestamp, writer)?,
                "ELITE_MONSTER_KILL" => print_monster_kill(event, player_info, timestamp, writer)?,
                "BUILDING_KILL" => print_building_kill(event, player_info, timestamp, writer)?,
                _ => writeln!(writer, "  - {} ms: Unprocessed event type: {}", timestamp, event_type)?,
            }
        }
    }
    Ok(())
}

fn print_champion_kill(event: &Value, player_info: &HashMap<u64, PlayerInfo>, timestamp: u64, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let killer_id = event["killerId"].as_u64().unwrap_or(0);
    let victim_id = event["victimId"].as_u64().unwrap_or(0);
    let killer = player_info.get(&killer_id).map_or("Unknown".to_string(), |p| p.champion_name.clone());
    let victim = player_info.get(&victim_id).map_or("Unknown".to_string(), |p| p.champion_name.clone());
    let assistants = get_assistants(event, player_info);
    let position = get_position(event);
    let time = ms_to_min_sec(timestamp);
    
    writeln!(writer, "  - {}: {} killed {} {} at position {}.", time, killer, victim, assistants, position)?;
    Ok(())
}

fn print_monster_kill(event: &Value, player_info: &HashMap<u64, PlayerInfo>, timestamp: u64, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let killer_id = event["killerId"].as_u64().unwrap_or(0);
    let killer = player_info.get(&killer_id).map_or("Unknown".to_string(), |p| p.champion_name.clone());
    let monster_type = event["monsterType"].as_str().unwrap_or("Unknown");
    let monster_subtype = event["monsterSubType"].as_str().unwrap_or("");
    let position = get_position(event);
    let time = ms_to_min_sec(timestamp);
    
    let formatted_monster = match (monster_type, monster_subtype) {
        ("HORDE", _) => "VOID_GRUB".to_string(),
        (_, "") => monster_type.to_string(),
        (_, subtype) => format!("{} {}", subtype, monster_type),
    };
    
    writeln!(writer, "  - {}: {} killed {} at position {}.", time, killer, formatted_monster, position)?;
    Ok(())
}

fn print_building_kill(event: &Value, player_info: &HashMap<u64, PlayerInfo>, timestamp: u64, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let killer_id = event["killerId"].as_u64().unwrap_or(0);
    let killer = player_info.get(&killer_id).map_or("Unknown".to_string(), |p| p.champion_name.clone());
    let building_type = event["buildingType"].as_str().unwrap_or("Unknown");
    let lane = event["laneType"].as_str().unwrap_or("Unknown");
    let team = if event["teamId"].as_u64().unwrap_or(0) == 100 { "Blue" } else { "Red" };
    let position = get_position(event);
    let time = ms_to_min_sec(timestamp);
    
    writeln!(writer, "  - {}: {} destroyed {} team's {} in {} lane at position {}.", time, killer, team, building_type, lane, position)?;
    Ok(())
}

fn get_assistants(event: &Value, player_info: &HashMap<u64, PlayerInfo>) -> String {
    if let Some(assistants) = event["assistingParticipantIds"].as_array() {
        if !assistants.is_empty() {
            let assistant_names: Vec<String> = assistants
                .iter()
                .filter_map(|id| id.as_u64())
                .filter_map(|id| player_info.get(&id))
                .map(|p| p.champion_name.clone())
                .collect();
            format!("(assisted by {})", assistant_names.join(", "))
        } else {
            String::from("(unassisted)")
        }
    } else {
        String::from("(unassisted)")
    }
}

fn team_to_color(team: u64) -> &'static str {
    match team {
        100 => "Blue",
        200 => "Red",
        _ => "Unknown",
    }
}

fn get_position(event: &Value) -> String {
    let x = event["position"]["x"].as_f64().unwrap_or(0.0);
    let y = event["position"]["y"].as_f64().unwrap_or(0.0);
    format!("({:.0}, {:.0})", x, y)
}

fn ms_to_min_sec(ms: u64) -> String {
    let total_seconds = ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

fn print_map_legend(writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "\n## Key for reading coordinates:")?;
    writeln!(writer, "- The Summoner's Rift map is 16000 x 16000 units")?;
    writeln!(writer, "- (0, 0) is the bottom-left corner of the map")?;
    writeln!(writer, "- (16000, 16000) is the top-right corner of the map")?;
    writeln!(writer, "- Blue team's base is in the bottom-left quadrant")?;
    writeln!(writer, "- Red team's base is in the top-right quadrant")?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lcu_rest_client = RESTClient::new()?;
    
    // Hardcoded game_id (replace with an actual game_id)
    let game_id = 5047559968;
    
    get_game_history(&lcu_rest_client, game_id).await?;
    
    Ok(())
}