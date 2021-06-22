CREATE TABLE IF NOT EXISTS units (
        id          SERIAL PRIMARY KEY,
        parent_id   INTEGER,
        name        VARCHAR NOT NULL,
        unit_type   VARCHAR NOT NULL,
        kills       INTEGER NOT NULL,
        losses      INTEGER NOT NULL,
        leader_id   INTEGER
);

CREATE TABLE IF NOT EXISTS personnel (
        id          SERIAL PRIMARY KEY,
        unit_id   INTEGER,
        first_name  VARCHAR NOT NULL,
        last_name   VARCHAR NOT NULL,
        age         INTEGER NOT NULL,
        nationality VARCHAR NOT NULL,
        rank        INTEGER NOT NULL,
        state       VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS ranks (
        id          SERIAL PRIMARY KEY,
        level       INTEGER NOT NULL,
        name        VARCHAR NOT NULL,
        title       VARCHAR NOT NULL,
        nationality VARCHAR NOT NULL
);