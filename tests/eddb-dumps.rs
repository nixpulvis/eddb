use chrono::NaiveDate;
use eddb::*;

fn assert_system(system: System) {
    dbg!(&system);

    // If the system is not populated, the population better be 0.
    // NOTE: The inverse is **not** true, meaning that a "populated" system may have a
    // population of 0.
    if !system.is_populated {
        if let Some(p) = system.population {
            assert_eq!(0, p);
        }
    }
    let release_date = NaiveDate::from_ymd_opt(2014, 12, 16).unwrap();
    assert!(release_date < system.updated_at.date_naive());
    assert_eq!(system.government.is_some(), system.government_id.is_some());
    assert_eq!(system.allegiance.is_some(), system.allegiance_id.is_some());
    assert_eq!(system.security.is_some(), system.security_id.is_some());
    assert_eq!(
        system.primary_economy.is_some(),
        system.primary_economy_id.is_some()
    );
    assert_eq!(system.power_state.is_some(), system.power_state_id.is_some());
    assert_eq!(
        system.controlling_minor_faction.is_some(),
        system.controlling_minor_faction_id.is_some()
    );
    assert_eq!(system.reserve_type.is_some(), system.reserve_type_id.is_some());
}

#[test]
fn csv_systems_recently() {
    System::each_csv("tests/systems_recently.csv", &mut |s| {
        assert_system(s);
        false
    });
}

#[test]
fn json_systems_populated() {
    System::each_json("tests/systems_populated.json", &mut |s| {
        assert_system(s);
        false
    });
}

/// Every power the populated dump names reads as one, and every standing
/// but `Expansion` — the one the journal has no word for.
#[test]
fn json_powers_read() {
    let (mut named, mut standing) = (0, 0);
    System::each_json("tests/systems_populated.json", &mut |s| {
        if s.power.is_some() {
            named += 1;
            assert!(s.controlling_power().is_some(), "{:?}", s.power);
        }
        if s.power_state.as_deref().is_some_and(|it| it != "Expansion") {
            standing += 1;
            assert!(s.powerplay_state().is_some(), "{:?}", s.power_state);
        }
        true
    });
    assert!(named > 0 && standing > 0, "the dump named no powers");
}
