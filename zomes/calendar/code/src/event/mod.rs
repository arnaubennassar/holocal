use hdk::{
    self,
    error::ZomeApiResult,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_persistence_api::cas::content::Address,
};


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Event {
    title: String,
    start: u32,
    end: Option<u32>,
    description: Option<String>,
    location: Option<Location>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

// Definiton

pub fn event_definition() -> ValidatingEntryType {
    entry!(
        name: "event",
        description: "this is a event entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Event>| {
            Ok(())
        },
        links: [
            from!(
                "calendar",
                link_type: "calendar->event",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ] 
    )
}

// Store

pub fn create_event(calendar_address: Address, event: Event) -> ZomeApiResult<Address> {
    // Create event
    let event_entry = Entry::App(
        "event".into(),
        event.into(),
    );
    let event_address = hdk::commit_entry(&event_entry)?;

    // Link created event to calendar
    hdk::link_entries(&calendar_address, &event_address, "calendar->event", "")?;
    Ok(event_address)
}

// Get

pub fn get_event(event_address: Address) -> ZomeApiResult<Event> {
    hdk::utils::get_as_type(event_address)
}

pub fn get_events_by_calendar(calendar_address: Address) -> ZomeApiResult<Vec<Event>> {
    hdk::utils::get_links_and_load_type(
        &calendar_address,
        LinkMatch::Exactly("calendar->event"),
        LinkMatch::Any,
    )
}

