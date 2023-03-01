use hdk::prelude::*;
use profile_integrity::*;

#[hdk_extern]
pub fn create_profile(profile: Profile) -> ExternResult<Record> {
  let agent = profile.agent.clone();
  let links = get_links(agent, LinkTypes::CreatorToProfiles, None)?;

  if links.len() > 0 {
    return Err(wasm_error!(
      WasmErrorInner::Guest(String::from("Agent already has a profile"))
    ));
  }

  let profile_hash = create_entry(&EntryTypes::Profile(profile.clone()))?;
  create_link(
    profile.agent.clone(),
    profile_hash.clone(),
    LinkTypes::CreatorToProfiles,
    (),
  )?;
  let record = get(profile_hash.clone(), GetOptions::default())?
    .ok_or(
      wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Profile"))
            ),
    )?;
  let path = Path::from("all_profiles");
  create_link(
    path.path_entry_hash()?,
    profile_hash.clone(),
    LinkTypes::AllProfiles,
    (),
  )?;
  Ok(record)
}

#[hdk_extern]
pub fn get_profile(original_profile_hash: ActionHash) -> ExternResult<Option<Record>> {
  let links = get_links(
    original_profile_hash.clone(),
    LinkTypes::ProfileUpdates,
    None,
  )?;
  let latest_link = links
    .into_iter()
    .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
  let latest_profile_hash = match latest_link {
    Some(link) => ActionHash::from(link.target.clone()),
    None => original_profile_hash.clone(),
  };
  get(latest_profile_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_my_profile(_: ()) -> ExternResult<Option<Record>> {
  let agent = agent_info()?.agent_initial_pubkey;
  let links = get_links(agent, LinkTypes::CreatorToProfiles, None)?;
  let latest_link = links
    .into_iter()
    .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
  let latest_profile_hash = match latest_link {
    Some(link) => ActionHash::from(link.target.clone()),
    None => return Ok(None),
  };
  get(latest_profile_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProfileInput {
  pub original_profile_hash: ActionHash,
  pub previous_profile_hash: ActionHash,
  pub updated_profile: Profile,
}

#[hdk_extern]
pub fn update_profile(input: UpdateProfileInput) -> ExternResult<Record> {
  let updated_profile_hash = update_entry(
    input.previous_profile_hash.clone(),
    &input.updated_profile,
  )?;
  create_link(
    input.original_profile_hash.clone(),
    updated_profile_hash.clone(),
    LinkTypes::ProfileUpdates,
    (),
  )?;
  let record = get(updated_profile_hash.clone(), GetOptions::default())?
    .ok_or(
      wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Profile"))
            ),
    )?;
  Ok(record)
}

#[hdk_extern]
pub fn delete_profile(original_profile_hash: ActionHash) -> ExternResult<ActionHash> {
  delete_entry(original_profile_hash)
}

#[hdk_extern]
pub fn get_profiles_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Record>> {
  let links = get_links(creator, LinkTypes::CreatorToProfiles, None)?;
  let get_input: Vec<GetInput> = links
    .into_iter()
    .map(|link| GetInput::new(
      ActionHash::from(link.target).into(),
      GetOptions::default(),
    ))
    .collect();
  let records: Vec<Record> = HDK
    .with(|hdk| hdk.borrow().get(get_input))?
    .into_iter()
    .filter_map(|r| r)
    .collect();
  Ok(records)
}
