// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

//! Ce programme définit un enum `Message` représentant différents types de messages pouvant être envoyés, et une structure `State` qui stocke l'état actuel du système. La structure `State` contient des informations sur la couleur, la position, l'état de sortie et le message en cours.

/// Enum représentant différents types de messages pouvant être envoyés.
enum Message {
    /// Change la couleur avec les composantes RGB.
    ChangeColor(u8, u8, u8),
    /// Répète le message fourni.
    Echo(String),
    /// Déplace l'objet à la position spécifiée.
    Move(Point),
    Quit,
}

/// Structure représentant un point dans un espace à deux dimensions.
struct Point {
    x: u8,
    y: u8,
}

/// Structure contenant l'état actuel du système.
struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    /// Change la couleur de l'état.
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }
    /// Indique que le programme doit quitter.
    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }
    /// Déplace l'objet à la position spécifiée.
    fn move_position(&mut self, p: Point) {
        self.position = p;
    }
    /// Traite le message donné en mettant à jour l'état en conséquence.    
    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(r, g, b) => self.color = (r, g, b),
            Message::Echo(s) => self.message = s,
            Message::Move(p) => self.position = p,
            Message::Quit => self.quit = true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
