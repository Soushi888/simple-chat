<script lang="ts">
	import {onMount, setContext} from 'svelte';
	import type {ActionHash, AgentPubKey, AppAgentClient, Record} from '@holochain/client';
	import {AppAgentWebsocket} from '@holochain/client';
	import '@material/mwc-circular-progress';

	import {clientContext} from './contexts';
	import CreateProfile from "./conversation/profile/CreateProfile.svelte";
	import ProfileDetail from "./conversation/profile/ProfileDetail.svelte";
	import type {Profile} from "./conversation/profile/types";
	import CreateMessage from "./conversation/message/CreateMessage.svelte";
	import AllMessages from "./conversation/message/AllMessages.svelte";
	import {decode} from "@msgpack/msgpack";

	let client: AppAgentClient | undefined;
	let loading = true;
	let createdProfileHash: ActionHash | undefined;
	let myProfile: Profile | undefined;

	$: client, loading;

	onMount(async () => {
		// We pass '' as url because it will dynamically be replaced in launcher environments
		client = await AppAgentWebsocket.connect('', 'simple-chat');
		myProfile = await getProfile();
		loading = false;
	});

	setContext(clientContext, {
		getClient: () => client,
	});

	const getProfile = async (): Promise<Profile | undefined> => {
		if (client === undefined) {
			return undefined;
		}

		try {
			const record = await client.callZome({
				cap_secret: null,
				role_name: 'conversation',
				zome_name: 'profile',
				fn_name: 'get_my_profile',
				payload: null,
			});
			if (record) {
				return decode((record.entry as any).Present.entry) as Profile;
			}
		} catch (e) {
			console.error(e);
		}

		return undefined;
	};
</script>

<main>
  {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate/>
    </div>
  {:else}
    <div id="content" style="display: flex; flex-direction: column; flex: 1;">
      <h1>Hello World !</h1>
      <p>This is a simple chat happ</p>

      {#if myProfile}
        <div>
          <h2>Profile created</h2>
          <p>Now you can start chatting with other agents</p>
        </div>
        <ProfileDetail profileHash={myProfile.agent}/>
        <CreateMessage author={myProfile.agent}/>
        <AllMessages/>
      {:else }
        <div>
          <h2>Profile not created</h2>
          <CreateProfile agent={client.myPubKey} on:profile-created={async () => myProfile = await getProfile()}/>
        </div>
      {/if}
    </div>
  {/if}
</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>
