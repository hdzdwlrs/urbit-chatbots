use urbit_chatbot_framework::{AuthoredMessage, Chatbot, Message};
use std::fs;
use rand::random;
use json;

// poll file structure
// pollname yea nay 3 4
// [vote counts for each poll option specified]

fn respond_to_message(authored_message: AuthoredMessage) -> Option<Message> {
    // Split the message up into words (split on whitespace)
    let words = authored_message.message.to_formatted_words();
    // let voter = authored_message.author; use later....

    // If the first word is the command `!poll`, we initialize a new poll.
    if words[0] == "!poll" {
        let opts = &words[1..];
        let mut msg = "Poll started with options: ".to_string();
        let pollid = random::<u8>();
        let mut pollfile = json::JsonValue::new_object();
        for i in 0..opts.len() {
            pollfile[&opts[i]] = 0.into();
            //pollfile.push_str(&json::stringify(data));
            if i == 0 {
                msg.push_str(&opts[i]);
            }
            else {
                msg.push_str(&(" ".to_owned() + &opts[i]));
            }
        }

        msg.push_str(&format!(". Type \"!vote {} [option]\" to participate.", pollid.to_string()));
        
        fs::write(format!("polls/{}.json", pollid), json::stringify(pollfile)).expect("error writing poll file");
        
        return Some(Message::new().add_text(&msg));
    }

    else if words[0] == "!vote" {
        if words.len() != 3 {
            return Some(Message::new().add_text("Invalid vote. Format: \"!vote [poll id] [option]\""));
        }
        // read poll file of chosen poll
        let polldata = fs::read_to_string(format!("polls/{}.json", words[1])).expect("error reading poll file");
        // TODO make sure voter hasn't voted already
        // parse vote data and find option to increment
        let mut parsed = json::parse(&polldata).unwrap();
        let curr_count = json::stringify(parsed[&words[2]].clone());
        let new_count = curr_count.parse::<u32>().unwrap() + 1;
        // modify json object to contain new vote
        parsed[&words[2]] = new_count.into();
        // write new result to file
        let result = json::stringify(parsed);
        fs::write(format!("polls/{}.json", words[1]), &result).expect("error writing poll file");
        
        let text = format!("vote for {} counted. current results: {}", words[2], result);
        return Some(Message::new().add_text(&text));
    }

    else if words[0] == "!endpoll" {
        if words.len() != 2 {
            return Some(Message::new().add_text("Invalid poll command"));
        }

        let result = fs::read_to_string(format!("polls/{}.json", words[1])).expect("error reading poll file");
        
        let text = format!("Poll ID {} has ended. Results: {}", words[1], result);
        return Some(Message::new().add_text(&text));
    }

    // Otherwise do not respond to message
    None
}


fn main() {
    let chat_bot = Chatbot::new_with_local_config(respond_to_message, "~bacrys", "testchat-30");

    chat_bot.run();
}
