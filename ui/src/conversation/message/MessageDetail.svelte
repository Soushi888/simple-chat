<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Message } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditMessage from './EditMessage.svelte'; 

const dispatch = createEventDispatcher();

export let messageHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let message: Message | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, message;

onMount(async () => {
  if (messageHash === undefined) {
    throw new Error(`The messageHash input is required for the MessageDetail element`);
  }
  await fetchMessage();
});

async function fetchMessage() {
  loading = true;
  error = undefined;
  record = undefined;
  message = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'conversation',
      zome_name: 'message',
      fn_name: 'get_message',
      payload: messageHash,
    });
    if (record) {
      message = decode((record.entry as any).Present.entry) as Message;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteMessage() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'conversation',
      zome_name: 'message',
      fn_name: 'delete_message',
      payload: messageHash,
    });
    dispatch('message-deleted', { messageHash: messageHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the message: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the message: {error.data.data}</span>
{:else if editing}
<EditMessage
  originalMessageHash={ messageHash}
  currentRecord={record}
  on:message-updated={async () => {
    editing = false;
    await fetchMessage()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditMessage>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteMessage()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Content:</strong></span>
    <span style="white-space: pre-line">{ message.content }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Timestamp:</strong></span>
    <span style="white-space: pre-line">{ new Date(message.timestamp / 1000).toLocaleString() }</span>
  </div>

</div>
{/if}

