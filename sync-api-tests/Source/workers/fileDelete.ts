/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import * as _path from "path";

import runSingle from "./tests";

const path = _path.posix;

runSingle((client, folder) => {
	const filename = folder.uri.with({
		path: path.join(folder.uri.path, "toDelete.txt"),
	});

	client.vscode.workspace.fileSystem.delete(filename);
}).catch(console.error);
