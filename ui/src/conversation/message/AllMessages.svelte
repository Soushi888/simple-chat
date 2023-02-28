<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import MessageDetail from './MessageDetail.svelte';
import type { MessageSignal } from './types';


let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {

  await fetchMessages();
  client.on('signal', signal => {
    if (signal.zome_name !== 'message') return;
    const payload = signal.payload as MessageSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Message') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchMessages() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'conversation',
      zome_name: 'message',
      fn_name: 'get_all_messages',
      payload: null,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the messages: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No messages found.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <MessageDetail messageHash={hash}  on:message-deleted={() => fetchMessages()}></MessageDetail>
    </div>
  {/each}
</div>
{/if}

