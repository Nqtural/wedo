<script setup lang="ts">
import { ref, onMounted } from "vue";
import { apiFetch } from "@/api";

import EditModal from "./EditModal.vue";

interface List {
	id: string;
	name: string;
}

const props = defineProps<{
	listId: string | null;
	create: boolean;
}>();

const emit = defineEmits<{
	close: [];
}>();

const list = ref<List | null>(
	props.create
		? {
				id: "",
				name: "",
			}
		: null,
);
const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
	if (props.create) {
		loading.value = false;
		return;
	}

	try {
		list.value = await apiFetch<List>(`/lists/${props.listId}`);
	} catch (e) {
		error.value = e instanceof Error ? e.message : "Unknown error";
	} finally {
		loading.value = false;
	}
});

async function saveList() {
	if (!list.value) return;

	if (props.create) {
		await createList();
	} else {
		await updateList();
	}
}

async function createList() {
	if (!list.value) return;

	await apiFetch("/lists", {
		method: "POST",
		body: JSON.stringify({
			name: list.value.name,
		}),
	});

	emit("close");
}

async function updateList() {
	if (!list.value) return;

	await apiFetch(`/lists/${props.listId}`, {
		method: "PUT",
		body: JSON.stringify({
			name: list.value.name,
		}),
	});

	emit("close");
}

async function deleteList() {
	await apiFetch(`/lists/${props.listId}`, {
		method: "DELETE",
	});

	emit("close");
}
</script>

<template>
	<EditModal
		:create="create"
		:loading="loading"
		:error="error"
		@close="emit('close')"
		@save="saveList"
		@delete="deleteList"
	>
		<label>Name</label>
		<input v-if="list" v-model="list.name" type="text" />
	</EditModal>
</template>
