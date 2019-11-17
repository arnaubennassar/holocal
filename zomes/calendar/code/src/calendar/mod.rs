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
};



#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Calendar {
    pub title: String,
}

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

pub fn create_calendar(_title: String) -> ZomeApiResult<()> {
    let calendar = Calendar{
        title: _title,
    };
    let calendar_entry = Entry::App(
        "calendar".into(),
        calendar.into(),
    );
    hdk::commit_entry(&calendar_entry)?;
    Ok(())
}



// Link definition ( calendar -> event )

// get calendar function


// add event

// subscribe to calendar (create link)
