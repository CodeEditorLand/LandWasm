// ---------------------------------------------------------------------------------------------
//  Copyright (c) Microsoft Corporation. All rights reserved.
//  Licensed under the MIT License. See License.txt in the project root for
// license information.
// --------------------------------------------------------------------------------------------
use std::{error::Error, fs, path::Path};

use lsp_server::{Connection, ExtractError, Message, RequestId, Response};
use lsp_types::{
	GotoDefinitionResponse,
	InitializeParams,
	Location,
	OneOf,
	ServerCapabilities,
	request::{GotoDefinition, Request},
};
use serde::{Deserialize, Serialize};
// use walkdir::WalkDir;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CountFilesParams {
	pub dir:String,
}

pub enum CountFilesRequest {}
impl Request for CountFilesRequest {
	type Params = CountFilesParams;
	type Result = u32;

	const METHOD:&'static str = "wasm-language-server/countFilesInDirectory";
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
	// Note that  we must have our logging only write out to stderr.
	eprintln!("Starting WASM based LSP server");

	// Create the transport. Includes the stdio (stdin and stdout) versions but
	// this could also be implemented to use sockets or HTTP.
	let (connection, io_threads) = Connection::stdio();

	// Run the server and wait for the two threads to end (typically by trigger
	// LSP Exit event).
	let server_capabilities = serde_json::to_value(&ServerCapabilities {
		definition_provider:Some(OneOf::Left(true)),
		..Default::default()
	})
	.unwrap();

	let initialization_params = connection.initialize(server_capabilities)?;

	main_loop(connection, initialization_params)?;

	io_threads.join()?;

	// Shut down gracefully.
	eprintln!("Shutting down server");

	Ok(())
}

fn main_loop(
	connection:Connection,
	params:serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
	eprintln!("Main loop");

	eprintln!("Open file");

	let file = fs::File::open("/workspace/package2.json").expect("Something went wrong"); // read_to_string("/workspace/package.json").expect("Something went wrong reading the file");

	eprintln!("File is open");
	// eprintln!("Content:\n{}", contents);

	let _params:InitializeParams = serde_json::from_value(params).unwrap();

	for msg in &connection.receiver {
		match msg {
			Message::Request(req) => {
				if connection.handle_shutdown(&req)? {
					return Ok(());
				}

				match cast::<GotoDefinition>(req.clone()) {
					Ok((id, params)) => {
						eprintln!("Received gotoDefinition request #{id}");

						let loc = Location::new(
							params.text_document_position_params.text_document.uri,
							lsp_types::Range::new(
								lsp_types::Position::new(0, 0),
								lsp_types::Position::new(0, 0),
							),
						);

						let mut vec = Vec::new();

						vec.push(loc);

						let result = Some(GotoDefinitionResponse::Array(vec));

						let result = serde_json::to_value(&result).unwrap();

						let resp = Response { id, result:Some(result), error:None };

						connection.sender.send(Message::Response(resp))?;

						continue;
					},
					Err(err @ ExtractError::JsonError { .. }) => {
						panic!("{err:?}")
					},
					Err(ExtractError::MethodMismatch(req)) => req,
				};

				match cast::<CountFilesRequest>(req.clone()) {
					Ok((id, params)) => {
						eprintln!("Received count request request #{id}");

						list_directory_contents(params.dir.as_str());

						let result = 10;

						eprintln!("Number of files #{result}");

						let result = serde_json::to_value(&result).unwrap();

						let resp = Response { id, result:Some(result), error:None };

						connection.sender.send(Message::Response(resp))?;

						continue;
					},
					Err(err @ ExtractError::JsonError { .. }) => {
						panic!("{err:?}")
					},
					Err(ExtractError::MethodMismatch(req)) => req,
				};
			},
			Message::Response(resp) => {
				eprintln!("got response: {resp:?}");
			},
			Message::Notification(not) => {
				eprintln!("Received notification: {not:?}");
			},
		}
	}

	Ok(())
}

fn cast<R>(
	req:lsp_server::Request,
) -> Result<(RequestId, R::Params), ExtractError<lsp_server::Request>>
where
	R: lsp_types::request::Request,
	R::Params: serde::de::DeserializeOwned, {
	req.extract(R::METHOD)
}

// fn count_files_in_directory(path: &str) -> usize {
//     let result = WalkDir::new(path)
//         .into_iter()
//         .filter_map(Result::ok)
//         .filter(|entry| entry.file_type().is_file())
//         .count();
//     return result;
// }

fn list_directory_contents<P:AsRef<Path>>(path:P) {
	eprintln!("In list directory {}", path.as_ref().display());

	match fs::read_dir(path) {
		Ok(entries) => {
			for entry in entries {
				if let Ok(entry) = entry {
					eprintln!("{}", entry.path().display());
				}
			}
		},
		Err(e) => {
			eprintln!("Failed to list directory: {}", e);
		},
	}
}
