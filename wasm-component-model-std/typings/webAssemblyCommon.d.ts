/*! *****************************************************************************
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0

THIS CODE IS PROVIDED ON AN *AS IS* BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, EITHER EXPRESS OR IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED
WARRANTIES OR CONDITIONS OF TITLE, FITNESS FOR A PARTICULAR PURPOSE,
MERCHANTABLITY OR NON-INFRINGEMENT.

See the Apache Version 2.0 License for specific language governing permissions
and limitations under the License.
***************************************************************************** */

declare namespace WebAssembly {

    type ImportExportKind = 'function' | 'global' | 'memory' | 'table';
    type TableKind = 'anyfunc' | 'externref';
    type ValueType = 'anyfunc' | 'externref' | 'f32' | 'f64' | 'i32' | 'i64' | 'v128';
    type ExportValue = Function | Global | Memory | Table;
    type Exports = Record<string, ExportValue>;
    type ImportValue = ExportValue | number;
    type Imports = Record<string, ModuleImports>;
    type ModuleImports = Record<string, ImportValue>;

    interface Table {
    	readonly length: number;
    	get(index: number): any;
    	grow(delta: number, value?: any): number;
    	set(index: number, value?: any): void;
    }

    interface TableDescriptor {
    	element: TableKind;
    	initial: number;
    	maximum?: number;
    }

    var Table: {
    	prototype: Table;
    	new(descriptor: TableDescriptor, value?: any): Table;
    };

    interface Global {
    	value: any;
    	valueOf(): any;
    }

    interface GlobalDescriptor {
    	mutable?: boolean;
    	value: ValueType;
    }

    var Global: {
    	prototype: Global;
    	new(descriptor: GlobalDescriptor, v?: any): Global;
    };

    interface CompileError extends Error {
    }

    var CompileError: {
    	prototype: CompileError;
    	new(message?: string): CompileError;
    	(message?: string): CompileError;
    };

    interface LinkError extends Error {
    }

    var LinkError: {
    	prototype: LinkError;
    	new(message?: string): LinkError;
    	(message?: string): LinkError;
    };

    interface MemoryDescriptor {
    	initial: number;
    	maximum?: number;
    	shared?: boolean;
    }

    interface Memory {
    	readonly buffer: ArrayBuffer;
    	grow(delta: number): number;
    }

    var Memory: {
    	prototype: Memory;
    	new(descriptor: MemoryDescriptor): Memory;
    };

    interface Module {
    }

    interface RuntimeError extends Error {
    }

    var RuntimeError: {
    	prototype: RuntimeError;
    	new(message?: string): RuntimeError;
    	(message?: string): RuntimeError;
    };

    interface Instance {
    	readonly exports: Exports;
    }

    var Instance: {
    	prototype: Instance;
    	new(module: Module, importObject?: Imports): Instance;
    };
}