<script setup lang="ts">
import { ref, onMounted } from "vue";
import { apiFetch } from "@/api";

import Button from "./Button.vue";
import EditModal from "./EditModal.vue";
import LinkCopy from "./LinkCopy.vue";

interface List {
	id: string;
	name: string;
}

const origin = window.location.origin;

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
const invitationCode = ref<string | null>(null);

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

async function getShareLink() {
	invitationCode.value = await apiFetch<string>(`/lists/${props.listId}/share`, {
		method: "POST",
	});
}
</script>

<template>
	<EditModal
		:create="create"
		:loading="loading"
		:error="error"
		title="Edit list"
		@close="emit('close')"
		@save="saveList"
		@delete="deleteList"
	>
		<label>Name</label>
		<input v-if="list" v-model="list.name" type="text" />
		<LinkCopy :hidden="invitationCode === null" :link="`${origin}/join/${invitationCode}`" />
		<template v-if="!create" #extra-buttons>
			<Button @click="getShareLink()" variant="primary">Share</Button>
		</template>
	</EditModal>
</template>
