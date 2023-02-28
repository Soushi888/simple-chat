import {Writable, writable} from 'svelte/store'
import type {Profile} from "./types";

export const profileStore: Writable<Profile> = writable();

export const getProfile = async (agentPubKey: string) => {
		const profile = await fetch(`http://localhost:8888/profile/${agentPubKey}`)
		return await profile.json()
}