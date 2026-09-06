<script setup lang="ts">
defineProps<{
	title: string
}>()
</script>

<template>
	<Header />

	<div class="list-page">
		<div class="header">
			<slot name="back" />
			<h2>{{ title }}</h2>
		</div>
		<hr>
		<ul>
			<slot />
		</ul>

		<slot name="actions" />
	</div>
</template>

<style scoped>
.list-page {
	display: flex;
	flex-direction: column;
	gap: 20px;
	align-items: center;
	width: min(50em, calc(100vw - 5em));
	background: var(--color-surface-0);
	border-radius: var(--radius-sm);
	color: var(--color-text);
	padding: 20px;
	max-height: 450px;

	/* push content further up */
	margin-bottom: 10em;
}

.header {
	display: flex;
	gap: 20px;
	width: 100%;
}

h2 {
	display: inline-block;
	float: left;
	margin: 0;
}

hr {
	height: 1.5px;
	background: var(--color-text);
	width: 100%;
	border: none;
	margin: 0px;
}

ul {
	margin: 0;
	padding: 0;
	list-style: none;
	display: flex;
	gap: 10px;
	flex-direction: column;
	flex: 1;
	flex-shrink: 0;
	justify-content: center;
	height: fit-content;
	width: 100%;
}
</style>
