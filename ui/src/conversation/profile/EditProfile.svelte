<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { Profile } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalProfileHash!: ActionHash;

export let currentRecord!: Record;
let currentProfile: Profile = decode((currentRecord.entry as any).Present.entry) as Profile;

let name: string | undefined = currentProfile.name;
let avatarUrl: string | undefined = currentProfile.avatar_url;

let errorSnackbar: Snackbar;

$: name, avatarUrl;
$: isProfileValid = name !== '' && avatarUrl !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditProfile element`);
  }
  if (originalProfileHash === undefined) {
    throw new Error(`The originalProfileHash input is required for the EditProfile element`);
  }
});

async function updateProfile() {

  const profile: Profile = { 
    name: name!,
    avatar_url: avatarUrl!,
    agent: currentProfile.agent,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'conversation',
      zome_name: 'profile',
      fn_name: 'update_profile',
      payload: {
        original_profile_hash: originalProfileHash,
        previous_profile_hash: currentRecord.signed_action.hashed.hash,
        updated_profile: profile
      }
    });
  
    dispatch('profile-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the profile: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Profile</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Name" value={ name } on:input={e => { name = e.target.value; } } required></mwc-textfield>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Avatar Url" value={ avatarUrl } on:input={e => { avatarUrl = e.target.value; } } required></mwc-textfield>    
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
      disabled={!isProfileValid}
      on:click={() => updateProfile()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
