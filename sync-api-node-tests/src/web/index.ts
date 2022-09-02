/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import assert from 'assert';

require('mocha/mocha');

export function run(): Promise<void> {
	return new Promise((resolve, reject) => {
		// debugger;
		// Create the mocha test
		mocha.setup({
			ui: 'tdd',
			color: true,
			reporter: undefined
		});

		// // @ts-ignore
		// fetch('http://localhost:3000/static/devextensions/dist/workers/fileStat.js').then((content) => {
		// 	console.log(content);
		// }).catch(console.error);

		require('./all.test');
		suite('Simple', () => {
			test('One', () => {
				assert.strictEqual(1, 1);
			});
		});

		try {
			// Run the mocha test
			mocha.run(failures => {
				if (failures > 0) {
					reject(new Error(`${failures} tests failed.`));
				} else {
					resolve();
				}
			});
		} catch (error) {
			console.error(error);
			reject(error);
		}
	});
}