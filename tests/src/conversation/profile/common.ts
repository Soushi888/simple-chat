import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleProfile(cell: CallableCell, partialProfile = {}) {
    return {
        ...{
          agent: cell.cell_id[1],
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  avatar_url: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialProfile
    };
}

export async function createProfile(cell: CallableCell, profile = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "profile",
      fn_name: "create_profile",
      payload: profile || await sampleProfile(cell),
    });
}

