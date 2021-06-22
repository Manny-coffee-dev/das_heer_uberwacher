extern crate das_heer_uberwatcher;
extern crate diesel;

use self::das_heer_uberwatcher::*;
use self::models::*;
use self::diesel::prelude::*;

use rand::Rng;

fn main() {
    let connection = establish_connection();
    create_company(&connection, "Echo Kompanie".to_string());
    
}

fn create_company(connection: &PgConnection, company_name: String){
    use das_heer_uberwatcher::schema::units::dsl::*;

    create_unit(&connection, None, &company_name, "company", 0, 0, None);

    let company_result = units.filter(das_heer_uberwatcher::schema::units::unit_type.eq("company"))
        .load::<Unit>(connection)
        .expect("Error finding company");
    
    for company in company_result{
        create_platoons(&connection, &company);
    }

    let platoon_result = units.filter(das_heer_uberwatcher::schema::units::unit_type.eq("platoon"))
        .load::<Unit>(connection)
        .expect("Error finding platoon");

    for platoon in platoon_result{
        create_section(&connection, &platoon)
    }

    let section_result = units.filter(das_heer_uberwatcher::schema::units::unit_type.eq("section"))
        .load::<Unit>(connection)
        .expect("Error finding section");

    for mut section in section_result{
        populate_section(&connection, &mut section)
    }
}

fn create_platoons(connection: &PgConnection, company: &Unit){
    create_unit(&connection, Some(company.id), "Erstens Zug", "platoon", 0, 0, None);
    create_unit(&connection, Some(company.id), "Zweitens Zug", "platoon", 0, 0, None);
    create_unit(&connection, Some(company.id), "Drittens Zug", "platoon", 0, 0, None);
}

fn create_section(connection: &PgConnection, platoon: &Unit){
    create_unit(&connection, Some(platoon.id), "Erstens Gruppen", "section", 0, 0, None);
    create_unit(&connection, Some(platoon.id), "Zweitens Gruppen", "section", 0, 0, None);
    create_unit(&connection, Some(platoon.id), "Drittens Gruppen", "section", 0, 0, None);
}

fn populate_section(connection: &PgConnection, section: &mut Unit){
    use das_heer_uberwatcher::schema::units::dsl::*;
    let mut rng = rand::thread_rng();

    // Unteroffizer
    let offizer = create_person(&connection, Some(section.id), "Hans", "Schmitt", rng.gen_range(18..34), "DE", 5, "Ready");
    println!("{:?}", offizer);

    // Jager/Soldat
    for _ in 0..7{
        create_person(&connection, Some(section.id), "Hans", "Schmitt", rng.gen_range(18..34), "DE", 1, "Ready");
    }

    let _ = diesel::update(units.find(section.id))
        .set(leader_id.eq(Some(offizer.id)))
        .get_result::<Unit>(connection)
        .expect(&format!("Unable to add leader {:?}", Some(offizer.id)));


}