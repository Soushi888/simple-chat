use hdk::prelude::*;
use message_integrity::*;
#[hdk_extern]
pub fn create_message(message: Message) -> ExternResult<Record> {
    let message_hash = create_entry(&EntryTypes::Message(message.clone()))?;
    let record = get(message_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Message"))
            ),
        )?;
    let path = Path::from("all_messages");
    create_link(
        path.path_entry_hash()?,
        message_hash.clone(),
        LinkTypes::AllMessages,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_message(original_message_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_message_hash.clone(),
        LinkTypes::MessageUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_message_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_message_hash.clone(),
    };
    get(latest_message_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMessageInput {
    pub original_message_hash: ActionHash,
    pub previous_message_hash: ActionHash,
    pub updated_message: Message,
}
#[hdk_extern]
pub fn update_message(input: UpdateMessageInput) -> ExternResult<Record> {
    let updated_message_hash = update_entry(
        input.previous_message_hash.clone(),
        &input.updated_message,
    )?;
    create_link(
        input.original_message_hash.clone(),
        updated_message_hash.clone(),
        LinkTypes::MessageUpdates,
        (),
    )?;
    let record = get(updated_message_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Message"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_message(original_message_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_message_hash)
}
