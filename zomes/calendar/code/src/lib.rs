#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

mod calendar;
mod event;
// mod subscription;
// mod anchor;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address,
};

use hdk_proc_macros::zome;


#[zome]
mod holocal {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    // CALENDAR
    #[entry_def]
    pub fn calendar_entry_def() -> ValidatingEntryType {
        calendar::calendar_definition()
    }
    // create calendar
    #[zome_fn("hc_public")]
	pub fn create_calendar(title: String) -> ZomeApiResult<Address> {
		calendar::create_calendar(title)
    }
    
    // get calendar
    #[zome_fn("hc_public")]
	pub fn get_calendar(calendar_address: Address) -> ZomeApiResult<calendar::Calendar> {
		calendar::get_calendar(calendar_address)
    }

    // EVENT
    #[entry_def]
    pub fn event_entry_def() -> ValidatingEntryType {
        event::event_definition()
    }
    // create event
    #[zome_fn("hc_public")]
	pub fn create_event(calendar_address: Address, event: event::Event) -> ZomeApiResult<Address> {
		event::create_event(calendar_address, event)
    }

    #[zome_fn("hc_public")]
	pub fn get_event(event_address: Address) -> ZomeApiResult<event::Event> {
		event::get_event(event_address)
    }

    #[zome_fn("hc_public")]
	pub fn get_events_by_calendar(calendar_address: Address) -> ZomeApiResult<Vec<event::Event>> {
		event::get_events_by_calendar(calendar_address)
    }
    // get calendar
    // get all calendars
    // create event into calendar
    // import event to calendar
    // subscribe to calendar
    

}
