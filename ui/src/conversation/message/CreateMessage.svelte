<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Message } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let author!: EntryHash;


let content: string = '';
let timestamp: number = Date.now();

let errorSnackbar: Snackbar;

$: content, timestamp, author;
$: isMessageValid = true && content !== '' && true;

onMount(() => {
  if (author === undefined) {
    throw new Error(`The author input is required for the CreateMessage element`);
  }
});

async function createMessage() {  
  const messageEntry: Message = { 
    content: content!,
    timestamp,
    author: author!,
  };

	console.log("messageEntry", messageEntry);

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'conversation',
      zome_name: 'message',
      fn_name: 'create_message',
      payload: messageEntry,
    });
    dispatch('message-created', { messageHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the message: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Message</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} } required></mwc-textarea>          
  </div>

  <mwc-button 
    raised
    label="Create Message"
    disabled={!isMessageValid}
    on:click={() => createMessage()}
  ></mwc-button>
</div>
