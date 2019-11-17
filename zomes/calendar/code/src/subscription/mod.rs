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
pub struct Subscription {

}

pub fn event_definition() -> ValidatingEntryType {
    entry!(
        name: "subscription",
        description: "this is a subscription entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Subscription>| {
            Ok(())
        }
    )
}

pub fn create_subscription(title: String) -> ZomeApiResult<()> {
    Ok(())
}
// Link definition ( subscriber -> calendar )

// subscribe function (calendar)
