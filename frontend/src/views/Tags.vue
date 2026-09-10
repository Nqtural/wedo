<script setup lang="ts">
import { nextTick, onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { apiFetch } from "@/api";
import {
	formatTagName,
	normalizeTagName,
	type Tag,
	type TagState,
	tagColor,
	tagColors,
} from "@/tag";

import Button from "../components/Button.vue";

const route = useRoute();
const router = useRouter();

interface ListOverview {
	id: string;
	name: string;
}

const listOverview = ref<ListOverview>();
const tags = ref<Tag[]>();
const listId = route.params.id;

const deletingTag = ref("");
const editingTag = ref("");
const editingTagPreviousState = ref<TagState>();
const colorPickerOpen = ref(false);

const createTagState = ref<TagState>(createTagStateDefault());
const colorPickerOpenNew = ref(false);

onMounted(async () => {
	listOverview.value = await apiFetch<ListOverview>(`/lists/${listId}`);
	await updateTags();
});

function createTagStateDefault() {
	const colorKeys = Object.keys(tagColors);
	return {
		name: "",
		color_key: colorKeys[Math.floor(Math.random() * colorKeys.length)]!,
	};
}

async function updateTags() {
	tags.value = (await apiFetch<Tag[]>(`/lists/${listId}/tags`)).sort((a, b) =>
		a.state.name.localeCompare(b.state.name),
	);
}

function startDeleting(tagId: string) {
	if (deletingTag.value !== "") {
		alert("You are already deleting another tag!");
		return;
	}

	deletingTag.value = tagId;
}

function cancelDeleting() {
	deletingTag.value = "";
}

async function deleteTag() {
	await apiFetch(`/tags/${deletingTag.value}`, {
		method: "DELETE",
	});
	await updateTags();

	cancelDeleting();
}

async function startEditing(tag: Tag) {
	if (editingTag.value !== "") {
		alert("You are already editing another tag!");
		return;
	}

	tag.state.name = normalizeTagName(tag.state.name);
	editingTag.value = tag.id;
	editingTagPreviousState.value = {
		name: tag.state.name,
		color_key: tag.state.color_key,
	};
}

function stopEditing() {
	colorPickerOpen.value = false;
	editingTag.value = "";
}

function cancelEditing(tag: Tag) {
	if (editingTagPreviousState.value) {
		tag.state = editingTagPreviousState.value;
	}
	stopEditing();
}

function pickColor(tagState: TagState, color_key: string) {
	tagState.color_key = color_key;
	colorPickerOpen.value = false;
	colorPickerOpenNew.value = false;
}

async function saveTag(tag: Tag) {
	tag.state.name = normalizeTagName(tag.state.name);
	await apiFetch(`/tags/${tag.id}`, {
		method: "PUT",
		body: JSON.stringify(tag.state),
	});
	stopEditing();
}

async function createTag() {
	await apiFetch(`/lists/${listId}/tags`, {
		method: "POST",
		body: JSON.stringify(createTagState.value),
	});
	await updateTags();
	createTagState.value = createTagStateDefault();
}
</script>

<template>
	<div class="wrapper">
		<h1>{{ listOverview?.name }}'s tags</h1>
		<table cellspacing="0" cellpadding="0">
			<tr>
				<th>Name</th>
				<th>Color</th>
				<th>Actions</th>
			</tr>
			<tr v-for="tag in tags" :key="tag.id">
				<td>
					<input
						v-if="editingTag === tag.id"
						v-model="tag.state.name"
						class="edit"
						@input="formatTagName($event, tag.state)"
					/>
					<span v-else>{{ normalizeTagName(tag.state.name) }}</span>
				</td>
				<td>
					<div v-if="editingTag === tag.id" class="edit-color">
						<span
							class="color-display clickable"
							@click="colorPickerOpen = true"
							:style="tagColor(tag.state.color_key)"
						></span>
						<div v-if="colorPickerOpen" class="color-picker">
							<span
								class="color-display clickable"
								v-for="tagColorKey in Object.keys(tagColors)"
								:style="tagColor(tagColorKey)"
								@click="pickColor(tag.state, tagColorKey)"
							></span>
						</div>
					</div>
					<span
						v-else
						class="color-display"
						:style="tagColor(tag.state.color_key)"
					></span>
				</td>
				<td>
					<div v-if="deletingTag === tag.id" class="button-wrapper">
						<Button @click="deleteTag()" variant="danger"
							>Confirm</Button
						>
						<Button @click="cancelDeleting()" variant="primary"
							>Cancel</Button
						>
					</div>
					<div
						v-else-if="editingTag === tag.id"
						class="button-wrapper"
					>
						<Button
							@click="saveTag(tag)"
							variant="success"
							normalizeTag();
							>Save</Button
						>
						<Button @click="cancelEditing(tag)" variant="primary"
							>Cancel</Button
						>
					</div>
					<div v-else class="button-wrapper">
						<Button @click="startEditing(tag)" variant="primary"
							>Edit</Button
						>
						<Button @click="startDeleting(tag.id)" variant="danger"
							>Delete</Button
						>
					</div>
				</td>
			</tr>
			<tr>
				<td>
					<input
						type="text"
						placeholder="new-tag..."
						v-model="createTagState.name"
						@input="formatTagName($event, createTagState)"
					/>
				</td>
				<td>
					<div class="edit-color">
						<span
							class="color-display clickable"
							@click="colorPickerOpenNew = true"
							:style="tagColor(createTagState.color_key)"
						></span>
						<div v-if="colorPickerOpenNew" class="color-picker">
							<span
								class="color-display clickable"
								v-for="tagColorKey in Object.keys(tagColors)"
								:style="tagColor(tagColorKey)"
								@click="pickColor(createTagState, tagColorKey)"
							></span>
						</div>
					</div>
				</td>
				<td>
					<div class="button-wrapper">
						<Button @click="createTag()" variant="success"
							>Create</Button
						>
					</div>
				</td>
			</tr>
		</table>
	</div>
	<Button variant="primary" @click="router.back()">Back</Button>
</template>

<style scoped>
.wrapper {
	background: var(--color-surface-0);
	border: var(--border-width) solid var(--color-surface-1);
	border-radius: var(--radius-sm);
	color: var(--color-text);
	display: flex;
	flex-direction: column;
	gap: 20px;
	padding: 20px;
	filter: var(--shadow-md);

	& h1 {
		margin: 0;
	}

	& table {
		width: 100%;
		border: var(--border-width) solid var(--color-surface-2);
		border-radius: var(--radius-sm);
		filter: var(--shadow-sm);

		& tr {
			--height: 45px;
			height: var(--height);
			background: var(--color-surface-0);

			&:nth-child(odd) {
				background: var(--color-surface-1);
			}

			& td,
			th {
				padding: 0;
				text-align: center;

				& input {
					background: none;
					border: none;
					color: inherit;
					outline: none;
					font-size: 1em;
					text-align: center;
					height: var(--height);

					&.edit {
						background-color: var(--color-surface-2);
					}
				}

				& .button-wrapper {
					display: flex;
					gap: 5px;
					padding-inline: 5px;

					& button {
						flex: 1;
					}
				}

				& .color-display {
					background-color: var(--tag-color);
					display: inline-block;
					width: 30px;
					aspect-ratio: 1/1;
					border-radius: 50%;

					&.clickable {
						cursor: pointer;
					}
				}

				& .edit-color {
					position: relative;

					& .color-picker {
						z-index: 1;
						top: 40px;
						position: absolute;
						left: 50%;
						transform: translateX(-50%);
						display: flex;
						gap: 5px;
						padding: 5px;
						border: var(--border-width) solid var(--color-surface-2);
						border-radius: var(--radius-sm);
						background: var(--color-surface-0);
						filter: var(--shadow-sm);

						&::before,
						&::after {
							content: "";
							position: absolute;
							left: 50%;
							transform: translateX(-50%);
							width: 0;
							height: 0;
						}

						&::before {
							top: -10px;
							border-left: 8px solid transparent;
							border-right: 8px solid transparent;
							border-bottom: 8px solid var(--color-surface-2);
						}

						&::after {
							top: -7px;
							border-left: 7px solid transparent;
							border-right: 7px solid transparent;
							border-bottom: 7px solid var(--color-surface-0);
						}
					}
				}
			}
		}
	}
}

.wrapper ~ button {
	position: absolute;
	bottom: 20px;
	left: 20px;
}
</style>
