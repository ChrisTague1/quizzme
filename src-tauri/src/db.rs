use std::collections::HashSet;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

type Result<T, E = String> = std::result::Result<T, E>;

const DECK_NOT_FOUND: &str = "Study guide not found";
const CARD_NOT_FOUND: &str = "Card not found";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Deck {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub card_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardSide {
    pub text: String,
    pub images: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub deck_id: i64,
    pub front: CardSide,
    pub back: CardSide,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardInput {
    pub id: Option<i64>,
    pub deck_id: i64,
    pub front: CardSide,
    pub back: CardSide,
}

fn err(e: impl ToString) -> String {
    e.to_string()
}

pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

pub fn open(path: &Path) -> rusqlite::Result<Connection> {
    let mut conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
    migrate(&mut conn)?;
    Ok(conn)
}

fn migrate(conn: &mut Connection) -> rusqlite::Result<()> {
    let version: i64 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
    if version == 0 {
        let tx = conn.transaction()?;
        tx.execute_batch(
            "CREATE TABLE decks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE TABLE cards (
                id INTEGER PRIMARY KEY,
                deck_id INTEGER NOT NULL REFERENCES decks(id) ON DELETE CASCADE,
                front_text TEXT NOT NULL DEFAULT '',
                front_images TEXT NOT NULL DEFAULT '[]',
                back_text TEXT NOT NULL DEFAULT '',
                back_images TEXT NOT NULL DEFAULT '[]',
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE INDEX cards_deck_id ON cards(deck_id);
            PRAGMA user_version = 1;",
        )?;
        tx.commit()?;
    }
    Ok(())
}

const DECK_SELECT: &str = "SELECT d.id, d.title, d.description, d.created_at, d.updated_at,
    (SELECT COUNT(*) FROM cards c WHERE c.deck_id = d.id) FROM decks d";

fn deck_from_row(r: &Row) -> rusqlite::Result<Deck> {
    Ok(Deck {
        id: r.get(0)?,
        title: r.get(1)?,
        description: r.get(2)?,
        created_at: r.get(3)?,
        updated_at: r.get(4)?,
        card_count: r.get(5)?,
    })
}

const CARD_SELECT: &str = "SELECT id, deck_id, front_text, front_images, back_text, back_images,
    created_at, updated_at FROM cards";

/// Raw card row; image columns are still JSON so parse errors surface as `String`s.
struct CardRow {
    id: i64,
    deck_id: i64,
    front_text: String,
    front_images: String,
    back_text: String,
    back_images: String,
    created_at: i64,
    updated_at: i64,
}

fn card_row(r: &Row) -> rusqlite::Result<CardRow> {
    Ok(CardRow {
        id: r.get(0)?,
        deck_id: r.get(1)?,
        front_text: r.get(2)?,
        front_images: r.get(3)?,
        back_text: r.get(4)?,
        back_images: r.get(5)?,
        created_at: r.get(6)?,
        updated_at: r.get(7)?,
    })
}

impl CardRow {
    fn into_card(self) -> Result<Card> {
        Ok(Card {
            id: self.id,
            deck_id: self.deck_id,
            front: CardSide {
                text: self.front_text,
                images: serde_json::from_str(&self.front_images).map_err(err)?,
            },
            back: CardSide {
                text: self.back_text,
                images: serde_json::from_str(&self.back_images).map_err(err)?,
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

fn validate_title(title: &str) -> Result<&str> {
    let t = title.trim();
    if t.is_empty() {
        return Err("Title is required".into());
    }
    Ok(t)
}

pub fn list_decks(conn: &Connection) -> Result<Vec<Deck>> {
    let mut stmt = conn.prepare(DECK_SELECT).map_err(err)?;
    let rows = stmt.query_map([], deck_from_row).map_err(err)?;
    rows.collect::<rusqlite::Result<_>>().map_err(err)
}

pub fn get_deck(conn: &Connection, id: i64) -> Result<Deck> {
    conn.query_row(&format!("{DECK_SELECT} WHERE d.id = ?1"), [id], deck_from_row)
        .optional()
        .map_err(err)?
        .ok_or_else(|| DECK_NOT_FOUND.into())
}

pub fn create_deck(conn: &Connection, title: &str, description: &str) -> Result<i64> {
    let title = validate_title(title)?;
    let now = now_ms();
    conn.execute(
        "INSERT INTO decks (title, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
        params![title, description, now],
    )
    .map_err(err)?;
    Ok(conn.last_insert_rowid())
}

pub fn update_deck(conn: &Connection, id: i64, title: &str, description: &str) -> Result<()> {
    let title = validate_title(title)?;
    let changed = conn
        .execute(
            "UPDATE decks SET title = ?1, description = ?2, updated_at = ?3 WHERE id = ?4",
            params![title, description, now_ms(), id],
        )
        .map_err(err)?;
    if changed == 0 {
        return Err(DECK_NOT_FOUND.into());
    }
    Ok(())
}

pub fn delete_deck(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM decks WHERE id = ?1", [id]).map_err(err)?;
    Ok(())
}

pub fn list_cards(conn: &Connection, deck_id: i64) -> Result<Vec<Card>> {
    let mut stmt = conn
        .prepare(&format!("{CARD_SELECT} WHERE deck_id = ?1 ORDER BY id"))
        .map_err(err)?;
    let rows = stmt.query_map([deck_id], card_row).map_err(err)?;
    rows.map(|r| r.map_err(err).and_then(CardRow::into_card)).collect()
}

pub fn get_card(conn: &Connection, id: i64) -> Result<Card> {
    conn.query_row(&format!("{CARD_SELECT} WHERE id = ?1"), [id], card_row)
        .optional()
        .map_err(err)?
        .ok_or_else(|| CARD_NOT_FOUND.to_string())?
        .into_card()
}

fn side_ok(s: &CardSide) -> bool {
    !s.text.trim().is_empty() || !s.images.is_empty()
}

pub fn save_card(conn: &mut Connection, input: &CardInput) -> Result<i64> {
    if !side_ok(&input.front) || !side_ok(&input.back) {
        return Err("Each side needs text or an image".into());
    }
    let front_images = serde_json::to_string(&input.front.images).map_err(err)?;
    let back_images = serde_json::to_string(&input.back.images).map_err(err)?;
    let now = now_ms();
    let tx = conn.transaction().map_err(err)?;
    let id = match input.id {
        None => {
            tx.execute(
                "INSERT INTO cards (deck_id, front_text, front_images, back_text, back_images,
                    created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
                params![
                    input.deck_id,
                    input.front.text,
                    front_images,
                    input.back.text,
                    back_images,
                    now
                ],
            )
            .map_err(err)?;
            tx.last_insert_rowid()
        }
        Some(id) => {
            let changed = tx
                .execute(
                    "UPDATE cards SET front_text = ?1, front_images = ?2, back_text = ?3,
                        back_images = ?4, updated_at = ?5 WHERE id = ?6 AND deck_id = ?7",
                    params![
                        input.front.text,
                        front_images,
                        input.back.text,
                        back_images,
                        now,
                        id,
                        input.deck_id
                    ],
                )
                .map_err(err)?;
            if changed == 0 {
                return Err(CARD_NOT_FOUND.into());
            }
            id
        }
    };
    tx.execute(
        "UPDATE decks SET updated_at = ?1 WHERE id = ?2",
        params![now, input.deck_id],
    )
    .map_err(err)?;
    tx.commit().map_err(err)?;
    Ok(id)
}

pub fn delete_card(conn: &mut Connection, id: i64) -> Result<()> {
    let tx = conn.transaction().map_err(err)?;
    let deck_id: i64 = tx
        .query_row("SELECT deck_id FROM cards WHERE id = ?1", [id], |r| r.get(0))
        .optional()
        .map_err(err)?
        .ok_or_else(|| CARD_NOT_FOUND.to_string())?;
    tx.execute("DELETE FROM cards WHERE id = ?1", [id]).map_err(err)?;
    tx.execute(
        "UPDATE decks SET updated_at = ?1 WHERE id = ?2",
        params![now_ms(), deck_id],
    )
    .map_err(err)?;
    tx.commit().map_err(err)
}

pub fn referenced_media(conn: &Connection) -> Result<HashSet<String>> {
    let mut stmt = conn
        .prepare("SELECT front_images, back_images FROM cards")
        .map_err(err)?;
    let mut rows = stmt.query([]).map_err(err)?;
    let mut set = HashSet::new();
    while let Some(row) = rows.next().map_err(err)? {
        for col in 0..2 {
            let json: String = row.get(col).map_err(err)?;
            let names: Vec<String> = serde_json::from_str(&json).map_err(err)?;
            set.extend(names);
        }
    }
    Ok(set)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (tempfile::TempDir, Connection) {
        let dir = tempfile::tempdir().unwrap();
        let conn = open(&dir.path().join("test.db")).unwrap();
        (dir, conn)
    }

    fn side(text: &str, images: &[&str]) -> CardSide {
        CardSide {
            text: text.into(),
            images: images.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn new_card(deck_id: i64, front: CardSide, back: CardSide) -> CardInput {
        CardInput { id: None, deck_id, front, back }
    }

    #[test]
    fn blank_title_rejected() {
        let (_d, conn) = setup();
        assert_eq!(create_deck(&conn, "  ", "").unwrap_err(), "Title is required");
        let id = create_deck(&conn, "Ok", "").unwrap();
        assert_eq!(update_deck(&conn, id, " ", "").unwrap_err(), "Title is required");
        assert_eq!(update_deck(&conn, id + 1, "X", "").unwrap_err(), DECK_NOT_FOUND);
        assert_eq!(get_deck(&conn, id + 1).unwrap_err(), DECK_NOT_FOUND);
    }

    #[test]
    fn card_count_and_updated_at() {
        let (_d, mut conn) = setup();
        let deck = create_deck(&conn, " Spanish ", "words").unwrap();
        let before = get_deck(&conn, deck).unwrap();
        assert_eq!(before.title, "Spanish");
        save_card(&mut conn, &new_card(deck, side("hola", &[]), side("hi", &[]))).unwrap();
        save_card(&mut conn, &new_card(deck, side("adios", &[]), side("bye", &[]))).unwrap();
        let decks = list_decks(&conn).unwrap();
        assert_eq!(decks[0].card_count, 2);
        assert!(decks[0].updated_at >= before.updated_at);
    }

    #[test]
    fn empty_side_rejected_image_only_ok() {
        let (_d, mut conn) = setup();
        let deck = create_deck(&conn, "D", "").unwrap();
        let bad = new_card(deck, side("q", &[]), side("   ", &[]));
        assert_eq!(save_card(&mut conn, &bad).unwrap_err(), "Each side needs text or an image");
        let ok = new_card(deck, side("q", &[]), side("", &["a.png"]));
        assert!(save_card(&mut conn, &ok).is_ok());
    }

    #[test]
    fn update_with_wrong_deck_rejected() {
        let (_d, mut conn) = setup();
        let a = create_deck(&conn, "A", "").unwrap();
        let b = create_deck(&conn, "B", "").unwrap();
        let id = save_card(&mut conn, &new_card(a, side("q", &[]), side("a", &[]))).unwrap();
        let mut input = new_card(b, side("q2", &[]), side("a2", &[]));
        input.id = Some(id);
        assert_eq!(save_card(&mut conn, &input).unwrap_err(), CARD_NOT_FOUND);
        assert_eq!(get_card(&conn, id).unwrap().front.text, "q");
    }

    #[test]
    fn images_order_round_trips() {
        let (_d, mut conn) = setup();
        let deck = create_deck(&conn, "D", "").unwrap();
        let input = new_card(deck, side("q", &["z.png", "a.png", "m.jpg"]), side(" a ", &[]));
        let id = save_card(&mut conn, &input).unwrap();
        let card = get_card(&conn, id).unwrap();
        assert_eq!(card.front.images, vec!["z.png", "a.png", "m.jpg"]);
        assert_eq!(card.back.text, " a ");
        assert_eq!(card.deck_id, deck);
    }

    #[test]
    fn delete_card_and_deck() {
        let (_d, mut conn) = setup();
        let keep = create_deck(&conn, "Keep", "").unwrap();
        let gone = create_deck(&conn, "Gone", "").unwrap();
        save_card(&mut conn, &new_card(keep, side("q", &["keep.png"]), side("a", &[]))).unwrap();
        let c = save_card(&mut conn, &new_card(gone, side("q", &["x.png"]), side("a", &["y.png"])))
            .unwrap();
        save_card(&mut conn, &new_card(gone, side("q", &[]), side("a", &["w.png"]))).unwrap();

        delete_card(&mut conn, c).unwrap();
        assert_eq!(get_card(&conn, c).unwrap_err(), CARD_NOT_FOUND);
        assert_eq!(delete_card(&mut conn, c).unwrap_err(), CARD_NOT_FOUND);
        assert_eq!(list_cards(&conn, gone).unwrap().len(), 1);

        delete_deck(&conn, gone).unwrap();
        assert!(list_cards(&conn, gone).unwrap().is_empty());
        let refs = referenced_media(&conn).unwrap();
        assert_eq!(refs, HashSet::from(["keep.png".to_string()]));
    }
}
