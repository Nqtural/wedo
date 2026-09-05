<script setup lang="ts">
import { ref, onMounted } from "vue";

import EditList from "../components/EditList.vue";
import Header from "../components/Header.vue";

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
	<Header />

	<div class="task-lists-container">
		<h2>Task Lists</h2>
		<hr>
		<ul>
			<li v-for="listItem in list" :key="listItem.id">
				<router-link :to="{ name: 'List', params: { id: listItem.id } }">
					{{ listItem.name }}
				</router-link>
				<button @click="selectedListId = listItem.id">Edit</button>
			</li>
			<button id="new-list-btn" @click="newList">Add list</button>
		</ul>
	</div>

	<Transition name="overlay">
		<EditList
			v-if="selectedListId || creatingList"
			:listId="selectedListId"
			:create="creatingList"
			@close="closeList"
		/>
	</Transition>
</template>

<style scoped>
.task-lists-container {
	display: flex;
	flex-direction: column;
	width: min(50em, calc(100vw - 5em));
	background: dimgray;
	border-radius: 6px;
	color: black;
	padding: 20px;

	/* push content further up */
	margin-bottom: 10em;
}

h2 {
	margin: 0;
}

hr {
	height: 1.5px;
	background: black;
	width: 100%;
	border: none;
	margin: 20px 0px;
}

ul {
	margin: 0;
	padding: 0;
	list-style: none;
	display: flex;
	gap: 10px;
	flex-direction: column;
	flex: 1;
	justify-content: center;
	align-items: center;
}

li {
	background: darkgray;
	border-radius: 6px;
	--li-height: 50px;
	height: var(--li-height);
	display: flex;
	position: relative;
	width: 100%;

	& a:hover ~ button,
	button:hover {
		opacity: 1;
		pointer-events: all;
	}
}

a {
	display: flex;
	align-items: center;
	padding: 10px;
	flex: 1;
	text-decoration: none;
	color: black;
	border-radius: 6px;
}

a ~ button {
	position: absolute;
	--margin: calc(var(--li-height) / 5);
	right: var(--margin);
	top: var(--margin);
	height: calc(var(--margin) * 3);
	width: calc(var(--margin) * 3);
	opacity: 0;
	overflow: hidden;
	pointer-events: none;
	border-radius: 50%;
	transition: opacity 0.1s;
}

#new-list-btn {
	height: 25px;
	width: 25%;
}
</style>
