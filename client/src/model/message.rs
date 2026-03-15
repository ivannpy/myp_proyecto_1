pub enum ChatMessage {
    Private {
        from: String,
        to: String,
        text: String,
        timestamp: String,
    },
    Public {
        from: String,
        text: String,
        timestamp: String,
    },
    Room {
        room_name: String,
        from: String,
        text: String,
        timestamp: String,
    },
    System {
        text: String,
        timestamp: String,
    },
}
