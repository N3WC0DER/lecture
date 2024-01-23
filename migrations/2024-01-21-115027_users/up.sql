CREATE TABLE IF NOT EXISTS users (
    chat_id BIGINT NOT NULL PRIMARY KEY,
    username VARCHAR NOT NULL,
    moderator BOOLEAN NOT NULL,
    institute_id INTEGER NOT NULL,
    course INTEGER NOT NULL,
    direction_id INTEGER NOT NULL,
    notification BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS reports (
    subject_id INTEGER NOT NULL PRIMARY KEY,
    lecture_id INTEGER NOT NULL
);