<script lang="ts">
	import {onMount, getContext} from 'svelte';
	import '@material/mwc-circular-progress';
	import type {EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction} from '@holochain/client';
	import {clientContext} from '../../contexts';
	import ProfileDetail from './ProfileDetail.svelte';
	import type {ProfileSignal} from './types';


	let client: AppAgentClient = (getContext(clientContext) as any).getClient();

	let hash: ActionHash | undefined;
	let loading = true;
	let error: any = undefined;

	$: hash, loading, error;

	onMount(async () => {
		await fetchProfile(client.myPubKey);
		client.on('signal', signal => {
			if (signal.zome_name !== 'profile') return;
			const payload = signal.payload as ProfileSignal;
			if (payload.type !== 'EntryCreated') return;
			if (payload.app_entry.type !== 'Profile') return;
			hash = payload.action.hashed.hash;
		});
	});

	async function fetchProfile(agent: AgentPubKey) {
		try {
			const records = await client.callZome({
				cap_secret: null,
				role_name: 'conversation',
				zome_name: 'profile',
				fn_name: 'get_profile_by_agent',
				payload: agent,
			});
			hash = records[0].action.hashed.hash;
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
  <span>Error fetching the profile: {error.data.data}.</span>
{:else if hash.length === 0}
  <span>Profile not found.</span>
{:else}
  <div style="display: flex; flex-direction: column">
    <div style="margin-bottom: 8px;">
      <ProfileDetail profileHash={hash} on:profile-deleted={() => fetchProfile(client.myPubKey)}></ProfileDetail>
    </div>
  </div>
{/if}

