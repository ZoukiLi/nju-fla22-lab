use trm_sim::trm::Machine;

fn main() {
    println!("Hello, world!");

    // print size of Vec
    let v = vec![1; 10];
    println!("size of v: {}", std::mem::size_of_val(v.as_slice()));
    println!("size of Vec: {}", std::mem::size_of::<Vec<i32>>());

    let model = r#"
[[state]]
name = 'q0'
is_start = true
is_final = false

[[state.trans]]
cons = '0'
prod = '1'
move = 'R'
next = 'q1'

[[state.trans]]
cons = '1'
prod = '0'
move = 'R'
next = 'q1'

[[state]]
name = 'q1'
is_start = false
is_final = true

[[state.trans]]
cons = '0'
prod = '1'
move = 'R'
next = 'q1'

[[state.trans]]
cons = '1'
prod = '0'
move = 'R'
next = 'q1'
    "#;
    let mut machine = Machine::new(model, "toml").expect("machine init error");
    machine.input("1101");
    loop {
        match machine.run().expect("machine running error") {
            true => break,
            false => {
                println!("{}", serde_yaml::to_string(&machine.identifier()).unwrap());
            }
        }
    }

    match machine.is_final() {
        true => println!("final"),
        false => println!("not final"),
    }

    let model = machine.model();
    let yaml_str = serde_yaml::to_string(&model).unwrap();
    let tom_str = toml::to_string_pretty(&model).unwrap();
    let json_str = serde_json::to_string_pretty(&model).unwrap();
    println!("{yaml_str}");
    println!("{tom_str}");
    println!("{json_str}");
}
