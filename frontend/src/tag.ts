export interface TagState {
	name: string;
	color_key: string;
}

export interface Tag {
	id: string;
	state: TagState;
}

export const tagColors: Record<string, string> = {
	red: "--red",
	orange: "--orange",
	yellow: "--yellow",
	green: "--green",
	teal: "--teal",
	blue: "--blue",
	purple: "--purple",
	pink: "--pink",
};

export function tagColor(color_key: string) {
	return {
		"--tag-color": `var(${tagColors[color_key] ?? "--color-primary"})`,
	};
}

export function formatTagName(event: Event, tagState: TagState) {
	const input = event.target as HTMLInputElement;

	tagState.name = input.value
		.toLowerCase()
		.replace(/\s+/g, "-")
		.replace(/[^a-z0-9-]/g, "")
		.replace(/-+/g, "-");
}

export function normalizeTagName(name: string) {
	return name
		.toLowerCase()
		.replace(/\s+/g, "-")
		.replace(/[^a-z0-9-]/g, "")
		.replace(/-+/g, "-")
		.replace(/^-+|-+$/g, "");
}
