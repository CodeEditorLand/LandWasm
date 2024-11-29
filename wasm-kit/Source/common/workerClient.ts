/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
import { BaseConnection, type AnyConnection } from "./connection";
import RAL from "./ral";
import type { SharedMemory } from "./sharedObject";
import type * as Messages from "./workerMessages";

export interface WorkerClient {
	launch(memory: SharedMemory): Promise<void>;

	terminate(): Promise<number>;
}
export namespace WorkerClient {
	export type ConnectionType = BaseConnection<
		Messages.Client.AsyncCalls,
		undefined,
		undefined,
		undefined,
		undefined,
		Messages.Service.Notifications
	>;
}
export const WorkerClient = RAL().WorkerClient;

export abstract class WorkerClientBase {
	protected setConnection(_connection: AnyConnection): void {
		throw new Error(`Must be implemented in subclass`);
	}
}
