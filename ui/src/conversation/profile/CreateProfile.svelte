<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Profile } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let agent!: AgentPubKey;


let name: string = '';
let avatarUrl: string = '';

let errorSnackbar: Snackbar;

$: agent, name, avatarUrl;
$: isProfileValid = true && name !== '' && avatarUrl !== '';

onMount(() => {
  if (agent === undefined) {
    throw new Error(`The agent input is required for the CreateProfile element`);
  }
});

async function createProfile() {  
  const profileEntry: Profile = { 
    agent: agent!,
    name: name!,
    avatar_url: avatarUrl!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'conversation',
      zome_name: 'profile',
      fn_name: 'create_profile',
      payload: profileEntry,
    });
    dispatch('profile-created', { profileHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the profile: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Profile</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Name" value={ name } on:input={e => { name = e.target.value; } } required></mwc-textfield>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Avatar Url" value={ avatarUrl } on:input={e => { avatarUrl = e.target.value; } } required></mwc-textfield>          
  </div>
            

  <mwc-button 
    raised
    label="Create Profile"
    disabled={!isProfileValid}
    on:click={() => createProfile()}
  ></mwc-button>
</div>
