const testCase = (tagName, impFn) => {
	it(`should be able to handle styles in ${tagName}.css`, done => {
		const element = document.createElement(tagName);
		document.body.appendChild(element);
		impFn().then(x => {
			try {
				// expect(x).toEqual(nsObj({}));
				expect(x).toEqual({ default: {} });
				const style = getComputedStyle(element);
				expect(style).toMatchSnapshot();
				done();
			} catch (e) {
				done(e);
			}
		}, done);
	});
};

testCase("div", () => import("./spacing.css"));
