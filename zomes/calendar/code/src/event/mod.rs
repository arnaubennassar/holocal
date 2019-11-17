use hdk::{
    self,
    error::ZomeApiResult,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
};


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Event {

}

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
        }
    )
}

pub fn create_event(title: String) -> ZomeApiResult<()> {
    Ok(())
}
// get event function

// create event