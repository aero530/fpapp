export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set([]),
	mimeTypes: {},
	_: {
		client: {"start":"_app/immutable/entry/start.D41QozNZ.js","app":"_app/immutable/entry/app.DZ5yhR8b.js","imports":["_app/immutable/entry/start.D41QozNZ.js","_app/immutable/chunks/entry.0Oz5ydGr.js","_app/immutable/chunks/scheduler.FmnKNk4x.js","_app/immutable/chunks/index.BePyX5ce.js","_app/immutable/entry/app.DZ5yhR8b.js","_app/immutable/chunks/scheduler.FmnKNk4x.js","_app/immutable/chunks/index.BIQaBuwX.js"],"stylesheets":[],"fonts":[],"uses_env_dynamic_public":false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
