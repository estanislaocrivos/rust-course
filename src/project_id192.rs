/* This project corresponds to task number 192 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

/*
Let's model a real-time chat system where users can
share audio and video files.

Define a DigitalContent enum with two variants:
AudioFile and VideoFile. Derive a Debug implementation.

Define a ChatMessage struct with two fields: `content`
and `time`. The struct should define one generic type, T,
which will be the type of the `content` field.
The `time` field should always be a String.
Derive a Debug implementation.

Add an impl block for ChatMessage structs whose T type
is a DigitalContent enum. Define a `consume_entertainment`
method that prints out the value of the `content` field in
Debug format. For example, "Watching the AudioFile".

Add an impl block for ChatMessage structs with any type T.
Define a `retrieve_time` method that returns a String.
It should return a clone of the `time` field from
the struct.

In `main`, create a ChatMessage with `content` set to a
string slice.

Create another ChatMessage with `content` set to a String.

Create another ChatMessage with `content' set to a
DigitalContent variant.

Invoke the `consume_entertainment` method on the
ChatMessage storing a DigitalContent enum.

Invoke the `retrieve_time` method on all 3 ChatMessage
instances and print out each String's content.
*/

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Content: {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        return self.time.clone();
    }
}

pub fn project_id192_solution() {
    let chat_1 = ChatMessage {
        content: "Hello!",
        time: "19:23:00".to_string(),
    };
    let chat_2 = ChatMessage {
        content: "Hello!".to_string(),
        time: "19:23:00".to_string(),
    };
    let chat_3 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: "19:23:00".to_string(),
    };

    println!("Chat timestamp: {}", chat_1.retrieve_time());
    println!("Chat timestamp: {}", chat_2.retrieve_time());
    println!("Chat timestamp: {}", chat_3.retrieve_time());
    chat_3.consume_entertainment(); // Can only call consume_entertainment on chat_3 due to having its content field of type DigitalContent
}
