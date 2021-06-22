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
    let mut company = create_unit(&connection, None, &company_name, "company", 0, 0, None);
    populate_company_command(connection, &mut company);
    create_platoons(&connection, &company)
}

fn create_platoons(connection: &PgConnection, company: &Unit){
    let first_platoon = create_unit(&connection, Some(company.id), "Erstens Zug", "platoon", 0, 0, None);
    create_section(connection, &first_platoon, true);

    let second_platoon = create_unit(&connection, Some(company.id), "Zweitens Zug", "platoon", 0, 0, None);
    create_section(connection, &second_platoon, true);

    let third_platoon = create_unit(&connection, Some(company.id), "Drittens Zug", "platoon", 0, 0, None);
    create_section(connection, &third_platoon, false);
}

fn create_section(connection: &PgConnection, platoon: &Unit, leutnant: bool){
    use das_heer_uberwatcher::schema::units::dsl::*;
    let mut first_section = create_unit(&connection, Some(platoon.id), "Erstens Gruppen", "section", 0, 0, None);
    let leader = populate_section_command(connection, &mut &mut first_section, leutnant);
    let _ = diesel::update(units.find(platoon.id))
        .set(leader_id.eq(Some(leader.id)))
        .get_result::<Unit>(connection)
        .expect(&format!("Unable to add leader {:?}", Some(leader.id)));

    let mut second_section = create_unit(&connection, Some(platoon.id), "Zweitens Gruppen", "section", 0, 0, None);
    populate_section(connection, &mut second_section);
    
    let mut third_section = create_unit(&connection, Some(platoon.id), "Drittens Gruppen", "section", 0, 0, None);
    populate_section(connection, &mut third_section);
}

fn populate_section(connection: &PgConnection, section: &mut Unit){
    use das_heer_uberwatcher::schema::units::dsl::*;
    use das_heer_uberwatcher::schema::names::dsl::*;
    let mut rng = rand::thread_rng();

    let name_result = names.load::<Name>(connection).expect("Error finding names");

    // Unteroffizer
    let offizer = create_person(&connection, Some(section.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 5, "Ready");
    println!("{:?}", offizer);

    // Jager/Soldat
    for _ in 0..7{
        create_person(&connection, Some(section.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 1, "Ready");
    }

    let _ = diesel::update(units.find(section.id))
        .set(leader_id.eq(Some(offizer.id)))
        .get_result::<Unit>(connection)
        .expect(&format!("Unable to add leader {:?}", Some(offizer.id)));
}

fn populate_section_command(connection: &PgConnection, section: &mut Unit, leutnant: bool) -> Person{
    use das_heer_uberwatcher::schema::units::dsl::*;
    use das_heer_uberwatcher::schema::names::dsl::*;
    let mut rng = rand::thread_rng();

    let name_result = names.load::<Name>(connection).expect("Error finding names");
    let leader;

    if leutnant {
         // Leutnant
         leader = create_person(&connection, Some(section.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 11, "Ready");
    } else {
         // Oberstabsfeldwebel
         leader = create_person(&connection, Some(section.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 9, "Ready");
    }

    // Unteroffizer
    let offizer = create_person(&connection, Some(section.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 5, "Ready");
    println!("{:?}", offizer);

    // Jager/Soldat
    for _ in 0..6{
        create_person(&connection, Some(section.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 1, "Ready");
    }

    let _ = diesel::update(units.find(section.id))
        .set(leader_id.eq(Some(offizer.id)))
        .get_result::<Unit>(connection)
        .expect(&format!("Unable to add leader {:?}", Some(offizer.id)));
    return leader
}

fn populate_company_command(connection: &PgConnection, company: &mut Unit){
    use das_heer_uberwatcher::schema::units::dsl::*;
    use das_heer_uberwatcher::schema::names::dsl::*;
    let mut rng = rand::thread_rng();

    let name_result = names.load::<Name>(connection).expect("Error finding names");

    // Hauptmann
    let offizer = create_person(&connection, Some(company.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 12, "Ready");
    println!("{:?}", offizer);

    // Feldwebel
    create_person(&connection, Some(company.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 6, "Ready");

    // Jager/Soldat
    create_person(&connection, Some(company.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 1, "Ready");
    create_person(&connection, Some(company.id), get_first_name(&name_result), get_last_name(&name_result), rng.gen_range(18..34), "DE", 1, "Ready");


    let _ = diesel::update(units.find(company.id))
        .set(leader_id.eq(Some(offizer.id)))
        .get_result::<Unit>(connection)
        .expect(&format!("Unable to add leader {:?}", Some(offizer.id)));
}

fn get_first_name(collection_of_names: &Vec<Name>) -> &str {
    let mut rng = rand::thread_rng();
    let name = collection_of_names[rng.gen_range(0..(collection_of_names.len()-1))].first_name.as_str();
    return name
}

fn get_last_name(collection_of_names: &Vec<Name>) -> &str {
    let mut rng = rand::thread_rng();
    let name = collection_of_names[rng.gen_range(0..(collection_of_names.len()-1))].last_name.as_str();
    return name
}