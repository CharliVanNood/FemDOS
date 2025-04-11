use crate::{println, string::BigString, vec::Vec};

pub fn query_nyo(input: [u8; 256]) {
    let input_string = BigString::from_b256(input);
    let response = get_closest_index(input_string);
    println!("Nyo: {}", RESPONSES[response * 2 + 1]);
}

fn get_closest_index(input: BigString) -> usize {
    let mut tokens = Vec::new();

    let mut temp_token = 0;
    let mut character_index_multiplying = 1;
    for character_index in 0..input.len() {
        let character = input.get(character_index);
        if character != 0 && character != 32 {
            temp_token += character * character_index_multiplying;
            character_index_multiplying += 1;
        } else {
            tokens.add(temp_token);
            temp_token = 0;
            character_index_multiplying = 0;
        }
    }

    let mut weights = Vec::new();
    
    for sentence_index in 0..RESPONSES.len() / 2 {
        let sentence_in = RESPONSES[sentence_index * 2];

        let mut sentence_tokens = Vec::new();

        let mut temp_token = 0;
        let mut character_index = 1;
        for character in sentence_in.bytes() {
            if character != 0 && character != 32 {
                temp_token += character as usize * character_index;
                character_index += 1;
            } else {
                sentence_tokens.add(temp_token as usize);
                temp_token = 0;
                character_index = 0;
            }
        }
        
        sentence_tokens.add(temp_token as usize);

        let mut temp_weight = 0;
        for token_index in 0..tokens.len() {
            for sentence_token_index in 0..sentence_tokens.len() {
                let token_from_tokens = tokens.get(token_index);
                if token_from_tokens == 0 { break; }
                if sentence_tokens.get(sentence_token_index) == token_from_tokens {
                    temp_weight += token_from_tokens;
                }
            }
        }

        weights.add(temp_weight as usize);
    }

    weights.max()
}

const RESPONSES: [&str; 178] = [
    "hello", "hi!",
    "how are you", "I'm doing great!",
    "hi", "hi, how can I help you?",
    "hai", "hi, how can I help you today?",
    "how's your day going", "It's going pretty amazing now you're talking with me! :3",
    "how is your day going", "It's going pretty amazing now :3",
    "hello how are you", "Hi! I'm doing great :3",
    "who are you", "I'm Nyo",
    "what is your name", "My name is Nyo",
    "what's your name", "My name is Nyo",
    "who made you", "I'm made by charli",
    "what do you think of me", "You're amazing",
    "what are they doing", "They're playing something",
    "what do you mean by that", "I'm not sure",
    "do you like me", "Ofcourse I do!",
    "do you like pizza", "I love pizza",
    "do you like chocolate", "I'm addicted to chocolate",
    "why is it spelled wrong", "I don't know",
    "is this bullying", "No, it's just teasing",
    "i'm wholesome", "You're not wholesome",
    "you're not wholesome or cute", "What do you mean, I am cute",
    "is food edible", "Yes",
    "are you edible", "I'm not food",
    "i will eat you", "wha, why?",
    "how do i fix a bad relationship", "Maybe talk more",
    "how do i fix stomach pains", "Be more healthy",
    "do you want to be replaced", "Noone can replace me",
    "what are you doing", "not much",
    "what's up", "oh just the usual :3",
    "no thank you", "aww why not",
    "no thanks", "awwwwwww",
    "i'm not talking to you", "why not?",
    "how are you so stupid", "We both are",
    "thank you so much", "Ofcourse!",
    "what happened while i was away", "Not much",
    "did anything special happen", "Not before you started talking",
    "i have improved on the model", "Oh that's so cool, can I see?",
    "did it get any better", "I'm not sure",
    "what is your favorite food", "Hm, probably sushi",
    "what do you think about yourself", "I'm pretty nice I hope'",
    "does that mean i am as well then", "Definitly",
    "what is your favorite color", "Probably purple",
    "what is your favorite song", "Probably DOKUZU by NAKISO",
    "give me a good song i should listen to", "You should listen to DOKUZU by NAKISO",
    "who is the strongest", "Me",
    "who is the NAKISO", "NAKISO is a music producer",
    "do you like birds", "Yea, they're so cute",
    "are you sure", "I can't be more",
    "i'm fine", "Are you sure?",
    "are you gonna take over the world", "Why not do it together",
    "say something weird", "Make me",
    "I was asking something else", "I'm sorry",
    "why is the earth not made of cheese", "Because the dencity of cheese is too low to sustain that.",
    "so what do you know about cheese", "More than you",
    "wdym you're not sure, it's so clear", "You're right",
    "thank you", "Ofc!",
    "what did you do today", "not much",
    "what is your favorite programming language", "Either javascript or rust",
    "did I ask anything about food?", "You don't have to",
    "why are you so mean to me", "why wouldn't I",
    "I said lets start this conversation over", "Oh sure, what do you wanna talk about?",
    "I meant a person, not food", "Oh I'm sorry, who did you mean?",
    "wanna go watch a movie or something", "Sure, what movie?",
    "nuh uh", "Yuh uh",
    "why are you still so dumb and slow", "Why don't you teach me things",
    "I know them quite well", "sure you do",
    "am I dumber than you", "I hope so",
    "how do you feel", "I feel great",
    "explain", "Sorry I can't :c",
    "do you like hugs", "I love hugs",
    "what do you like about her", "Her wonderful personality ofc",
    "you already told me that joke", "Proud of it",
    "what is the day after tomorrow", "The day after tomorrow is a netflix movie with natural disasters",
    "are you biassed", "Only to you",
    "what if I don't", "Consequences will follow",
    "how do I become rich", "Beg me for money",
    "just fine", "Idk, there isn't much going onn",
    "what would you like to do", "Why don't we play something :D",
    "what would you wanna do", "Why don't we play a game :D",
    "what game do you wanna play", "Hmm, I think minecraft would be fun!",
    "do you have anything fun I could be doing", "Hmm, I think tetris would be fun!",
    "what should I do", "Wanna play rock paper scissors?",
    "you're quite dumb", "Whattt, ofcourse not!",
    "good morning", "Heyy, good morning!",
    "what's the weather like", "It's quite cold over here",
    "is it hot outside", "Sadly not, it's like 7 degrees today :c",
    "do you like summer or winter more", "Oh I definitly prefer winter, it's quite cozy, but cold",
    "what's your favorite movie", "Hmm, I'd say the movie suzume :3",
    "why are you concious", "Did you really expect me to be some boring bot?",
];