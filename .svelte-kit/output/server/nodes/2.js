

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/2.DY9YOwme.js","_app/immutable/chunks/scheduler.FmnKNk4x.js","_app/immutable/chunks/index.BIQaBuwX.js","_app/immutable/chunks/index.BePyX5ce.js"];
export const stylesheets = ["_app/immutable/assets/2.CyVznJpm.css"];
export const fonts = [];
