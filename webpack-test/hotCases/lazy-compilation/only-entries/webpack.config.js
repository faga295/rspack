"use strict";

/** @type {import("@rspack/core").Configuration} */
module.exports = {
	experiments: {
		lazyCompilation: {
			entries: false,
			imports: false
		}
	}
};
