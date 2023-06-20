/** @type {import("../../../../src/index").RspackOptions} */
module.exports = {
	entry: {
		main: "./index"
	},
	target: "web",
	output: {
		filename: "[name].js",
		chunkFilename: "chunks/async-[name].js"
	}
};
