<script setup lang="ts">
import { ref, onMounted } from "vue";

import EditList from "../components/EditList.vue";

interface ListItem {
	id: string;
	name: string;
}

const list = ref<ListItem[]>([]);
const selectedListId = ref<string | null>(null);
const creatingList = ref(false);

async function getLists() {
		const response = await fetch(`${import.meta.env.VITE_API_URL}/lists`);

	if (!response.ok) {
		throw new Error(`HTTP error: ${response.status}`);
	}

	list.value = await response.json();
}

onMounted(async () => {
	await getLists();
});

async function createList() {
	const response = await fetch(
		`${import.meta.env.VITE_API_URL}/lists`,
		{
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				name: "name",
			}),
		}
	)

	getLists();
}

function newList() {
	creatingList.value = true;
}

function closeList() {
	selectedListId.value = null;
	creatingList.value = false;

	getLists();
}
</script>

<template>
	<h1>Lists</h1>

	<ul>
		<li v-for="listItem in list" :key="listItem.id">
			<router-link :to="{ name: 'List', params: { id: listItem.id }, query: { name: listItem.name } }">
				{{ listItem.name }}
			</router-link>
			<button @click="selectedListId = listItem.id">Edit</button>
		</li>
		<li>
			<button @click="newList">Add list</button>
		</li>
	</ul>

	<EditList
		v-if="selectedListId || creatingList"
		:listId="selectedListId"
		:create="creatingList"
		@close="closeList"
	/>
</template>

<style scoped></style>
