import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleMessage(cell: CallableCell, partialMessage = {}) {
    return {
        ...{
	  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  timestamp: 1674053334548000,
	  author: (await fakeEntryHash()),
        },
        ...partialMessage
    };
}

export async function createMessage(cell: CallableCell, message = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "message",
      fn_name: "create_message",
      payload: message || await sampleMessage(cell),
    });
}

