/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import * as _path from "path";

import runSingle from "./tests";

const path = _path.posix;

runSingle((client, folder) => {
	const dirname = folder.uri.with({
		path: path.join(folder.uri.path, "directory_new"),
	});

	client.vscode.workspace.fileSystem.delete(dirname, { recursive: true });
}).catch(console.error);
