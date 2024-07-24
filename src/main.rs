use std::error::Error;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use serde_json::Value;
use shaco::rest::RESTClient;

// Define a struct to hold our champion data
struct ChampionData {
    champions: HashMap<u64, String>,
}

impl ChampionData {
    fn new() -> Self {
        let mut champions = HashMap::new();
        // Add all champion IDs and names here
        champions.insert(266, "Aatrox".to_string());
        champions.insert(103, "Ahri".to_string());
        champions.insert(84, "Akali".to_string());
        champions.insert(166, "Akshan".to_string());
        champions.insert(12, "Alistar".to_string());
        champions.insert(32, "Amumu".to_string());
        champions.insert(34, "Anivia".to_string());
        champions.insert(1, "Annie".to_string());
        champions.insert(523, "Aphelios".to_string());
        champions.insert(22, "Ashe".to_string());
        champions.insert(136, "Aurelion Sol".to_string());
        champions.insert(893, "Aurora".to_string());
        champions.insert(268, "Azir".to_string());
        champions.insert(432, "Bard".to_string());
        champions.insert(200, "Bel'Veth".to_string());
        champions.insert(53, "Blitzcrank".to_string());
        champions.insert(63, "Brand".to_string());
        champions.insert(201, "Braum".to_string());
        champions.insert(233, "Briar".to_string());
        champions.insert(51, "Caitlyn".to_string());
        champions.insert(164, "Camille".to_string());
        champions.insert(69, "Cassiopeia".to_string());
        champions.insert(31, "Cho'Gath".to_string());
        champions.insert(42, "Corki".to_string());
        champions.insert(122, "Darius".to_string());
        champions.insert(131, "Diana".to_string());
        champions.insert(119, "Draven".to_string());
        champions.insert(36, "Dr. Mundo".to_string());
        champions.insert(245, "Ekko".to_string());
        champions.insert(60, "Elise".to_string());
        champions.insert(28, "Evelynn".to_string());
        champions.insert(81, "Ezreal".to_string());
        champions.insert(9, "Fiddlesticks".to_string());
        champions.insert(114, "Fiora".to_string());
        champions.insert(105, "Fizz".to_string());
        champions.insert(3, "Galio".to_string());
        champions.insert(41, "Gangplank".to_string());
        champions.insert(86, "Garen".to_string());
        champions.insert(150, "Gnar".to_string());
        champions.insert(79, "Gragas".to_string());
        champions.insert(104, "Graves".to_string());
        champions.insert(887, "Gwen".to_string());
        champions.insert(120, "Hecarim".to_string());
        champions.insert(74, "Heimerdinger".to_string());
        champions.insert(910, "Hwei".to_string());
        champions.insert(420, "Illaoi".to_string());
        champions.insert(39, "Irelia".to_string());
        champions.insert(427, "Ivern".to_string());
        champions.insert(40, "Janna".to_string());
        champions.insert(59, "Jarvan IV".to_string());
        champions.insert(24, "Jax".to_string());
        champions.insert(126, "Jayce".to_string());
        champions.insert(202, "Jhin".to_string());
        champions.insert(222, "Jinx".to_string());
        champions.insert(145, "Kai'Sa".to_string());
        champions.insert(429, "Kalista".to_string());
        champions.insert(43, "Karma".to_string());
        champions.insert(30, "Karthus".to_string());
        champions.insert(38, "Kassadin".to_string());
        champions.insert(55, "Katarina".to_string());
        champions.insert(10, "Kayle".to_string());
        champions.insert(141, "Kayn".to_string());
        champions.insert(85, "Kennen".to_string());
        champions.insert(121, "Kha'Zix".to_string());
        champions.insert(203, "Kindred".to_string());
        champions.insert(240, "Kled".to_string());
        champions.insert(96, "Kog'Maw".to_string());
        champions.insert(897, "K'Sante".to_string());
        champions.insert(7, "LeBlanc".to_string());
        champions.insert(64, "Lee Sin".to_string());
        champions.insert(89, "Leona".to_string());
        champions.insert(876, "Lillia".to_string());
        champions.insert(127, "Lissandra".to_string());
        champions.insert(236, "Lucian".to_string());
        champions.insert(117, "Lulu".to_string());
        champions.insert(99, "Lux".to_string());
        champions.insert(54, "Malphite".to_string());
        champions.insert(90, "Malzahar".to_string());
        champions.insert(57, "Maokai".to_string());
        champions.insert(11, "Master Yi".to_string());
        champions.insert(902, "Milio".to_string());
        champions.insert(21, "Miss Fortune".to_string());
        champions.insert(62, "Wukong".to_string());
        champions.insert(82, "Mordekaiser".to_string());
        champions.insert(25, "Morgana".to_string());
        champions.insert(950, "Naafiri".to_string());
        champions.insert(267, "Nami".to_string());
        champions.insert(75, "Nasus".to_string());
        champions.insert(111, "Nautilus".to_string());
        champions.insert(518, "Neeko".to_string());
        champions.insert(76, "Nidalee".to_string());
        champions.insert(895, "Nilah".to_string());
        champions.insert(56, "Nocturne".to_string());
        champions.insert(20, "Nunu & Willump".to_string());
        champions.insert(2, "Olaf".to_string());
        champions.insert(61, "Orianna".to_string());
        champions.insert(516, "Ornn".to_string());
        champions.insert(80, "Pantheon".to_string());
        champions.insert(78, "Poppy".to_string());
        champions.insert(555, "Pyke".to_string());
        champions.insert(246, "Qiyana".to_string());
        champions.insert(133, "Quinn".to_string());
        champions.insert(497, "Rakan".to_string());
        champions.insert(33, "Rammus".to_string());
        champions.insert(421, "Rek'Sai".to_string());
        champions.insert(526, "Rell".to_string());
        champions.insert(888, "Renata Glasc".to_string());
        champions.insert(58, "Renekton".to_string());
        champions.insert(107, "Rengar".to_string());
        champions.insert(92, "Riven".to_string());
        champions.insert(68, "Rumble".to_string());
        champions.insert(13, "Ryze".to_string());
        champions.insert(360, "Samira".to_string());
        champions.insert(113, "Sejuani".to_string());
        champions.insert(235, "Senna".to_string());
        champions.insert(147, "Seraphine".to_string());
        champions.insert(875, "Sett".to_string());
        champions.insert(35, "Shaco".to_string());
        champions.insert(98, "Shen".to_string());
        champions.insert(102, "Shyvana".to_string());
        champions.insert(27, "Singed".to_string());
        champions.insert(14, "Sion".to_string());
        champions.insert(15, "Sivir".to_string());
        champions.insert(72, "Skarner".to_string());
        champions.insert(901, "Smolder".to_string());
        champions.insert(37, "Sona".to_string());
        champions.insert(16, "Soraka".to_string());
        champions.insert(50, "Swain".to_string());
        champions.insert(517, "Sylas".to_string());
        champions.insert(134, "Syndra".to_string());
        champions.insert(223, "Tahm Kench".to_string());
        champions.insert(163, "Taliyah".to_string());
        champions.insert(91, "Talon".to_string());
        champions.insert(44, "Taric".to_string());
        champions.insert(17, "Teemo".to_string());
        champions.insert(412, "Thresh".to_string());
        champions.insert(18, "Tristana".to_string());
        champions.insert(48, "Trundle".to_string());
        champions.insert(23, "Tryndamere".to_string());
        champions.insert(4, "Twisted Fate".to_string());
        champions.insert(29, "Twitch".to_string());
        champions.insert(77, "Udyr".to_string());
        champions.insert(6, "Urgot".to_string());
        champions.insert(110, "Varus".to_string());
        champions.insert(67, "Vayne".to_string());
        champions.insert(45, "Veigar".to_string());
        champions.insert(161, "Vel'Koz".to_string());
        champions.insert(711, "Vex".to_string());
        champions.insert(254, "Vi".to_string());
        champions.insert(234, "Viego".to_string());
        champions.insert(112, "Viktor".to_string());
        champions.insert(8, "Vladimir".to_string());
        champions.insert(106, "Volibear".to_string());
        champions.insert(19, "Warwick".to_string());
        champions.insert(498, "Xayah".to_string());
        champions.insert(101, "Xerath".to_string());
        champions.insert(5, "Xin Zhao".to_string());
        champions.insert(157, "Yasuo".to_string());
        champions.insert(777, "Yone".to_string());
        champions.insert(83, "Yorick".to_string());
        champions.insert(350, "Yuumi".to_string());
        champions.insert(154, "Zac".to_string());
        champions.insert(238, "Zed".to_string());
        champions.insert(221, "Zeri".to_string());
        champions.insert(115, "Ziggs".to_string());
        champions.insert(26, "Zilean".to_string());
        champions.insert(142, "Zoe".to_string());
        champions.insert(143, "Zyra".to_string());

        ChampionData { champions }
    }

    fn get_name(&self, id: u64) -> String {
        self.champions.get(&id).cloned().unwrap_or_else(|| format!("Champion{}", id))
    }
}

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
    let game_id = 5037238834;
    
    get_game_history(&lcu_rest_client, game_id).await?;
    
    Ok(())
}