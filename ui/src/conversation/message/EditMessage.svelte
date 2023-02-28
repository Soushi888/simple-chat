<script lang="ts">
	import {createEventDispatcher, getContext, onMount} from 'svelte';
	import type {AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash} from '@holochain/client';
	import {decode} from '@msgpack/msgpack';
	import {clientContext} from '../../contexts';
	import type {Message} from './types';
	import '@material/mwc-button';
	import '@material/mwc-snackbar';
	import type {Snackbar} from '@material/mwc-snackbar';

	import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
	import '@material/mwc-textarea';

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	const dispatch = createEventDispatcher();

	export let originalMessageHash!: ActionHash;

	export let currentRecord!: Record;
	let currentMessage: Message = decode((currentRecord.entry as any).Present.entry) as Message;

	let content: string | undefined = currentMessage.content;
	let timestamp: number | undefined = currentMessage.timestamp;

	let errorSnackbar: Snackbar;

	$: content, timestamp;
	$: isMessageValid = true && content !== '' && true;

	onMount(() => {
		if (currentRecord === undefined) {
			throw new Error(`The currentRecord input is required for the EditMessage element`);
		}
		if (originalMessageHash === undefined) {
			throw new Error(`The originalMessageHash input is required for the EditMessage element`);
		}
	});

	async function updateMessage() {

		const message: Message = {
			content: content!,
			timestamp: timestamp!,
			author: currentMessage.author,
		};

		try {
			const updateRecord: Record = await client.callZome({
				cap_secret: null,
				role_name: 'conversation',
				zome_name: 'message',
				fn_name: 'update_message',
				payload: {
					original_message_hash: originalMessageHash,
					previous_message_hash: currentRecord.signed_action.hashed.hash,
					updated_message: message
				}
			});

			dispatch('message-updated', {actionHash: updateRecord.signed_action.hashed.hash});
		} catch (e) {
			errorSnackbar.labelText = `Error updating the message: ${e.data.data}`;
			errorSnackbar.show();
		}
	}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Message</span>

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} }
                  required></mwc-textarea>
  </div>

  <div style="margin-bottom: 16px">
    <vaadin-date-time-picker label="Timestamp" value={new Date(timestamp / 1000).toISOString()}
                             on:change={e => { timestamp = new Date(e.target.value).valueOf() * 1000;} }
                             required></vaadin-date-time-picker>
  </div>


  <div style="display: flex; flex-direction: row">
    <mwc-button
        outlined
        label="Cancel"
        on:click={() => dispatch('edit-canceled')}
        style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button
        raised
        label="Save"
        disabled={!isMessageValid}
        on:click={() => updateMessage()}
        style="flex: 1;"
    ></mwc-button>
  </div>
</div>
