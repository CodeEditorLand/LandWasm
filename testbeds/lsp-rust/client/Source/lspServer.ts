/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import { Readable, WasmProcess, Writable } from "@vscode/wasm-wasi";
import {
	Disposable,
	Emitter,
	Event,
	Message,
	MessageTransports,
	RAL,
	ReadableStreamMessageReader,
	WriteableStreamMessageWriter,
} from "vscode-languageclient";

class ReadableStreamImpl implements RAL.ReadableStream {
	private readonly errorEmitter: Emitter<
		[Error, Message | undefined, number | undefined]
	>;

	private readonly closeEmitter: Emitter<void>;

	private readonly endEmitter: Emitter<void>;

	private readonly readable: Readable;

	constructor(readable: Readable) {
		this.errorEmitter = new Emitter<[Error, Message, number]>();

		this.closeEmitter = new Emitter<void>();

		this.endEmitter = new Emitter<void>();

		this.readable = readable;
	}

	public get onData(): Event<Uint8Array> {
		return this.readable.onData;
	}

	public get onError(): Event<
		[Error, Message | undefined, number | undefined]
	> {
		return this.errorEmitter.event;
	}

	public fireError(error: any, message?: Message, count?: number): void {
		this.errorEmitter.fire([error, message, count]);
	}

	public get onClose(): Event<void> {
		return this.closeEmitter.event;
	}

	public fireClose(): void {
		this.closeEmitter.fire(undefined);
	}

	public onEnd(listener: () => void): Disposable {
		return this.endEmitter.event(listener);
	}

	public fireEnd(): void {
		this.endEmitter.fire(undefined);
	}
}

type MessageBufferEncoding = RAL.MessageBufferEncoding;

class WritableStreamImpl implements RAL.WritableStream {
	private readonly errorEmitter: Emitter<
		[Error, Message | undefined, number | undefined]
	>;

	private readonly closeEmitter: Emitter<void>;

	private readonly endEmitter: Emitter<void>;

	private readonly writable: Writable;

	constructor(writable: Writable) {
		this.errorEmitter = new Emitter<[Error, Message, number]>();

		this.closeEmitter = new Emitter<void>();

		this.endEmitter = new Emitter<void>();

		this.writable = writable;
	}

	public get onError(): Event<
		[Error, Message | undefined, number | undefined]
	> {
		return this.errorEmitter.event;
	}

	public fireError(error: any, message?: Message, count?: number): void {
		this.errorEmitter.fire([error, message, count]);
	}

	public get onClose(): Event<void> {
		return this.closeEmitter.event;
	}

	public fireClose(): void {
		this.closeEmitter.fire(undefined);
	}

	public onEnd(listener: () => void): Disposable {
		return this.endEmitter.event(listener);
	}

	public fireEnd(): void {
		this.endEmitter.fire(undefined);
	}

	public write(
		data: string | Uint8Array,
		_encoding?: MessageBufferEncoding,
	): Promise<void> {
		if (typeof data === "string") {
			return this.writable.write(data, "utf-8");
		} else {
			return this.writable.write(data);
		}
	}

	public end(): void {}
}

export async function runServerProcess(
	process: WasmProcess,
	readable: Readable | undefined = process.stdout,
	writable: Writable | undefined = process.stdin,
): Promise<MessageTransports> {
	if (readable === undefined || writable === undefined) {
		throw new Error(
			"Process created without streams or no streams provided.",
		);
	}

	const reader = new ReadableStreamImpl(readable);

	const writer = new WritableStreamImpl(writable);

	process.run().then(
		(value) => {
			if (value === 0) {
				reader.fireEnd();
			} else {
				reader.fireError([
					new Error(`Process exited with code: ${value}`),
					undefined,
					undefined,
				]);
			}
		},
		(error) => {
			reader.fireError([error, undefined, undefined]);
		},
	);

	return {
		reader: new ReadableStreamMessageReader(reader),
		writer: new WriteableStreamMessageWriter(writer),
		detached: false,
	};
}
