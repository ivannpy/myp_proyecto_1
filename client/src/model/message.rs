#[derive(Debug, Clone)]
pub enum ChatMessage {
    Private {
        from: String,
        text: String,
    },
    Public {
        from: String,
        text: String,
    },
    Room {
        roomname: String,
        from: String,
        text: String,
    },
}
