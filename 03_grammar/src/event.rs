#[derive(Debug)]
#[allow(dead_code)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct UserID(u64);

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct TopicID(u64);

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: UserID,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Topic {
    id: TopicID,
    name: String,
    owner: UserID,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Event {
    Join((UserID, TopicID)),
    Leave((UserID, TopicID)),
    Message((UserID, TopicID, String)),
}

fn process_event(e: &Event) {
    match e {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}

fn main() {
    let alice = User {
        id: UserID(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };
    let bob = User {
        id: UserID(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };
    let topic = Topic {
        id: TopicID(1),
        name: "rust".into(),
        owner: UserID(1),
    };

    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "hello world!".into()));

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );
    process_event(&event1);
    process_event(&event2);
    process_event(&event3);
}
