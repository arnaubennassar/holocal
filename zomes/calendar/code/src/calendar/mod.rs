use hdk::{
    self,
    error::ZomeApiResult,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_persistence_api::cas::content::Address,
};



#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Calendar {
    pub title: String,
}

// Definition

pub fn calendar_definition() -> ValidatingEntryType {
    entry!(
        name: "calendar",
        description: "this is a calendar entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Calendar>| {
            Ok(())
        }
    )
}

// Store

pub fn create_calendar(title: String) -> ZomeApiResult<Address> {
    let calendar = Calendar{
        title: title,
    };
    let calendar_entry = Entry::App(
        "calendar".into(),
        calendar.into(),
    );
    let calendar_address = hdk::commit_entry(&calendar_entry)?;
    Ok(calendar_address)
}

// Get

pub fn get_calendar(calendar_address: Address) -> ZomeApiResult<Calendar> {
    hdk::utils::get_as_type(calendar_address)
}

// Link

// pub fn add_event(event_address: Address)


// Link definition ( calendar -> event )

// get calendar function


// add event

// subscribe to calendar (create link)
