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

CREATE TABLE IF NOT EXISTS names (
        id          SERIAL PRIMARY KEY,
        first_name  VARCHAR NOT NULL,
        last_name   VARCHAR NOT NULL
);

INSERT INTO "names" (first_name,last_name) VALUES ('Lamar','Clayton'),('Nigel','Montoya'),('Perry','Marks'),('Hoyt','Sullivan'),('Griffin','Callahan'),('Drake','Edwards'),('Dolan','Chambers'),('Merrill','Gallagher'),('Colby','Christensen'),('Eric','Hicks');
INSERT INTO "names" (first_name,last_name) VALUES ('Garth','Walters'),('Axel','Knight'),('Blaze','Crawford'),('Cade','Rowe'),('Phelan','Atkinson'),('Zane','Lindsay'),('Asher','Stuart'),('Rashad','Silva'),('Ignatius','Gay'),('Rooney','Aguilar');
INSERT INTO "names" (first_name,last_name) VALUES ('Lane','Wolfe'),('Fletcher','Dotson'),('Burton','Mckinney'),('Yuli','Ratliff'),('Ivor','Johnson'),('Hamilton','Sims'),('Melvin','Rodriguez'),('Cruz','Walsh'),('Cedric','Carney'),('Orlando','Cash');
INSERT INTO "names" (first_name,last_name) VALUES ('Dylan','Potter'),('Jason','Robinson'),('Ezekiel','Guerra'),('Baxter','Blake'),('Reese','Young'),('Basil','Mcclain'),('Geoffrey','Holcomb'),('Steel','Love'),('Nolan','Phillips'),('Otto','Williamson');
INSERT INTO "names" (first_name,last_name) VALUES ('Jermaine','Foreman'),('Brody','Hammond'),('Mohammad','Abbott'),('Kasimir','Reed'),('Orson','Quinn'),('Francis','Dyer'),('Mason','Cochran'),('Ira','Francis'),('Andrew','Justice'),('Tanek','Stanley');
INSERT INTO "names" (first_name,last_name) VALUES ('Chadwick','Fulton'),('Logan','Dillon'),('Wallace','Howell'),('Troy','Adams'),('Ali','Workman'),('Amal','Bryant'),('Aaron','Porter'),('Arsenio','Cash'),('Ross','Randall'),('Julian','Shelton');
INSERT INTO "names" (first_name,last_name) VALUES ('Mason','Pate'),('Randall','Holt'),('Clarke','Acosta'),('Declan','Larsen'),('Barry','Green'),('Zachary','Mcdonald'),('Justin','Dixon'),('Nero','Mcknight'),('Benjamin','Mendez'),('Logan','Ayala');
INSERT INTO "names" (first_name,last_name) VALUES ('Macon','Salazar'),('Kaseem','Gibbs'),('Kaseem','Vega'),('Walker','Spence'),('Reese','Hill'),('Cruz','Bishop'),('Nathan','Jimenez'),('Armando','Rios'),('Nolan','Sweet'),('Emerson','Jackson');
INSERT INTO "names" (first_name,last_name) VALUES ('Nehru','Rosario'),('Otto','William'),('Maxwell','Knapp'),('Tad','Huffman'),('Darius','Holcomb'),('Aristotle','Mueller'),('Elliott','Burt'),('Rogan','Grimes'),('Axel','Giles'),('Jason','Le');
INSERT INTO "names" (first_name,last_name) VALUES ('Graiden','Mccarty'),('Orlando','Logan'),('John','Molina'),('Cruz','Jefferson'),('Ezra','Blair'),('Aquila','Mckinney'),('Wesley','Hinton'),('Fitzgerald','Durham'),('Harlan','Finch'),('Hammett','Morse');