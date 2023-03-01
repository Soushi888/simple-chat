<script lang="ts">
	import {createEventDispatcher, onMount, getContext} from 'svelte';
	import '@material/mwc-circular-progress';
	import {decode} from '@msgpack/msgpack';
	import type {Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash} from '@holochain/client';
	import {clientContext} from '../../contexts';
	import type {Profile} from './types';
	import '@material/mwc-circular-progress';
	import type {Snackbar} from '@material/mwc-snackbar';
	import '@material/mwc-snackbar';
	import '@material/mwc-icon-button';
	import EditProfile from './EditProfile.svelte';

	const dispatch = createEventDispatcher();

	export let profileHash: ActionHash;

	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let loading = true;
	let error: any = undefined;

	let record: Record | undefined;
	let profile: Profile | undefined;

	let editing = false;

	let errorSnackbar: Snackbar;

	$: editing, error, loading, record, profile;

	onMount(async () => {
		if (profileHash === undefined) {
			throw new Error(`The profileHash input is required for the ProfileDetail element`);
		}
		await fetchProfile();
	});

	async function fetchProfile() {
		loading = true;
		error = undefined;
		record = undefined;
		profile = undefined;

		try {
			record = await client.callZome({
				cap_secret: null,
				role_name: 'conversation',
				zome_name: 'profile',
				fn_name: 'get_profile',
				payload: profileHash,
			});
			if (record) {
				profile = decode((record.entry as any).Present.entry) as Profile;
			}
		} catch (e) {
			error = e;
		}

		loading = false;
	}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
  <div style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{:else if error}
  <span>Error fetching the profile: {error.data.data}</span>
{:else if editing}
  <EditProfile
      originalProfileHash={profileHash}
      currentRecord={record}
      on:profile-updated={async () => {
    editing = false;
    await fetchProfile()
  } }
      on:edit-canceled={() => { editing = false; } }
  ></EditProfile>
{:else}

  <div class="profile-details">
    <div style="display: flex; flex-direction: row">
      <span style="flex: 1"></span>
      <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    </div>

    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span style="margin-right: 4px"><strong>Name:</strong></span>
      <span style="white-space: pre-line">{ profile.name }</span>
    </div>

    <div class="avatar">
      <img src="{profile.avatar_url}" alt={`${profile.name}'s avatar`}>
    </div>

  </div>
{/if}

<style>
    .profile-details {
        display: flex;
        align-items: center;
        justify-content: space-around;
        flex-direction: row-reverse;
        border: #333333 1px solid;
        padding: 10px;
    }

    .avatar {
        align-items: center;
    }

    .avatar img {
        max-width: 100px;
        height: 100px;
        border-radius: 50%;
    }
</style>

<!-- https://avatars.githubusercontent.com/u/106820774?s=400&u=4dd78685090e5ac2a4c2a640af7f40be34de126f&v=4 -->

<!-- https://media.licdn.com/dms/image/C5603AQGV-WawixTakw/profile-displayphoto-shrink_800_800/0/1657139065357?e=1683158400&v=beta&t=ngDKSOfZkSsLIqG58GR-z8PuzweYXDEH9Haf0gTPdWk -->