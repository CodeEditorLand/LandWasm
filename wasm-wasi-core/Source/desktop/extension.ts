/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
import { ExtensionContext, workspace } from "vscode";

import { APILoader } from "../common/api";
import { NodeWasiProcess } from "./process";
import RIL from "./ril";

RIL.install();

export async function activate(context: ExtensionContext) {
	return new APILoader(context, NodeWasiProcess, async (source) => {
		const bits = await workspace.fs.readFile(source);

		return WebAssembly.compile(bits);
	});
}

export function deactivate() {}
