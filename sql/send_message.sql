INSERT INTO messages (username, body, time)
VALUES ($1, $2, NOW())
RETURNING id, username, body, time;