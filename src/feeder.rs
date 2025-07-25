use ulid::Ulid;

use crate::types::{Db, Message};

/**
 * Feeds the database with initial messages for demonstration purposes.
 */
pub fn feed_db(db: &Db) {
    let system_message = Message {
        id: Ulid::new(),
        author: "Démo".to_string(),
        content: "Bonjour !".to_string(),
        avatar: format!("https://api.dicebear.com/9.x/thumbs/svg?seed={}", "Démo"),
    };

    let user_message = Message {
        id: Ulid::new(),
        author: "Utilisateur".to_string(),
        content: "Message de démonstration".to_string(),
        avatar: format!("https://api.dicebear.com/9.x/thumbs/svg?seed={}", "User"),
    };

    let mut db = db.write().unwrap();
    db.insert(system_message.id, system_message.clone());
    db.insert(user_message.id, user_message.clone());
}
