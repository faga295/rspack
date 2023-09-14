/** @type {import('@rspack/core').Configuration} */
module.exports = {
	module: {
		rules: [
			{
				test: /\.js$/,
				use: ["./test-loader.js"]
			}
		]
	},
	stats: {
		all: false,
		loggingDebug: [/TestLoader/]
	}
};
