extern crate das_heer_uberwatcher;
extern crate diesel;

use self::das_heer_uberwatcher::*;


fn main() {
    let connection = establish_connection();
    // German Enlisted
    create_rank(&connection, 1, "Soldat", "S.", "DE");
    create_rank(&connection, 2, "Gefreiter", "Gefr.", "DE");
    create_rank(&connection, 3, "Hauptgefreiter", "HtpGefr.", "DE");
    create_rank(&connection, 4, "Stabsgefreiter", "StGefr.", "DE");
    create_rank(&connection, 5, "Unteroffizier", "U.", "DE");
    create_rank(&connection, 6, "Feldwebel", "Fw.", "DE");
    create_rank(&connection, 7, "Hauptfeldwebel", "HtpFw.", "DE");
    create_rank(&connection, 8, "Stabsfeldwebel", "StFw.", "DE");
    create_rank(&connection, 9, "Oberstabsfeldwebel", "OStFw.", "DE");
    // German Officers
    create_rank(&connection, 10, "Fahrich", "Fr.", "DE");
    create_rank(&connection, 11, "Leutnant", "Lt.", "DE");
    create_rank(&connection, 12, "Hauptmann", "Htpm.", "DE");
    create_rank(&connection, 13, "Major", "Maj.", "DE");
    create_rank(&connection, 14, "Oberstleutnant", "OTL.", "DE");
    create_rank(&connection, 15, "Oberst", "O.", "DE");
    create_rank(&connection, 16, "Brigadegeneral", "BrigGen.", "DE");
    create_rank(&connection, 17, "Generalmajor", "GenMaj.", "DE");
    create_rank(&connection, 18, "Generalleutnant", "GenLt.", "DE");
    create_rank(&connection, 19, "General", "Gen.", "DE");
}
