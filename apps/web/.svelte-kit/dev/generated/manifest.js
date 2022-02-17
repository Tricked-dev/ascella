const c = [
	() => import("../../../src/routes/__layout.svelte"),
	() => import("../components/error.svelte"),
	() => import("../../../src/routes/index.svelte"),
	() => import("../../../src/routes/contributors.svelte"),
	() => import("../../../src/routes/domains.svelte"),
	() => import("../../../src/routes/docs/__layout.svelte"),
	() => import("../../../src/routes/docs/index.md"),
	() => import("../../../src/routes/docs/ascella-desktop.md"),
	() => import("../../../src/routes/docs/installing.md"),
	() => import("../../../src/routes/docs/effects.md"),
	() => import("../../../src/routes/docs/privacy.md"),
	() => import("../../../src/routes/docs/domain.md"),
	() => import("../../../src/routes/docs/signup.md"),
	() => import("../../../src/routes/docs/rules.md"),
	() => import("../../../src/routes/docs/api.md"),
	() => import("../../../src/routes/docs/faq.md"),
	() => import("../../../src/routes/[...image]/__layout.reset.svelte"),
	() => import("../../../src/routes/[...image]/index.svelte")
];

const d = decodeURIComponent;

export const routes = [
	// src/routes/index.svelte
	[/^\/$/, [c[0], c[2]], [c[1]]],

	// src/routes/contributors.svelte
	[/^\/contributors\/?$/, [c[0], c[3]], [c[1]]],

	// src/routes/domains.svelte
	[/^\/domains\/?$/, [c[0], c[4]], [c[1]]],

	// src/routes/docs/index.md
	[/^\/docs\/?$/, [c[0], c[5], c[6]], [c[1]]],

	// src/routes/docs/ascella-desktop.md
	[/^\/docs\/ascella-desktop\/?$/, [c[0], c[5], c[7]], [c[1]]],

	// src/routes/docs/installing.md
	[/^\/docs\/installing\/?$/, [c[0], c[5], c[8]], [c[1]]],

	// src/routes/docs/effects.md
	[/^\/docs\/effects\/?$/, [c[0], c[5], c[9]], [c[1]]],

	// src/routes/docs/privacy.md
	[/^\/docs\/privacy\/?$/, [c[0], c[5], c[10]], [c[1]]],

	// src/routes/docs/domain.md
	[/^\/docs\/domain\/?$/, [c[0], c[5], c[11]], [c[1]]],

	// src/routes/docs/signup.md
	[/^\/docs\/signup\/?$/, [c[0], c[5], c[12]], [c[1]]],

	// src/routes/docs/rules.md
	[/^\/docs\/rules\/?$/, [c[0], c[5], c[13]], [c[1]]],

	// src/routes/docs/api.md
	[/^\/docs\/api\/?$/, [c[0], c[5], c[14]], [c[1]]],

	// src/routes/docs/faq.md
	[/^\/docs\/faq\/?$/, [c[0], c[5], c[15]], [c[1]]],

	// src/routes/[...image]/index.svelte
	[/^(?:\/(.*))?\/?$/, [c[16], c[17]], [], (m) => ({ image: d(m[1] || '')})]
];

// we import the root layout/error components eagerly, so that
// connectivity errors after initialisation don't nuke the app
export const fallback = [c[0](), c[1]()];