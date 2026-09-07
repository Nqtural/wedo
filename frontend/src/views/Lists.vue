<script setup lang="ts">
import { ref, onMounted } from "vue";
import { apiFetch } from "@/api";

import EditList from "../components/EditList.vue";
import Header from "../components/Header.vue";
import Button from "../components/Button.vue";
import ListPage from "../components/ListPage.vue";
import ListItem from "../components/ListItem.vue";
import ListItemActions from "../components/ListItemActions.vue";

interface ListItem {
	id: string;
	name: string;
}

const list = ref<ListItem[]>([]);
const selectedListId = ref<string | null>(null);
const creatingList = ref(false);

async function getLists() {
	list.value = await apiFetch<ListItem[]>("/lists");
}

onMounted(async () => {
	await getLists();
});

async function createList() {
	await apiFetch("/lists", {
		method: "POST",
		body: JSON.stringify({
			name: "name",
		}),
	});

	await getLists();
}

function newList() {
	creatingList.value = true;
}

async function closeList() {
	selectedListId.value = null;
	creatingList.value = false;

	await getLists();
}
</script>

<template>
	<ListPage title="Task Lists">
		<ListItem v-for="listItem in list" :key="listItem.id">
			<router-link :to="{ name: 'List', params: { id: listItem.id } }">
				{{ listItem.name }}
			</router-link>
			<ListItemActions>
				<template #hiding>
					<Button
						type="button"
						variant="primary"
						@click.stop="selectedListId = listItem.id"
						>Edit</Button
					>
				</template>
			</ListItemActions>
		</ListItem>

		<template #actions>
			<Button type="button" variant="success" @click="newList"
				>Add list</Button
			>
		</template>
	</ListPage>

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
a {
	display: flex;
	align-items: center;
	padding: 10px;
	flex: 1;
	text-decoration: none;
	color: var(--color-text);
	border-radius: 6px;
}

li:hover .btn-container > * {
	opacity: 1;
}
</style>
