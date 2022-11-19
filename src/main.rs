use std::fs;
use std::env;
use std::fmt;

use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    due_in_n_days: u32,
}



#[derive(Debug)]
#[derive(PartialEq)]
enum CommandType {
    Add,
    GetAll,
    GetUrgent,
}
#[derive(Debug)]
struct Command {
    ctype: CommandType,
    arg: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} à faire dans {}", self.name, self.due_in_n_days)
    }
}


fn usage() {
    println!("Wilkommen to another TODO cli app");
    println!("-u, --urgent: display task to do before next week");
    println!("-a --add [name] [due_date]: add a new task with its date");
    panic!("Usage");
}

fn filter_urgent(task_list: Vec<Task>, days: u32) {

    for t in &task_list {
        if t.due_in_n_days < days{
            println!("La tâche {} est à faire avant {} jours ",t.name, t.due_in_n_days);
        }
    }
}

fn main() {
    let mut task_list: Vec<Task> =load_data();

    let args: Vec<String> = env::args().collect();
    let command: Command = parse_command(&args);

    match command.ctype {
        CommandType::Add => {

            let split_arg = command.arg.split('|');
            let vec_split: Vec<&str> = split_arg.collect();

            let name: &str = vec_split.get(0).unwrap();
            let days_s: &str = vec_split.get(1).unwrap();
            
            let days: u32 = days_s.parse::<u32>().expect("Usage add: -a [name] [days<u32>]");

            let new_task: Task = Task { name: String::from(name), due_in_n_days: days };
            task_list.push(new_task);

            save_data(&task_list);


            for t in task_list.iter() {
                println!("{}", t);
            }
        },
        CommandType::GetAll => {

            for t in task_list.iter() {
                println!("{}", t);
            }
        },
        CommandType::GetUrgent => {
            let days = command.arg.parse::<u32>().expect("Usage urgent: -u [days] (integer)");
            filter_urgent(task_list, days);
        }
    }

  
    //tasklist.push(

 //   println!("Test {:?}", t1);

 


//    println!("Test {:?}", task_list);


}


fn parse_command(args: &[String]) -> Command {

    let query = args.get(1);

    let mut query_command: CommandType = CommandType::GetAll;
    let mut query_body: String = String::from("");
    match query {
        Some(x) => {
            match x.as_str() {
                "-a"|"--add" => query_command = CommandType::Add,
                "-u"|"--urgent" => query_command = CommandType::GetUrgent,
                "-h" => usage(),
                "-l"|"--list"|_ => query_command = CommandType::GetAll,
            }
        }
        None =>{
            usage();
            panic!("ARGUMENTE ")
        } ,
    }

    if query_command == CommandType::GetUrgent {
        let arg = args.get(2);

        match arg {
            Some(x) => query_body = x.to_string(),
            None => {
                usage();
                panic!("ARGUMENTE le body du ")
            }
        }
    } else if query_command == CommandType::Add {
        let name = args.get(2).unwrap();
        let due_days = args.get(3).unwrap();


        query_body = name.to_owned() + &String::from("|") + due_days;
        
    }
    
    /*match query.as_str() {
        "-a"|"-add" => println!("add"),
        "-u"|"-urgent" => println!("urgetn"),
        _ => println!("usage???"),
    }*/

    Command {
        ctype: query_command,
        arg: query_body,
    }
}

fn load_data() -> Vec<Task> {


    let content = fs::read_to_string("./data")
        .expect("Can't open the data file :(");


    serde_json::from_str::<Vec<Task>>(content.as_str()).unwrap()
}

fn save_data(data: &Vec<Task>) {

    let content = serde_json::to_string(data).unwrap();

    fs::write("./data", content).expect("Can't save data");
}


