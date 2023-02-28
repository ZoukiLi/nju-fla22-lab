//! Test usage of serde crate

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Weak;
use std::vec;

/// simple struct to test usage of serde crate
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Point {
    name: String,
    point: (f64, f64),
}

/// a little test for state
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct State {
    name: String,

    #[serde(default)]
    is_start: bool,

    #[serde(default)]
    is_final: bool,

    #[serde(default)]
    transitions: Vec<Transition>,
}

/// transition struct for state
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transition {
    #[serde(skip)]
    #[allow(dead_code, unused_variables)]
    next_state: Option<Weak<State>>,

    #[serde(rename = "next")]
    next_state_name: String,
    #[serde(rename = "cons")]
    consume: String,
    #[serde(rename = "prod")]
    produce: String,
    #[serde(
        rename = "move",
        deserialize_with = "deserialize_moves",
        serialize_with = "serialize_moves"
    )]
    move_direction: Vec<MoveDirection>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum MoveDirection {
    Left,
    Right,
    Stay,
}

fn deserialize_moves<'de, D>(deserializer: D) -> Result<Vec<MoveDirection>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    s.chars()
        .map(|c| match c {
            'L' => Ok(MoveDirection::Left),
            'R' => Ok(MoveDirection::Right),
            'S' => Ok(MoveDirection::Stay),
            _ => Err(serde::de::Error::custom(format!(
                "invalid move direction: {}",
                c
            ))),
        })
        .collect()
}

fn serialize_moves<S>(moves: &[MoveDirection], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s: String = moves
        .iter()
        .map(|m| match m {
            MoveDirection::Left => 'L',
            MoveDirection::Right => 'R',
            MoveDirection::Stay => 'S',
        })
        .collect();
    serializer.serialize_str(&s)
}

/// simple test usage of serde crate
#[test]
fn test_serde_point() {
    let point_array = vec![
        Point {
            name: String::from("A"),
            point: (1.0, 2.0),
        },
        Point {
            name: String::from("B"),
            point: (3.0, 4.0),
        },
    ];
    let mut point_map = HashMap::new();
    point_map.insert(String::from("points"), point_array);

    let se_toml = toml::to_string(&point_map).unwrap();
    println!("serialized: \n{}", se_toml);

    let se_json = serde_json::to_string(&point_map).unwrap();
    println!("serialized: \n{}", se_json);
}

/// test usage of serde crate with default
#[test]
fn test_ser_state() {
    let state = State {
        name: String::from("A"),
        is_start: true,
        is_final: false,
        transitions: vec![
            Transition {
                next_state: None,
                next_state_name: String::from("B"),
                consume: String::from("0"),
                produce: String::from("1"),
                move_direction: vec![MoveDirection::Right],
            },
            Transition {
                next_state: None,
                next_state_name: String::from("C"),
                consume: String::from("1"),
                produce: String::from("0"),
                move_direction: vec![MoveDirection::Left],
            },
        ],
    };

    let se_toml = toml::to_string(&state).unwrap();
    println!("toml: \n{}", se_toml);

    let se_json = serde_json::to_string(&state).unwrap();
    println!("json: \n{}", se_json);
}

/// test usage of serde crate with default
/// and array of table deserialize
#[test]
fn test_de_states() {
    const FILE_PATH: &str = "../turing-programs/trivial_trm.toml";
    let file_content = std::fs::read_to_string(FILE_PATH).unwrap_or_else(|_| {
        panic!(
            "pwd: {}",
            std::env::current_dir().unwrap().to_str().unwrap()
        )
    });
    let de_states: HashMap<String, Vec<State>> = toml::from_str(&file_content).unwrap();
    println!("de_states: {:#?}", de_states);

    let se_states = toml::to_string_pretty(&de_states).unwrap();
    println!("se_states: \n{}", se_states);

    let se_json = serde_json::to_string_pretty(&de_states).unwrap();
    println!("json: \n{}", se_json);
}

/// array of table deserialize
#[derive(Serialize, Deserialize, Debug, Clone)]
struct TuringMachine {
    state: Vec<State>,
}

/// test usage of ser a turing machine
#[test]
fn test_ser_turing_machine() {
    let state1 = State {
        name: String::from("A"),
        is_start: true,
        is_final: false,
        transitions: vec![
            Transition {
                next_state: None,
                next_state_name: String::from("B"),
                consume: String::from("0"),
                produce: String::from("1"),
                move_direction: vec![MoveDirection::Right],
            },
            Transition {
                next_state: None,
                next_state_name: String::from("C"),
                consume: String::from("1"),
                produce: String::from("0"),
                move_direction: vec![MoveDirection::Left],
            },
        ],
    };

    let state2 = State {
        name: String::from("B"),
        is_final: true,
        ..Default::default()
    };

    let tm = TuringMachine {
        state: vec![state1, state2],
    };

    let se_toml = toml::to_string(&tm).unwrap();
    println!("toml: \n{}", se_toml);

    let se_json = serde_json::to_string(&tm).unwrap();
    println!("json: \n{}", se_json);
}
