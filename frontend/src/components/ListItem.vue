<script setup lang="ts">
type Props = {
	expanded?: boolean;
};

withDefaults(defineProps<Props>(), {
	expanded: false,
});
</script>

<template>
	<li :class="{ expanded: expanded }">
		<div class="wrapper wrapper--normal">
			<slot />
		</div>
		<div class="wrapper wrapper--expanded">
			<slot name="expanded" />
		</div>
	</li>
</template>

<style scoped>
li {
	background: var(--color-surface-1);
	border: var(--border-width) solid var(--color-surface-2);
	border-radius: var(--radius-sm);
	--li-height: 50px;
	height: var(--li-height);
	flex-shrink: 0;
	display: flex;
	flex-direction: column;
	transition:
		height var(--transition-slow),
		border-color var(--transition-fast),
		filter var(--transition-slow);
	filter: var(--shadow-sm);
	overflow: hidden;

	&:hover {
		filter: var(--shadow-md);

		& .btn-container {
			opacity: 1;
		}
	}

	&.expanded {
		height: calc(var(--li-height) * 2.5);
	}
}

.wrapper {
	display: flex;
	width: 100%;
	padding: 10px;
	box-sizing: border-box;
	position: relative;
	flex-shrink: 0;

	&.wrapper--normal {
		height: var(--li-height);
	}

	&.wrapper--expanded {
		height: calc(var(--li-height) * 1.5);
		padding-top: 0;
	}
}
</style>
