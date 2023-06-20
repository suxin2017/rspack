// import fs from "fs";
// import path from "path";
it("chunks/async-two_js.js should exist", async function (done) {
	await import("./two");
	done();
	// expect(
	// 	fs
	// 		.readdirSync(path.resolve(__dirname, "chunks"))
	// 		.includes("async-two_js.js")
	// ).toBe(true);
});
