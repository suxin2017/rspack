const { join } = require("path");

/** @type {import("../../../../").Configuration} */
module.exports = {
	target: "web",
	mode: "development",
	// output: {
	// 	path: join(__dirname, "dist")
	// },
	experiments: {
		css: true
	},
	optimization: {
		splitChunks: {
			cacheGroups: {
				css: {
					type: "css",
					enforce: true,
					name: "css"
				}
			}
		}
	},
	externalsPresets: {
		node: true
	},
	node: {
		__dirname: false
	}
};
