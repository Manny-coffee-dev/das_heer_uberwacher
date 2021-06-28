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

INSERT INTO public.ranks VALUES (20, 1, 'Soldat', 'S.', 'DE');
INSERT INTO public.ranks VALUES (21, 2, 'Gefreiter', 'Gefr.', 'DE');
INSERT INTO public.ranks VALUES (22, 3, 'Hauptgefreiter', 'HtpGefr.', 'DE');
INSERT INTO public.ranks VALUES (23, 4, 'Stabsgefreiter', 'StGefr.', 'DE');
INSERT INTO public.ranks VALUES (24, 5, 'Unteroffizier', 'U.', 'DE');
INSERT INTO public.ranks VALUES (25, 6, 'Feldwebel', 'Fw.', 'DE');
INSERT INTO public.ranks VALUES (26, 7, 'Hauptfeldwebel', 'HtpFw.', 'DE');
INSERT INTO public.ranks VALUES (27, 8, 'Stabsfeldwebel', 'StFw.', 'DE');
INSERT INTO public.ranks VALUES (28, 9, 'Oberstabsfeldwebel', 'OStFw.', 'DE');
INSERT INTO public.ranks VALUES (29, 10, 'Fahrich', 'Fr.', 'DE');
INSERT INTO public.ranks VALUES (30, 11, 'Leutnant', 'Lt.', 'DE');
INSERT INTO public.ranks VALUES (31, 12, 'Hauptmann', 'Htpm.', 'DE');
INSERT INTO public.ranks VALUES (32, 13, 'Major', 'Maj.', 'DE');
INSERT INTO public.ranks VALUES (33, 14, 'Oberstleutnant', 'OTL.', 'DE');
INSERT INTO public.ranks VALUES (34, 15, 'Oberst', 'O.', 'DE');
INSERT INTO public.ranks VALUES (35, 16, 'Brigadegeneral', 'BrigGen.', 'DE');
INSERT INTO public.ranks VALUES (36, 17, 'Generalmajor', 'GenMaj.', 'DE');
INSERT INTO public.ranks VALUES (37, 18, 'Generalleutnant', 'GenLt.', 'DE');
INSERT INTO public.ranks VALUES (38, 19, 'General', 'Gen.', 'DE');

CREATE TABLE IF NOT EXISTS names (
        id          SERIAL PRIMARY KEY,
        first_name  VARCHAR NOT NULL,
        last_name   VARCHAR NOT NULL
);

INSERT INTO "names" (first_name,last_name) VALUES ('Alfred','Zuller'),('Klaus','Zimmer'),('Hans','Knapp'),('Hoyt','Wolfe'),('Griffin','Callahan'),('Drake','Mann'),('Dolan','Muller'),('Merrill','Gallagher'),('Colby','Christensen'),('Eric','Burt');
INSERT INTO "names" (first_name,last_name) VALUES ('Mason','Walters'),('Axel','Knight'),('Hammett','Holt'),('Cade','Rowe'),('Phelan','Fulton'),('Aary','Bryant'),('Asher','Stuart'),('Maxwell','Young'),('Elliott','Walsh'),('Otto','Williamson');
INSERT INTO "names" (first_name,last_name) VALUES ('Ben','Schmidt'),('Alexander','Schneider'),('Elias','Fischer'),('Felix','Weber'),('Jonas','Schafer'),('Louis','Meyer'),('Aadne','Wagner'),('Alrec','Becker'),('Elliott','Bauer'),('Emil','Hoffmann');
INSERT INTO "names" (first_name,last_name) VALUES ('Armin','Schulz'),('Johann','Koch'),('Kai','Richter'),('Korbl','Klein'),('Konrad','Wolf'),('Kurt','Schrider'),('Lothar','Neumann'),('Markus','Braun'),('Mathis','Werner'),('Radulf','Hartmann');
INSERT INTO "names" (first_name,last_name) VALUES ('Parsifal','Weiss'),('Rolf','Kruger'),('Siegfreid','Lange'),('Sepp','Meier'),('Rudi','Walter'),('Sussmann','Kohler'),('Stephan','Huber'),('Walther','Kiaser'),('Wenzel','Jung'),('Yvo','Baumann');
INSERT INTO "names" (first_name,last_name) VALUES ('Wotan','Vogel'),('Georg','Gunther'),('Gotz','Lorenz'),('Heine','Graf'),('Hrodulf','Simon'),('Jakob','Bohm'),('Ignatz','Kramer'),('Ivon','Albrecht'),('Killian','Haas'),('Jorg','Sauer');
INSERT INTO "names" (first_name,last_name) VALUES ('Thomas','Ziegler'),('Kuno','Seidel'),('Konstantin','Kuhn'),('Luther','Busch'),('Manfred','Voigt'),('Rein','Pohl'),('Tadday','Winter'),('Apsel','Sommer'),('Baldric','Franke'),('Berndt','Winkler');
