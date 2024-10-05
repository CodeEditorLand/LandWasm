// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
pub mod host {
	pub mod api {
		#[allow(clippy::all)]
		pub mod types {
			#[used]
			#[doc(hidden)]
			#[cfg(target_arch = "wasm32")]
			static __FORCE_SECTION_REF:fn() =
				super::super::super::__link_custom_section_describing_imports;
			use super::super::super::_rt;
			#[repr(C)]
			#[derive(Clone, Copy)]
			pub struct Position {
				pub line:u32,
				pub character:u32,
			}
			impl ::core::fmt::Debug for Position {
				fn fmt(
					&self,
					f:&mut ::core::fmt::Formatter<'_>,
				) -> ::core::fmt::Result {
					f.debug_struct("Position")
						.field("line", &self.line)
						.field("character", &self.character)
						.finish()
				}
			}
			#[repr(C)]
			#[derive(Clone, Copy)]
			pub struct Range {
				pub start:Position,
				pub end:Position,
			}
			impl ::core::fmt::Debug for Range {
				fn fmt(
					&self,
					f:&mut ::core::fmt::Formatter<'_>,
				) -> ::core::fmt::Result {
					f.debug_struct("Range")
						.field("start", &self.start)
						.field("end", &self.end)
						.finish()
				}
			}
			#[derive(Clone)]
			pub struct TextDocumentContentChangeEvent {
				pub range:Range,
				pub range_offset:u32,
				pub range_length:u32,
				pub text:_rt::String,
			}
			impl ::core::fmt::Debug for TextDocumentContentChangeEvent {
				fn fmt(
					&self,
					f:&mut ::core::fmt::Formatter<'_>,
				) -> ::core::fmt::Result {
					f.debug_struct("TextDocumentContentChangeEvent")
						.field("range", &self.range)
						.field("range-offset", &self.range_offset)
						.field("range-length", &self.range_length)
						.field("text", &self.text)
						.finish()
				}
			}

			#[derive(Debug)]
			#[repr(transparent)]
			pub struct TextDocument {
				handle:_rt::Resource<TextDocument>,
			}

			impl TextDocument {
				#[doc(hidden)]
				pub unsafe fn from_handle(handle:u32) -> Self {
					Self { handle:_rt::Resource::from_handle(handle) }
				}

				#[doc(hidden)]
				pub fn take_handle(&self) -> u32 {
					_rt::Resource::take_handle(&self.handle)
				}

				#[doc(hidden)]
				pub fn handle(&self) -> u32 {
					_rt::Resource::handle(&self.handle)
				}
			}

			unsafe impl _rt::WasmResource for TextDocument {
				#[inline]
				unsafe fn drop(_handle:u32) {
					#[cfg(not(target_arch = "wasm32"))]
					unreachable!();

					#[cfg(target_arch = "wasm32")]
					{
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[resource-drop]text-document"]
							fn drop(_:u32);
						}

						drop(_handle);
					}
				}
			}

			#[repr(u8)]
			#[derive(Clone, Copy, Eq, PartialEq)]
			pub enum TextDocumentChangeReason {
				Undo,
				Redo,
			}
			impl ::core::fmt::Debug for TextDocumentChangeReason {
				fn fmt(
					&self,
					f:&mut ::core::fmt::Formatter<'_>,
				) -> ::core::fmt::Result {
					match self {
						TextDocumentChangeReason::Undo => {
							f.debug_tuple("TextDocumentChangeReason::Undo")
								.finish()
						},
						TextDocumentChangeReason::Redo => {
							f.debug_tuple("TextDocumentChangeReason::Redo")
								.finish()
						},
					}
				}
			}

			impl TextDocumentChangeReason {
				pub(crate) unsafe fn _lift(val:u8) -> TextDocumentChangeReason {
					if !cfg!(debug_assertions) {
						return ::core::mem::transmute(val);
					}

					match val {
						0 => TextDocumentChangeReason::Undo,
						1 => TextDocumentChangeReason::Redo,

						_ => panic!("invalid enum discriminant"),
					}
				}
			}

			pub struct TextDocumentChangeEvent {
				pub document:TextDocument,
				pub content_changes:_rt::Vec<TextDocumentContentChangeEvent>,
				pub reason:Option<TextDocumentChangeReason>,
			}
			impl ::core::fmt::Debug for TextDocumentChangeEvent {
				fn fmt(
					&self,
					f:&mut ::core::fmt::Formatter<'_>,
				) -> ::core::fmt::Result {
					f.debug_struct("TextDocumentChangeEvent")
						.field("document", &self.document)
						.field("content-changes", &self.content_changes)
						.field("reason", &self.reason)
						.finish()
				}
			}

			#[derive(Debug)]
			#[repr(transparent)]
			pub struct OutputChannel {
				handle:_rt::Resource<OutputChannel>,
			}

			impl OutputChannel {
				#[doc(hidden)]
				pub unsafe fn from_handle(handle:u32) -> Self {
					Self { handle:_rt::Resource::from_handle(handle) }
				}

				#[doc(hidden)]
				pub fn take_handle(&self) -> u32 {
					_rt::Resource::take_handle(&self.handle)
				}

				#[doc(hidden)]
				pub fn handle(&self) -> u32 {
					_rt::Resource::handle(&self.handle)
				}
			}

			unsafe impl _rt::WasmResource for OutputChannel {
				#[inline]
				unsafe fn drop(_handle:u32) {
					#[cfg(not(target_arch = "wasm32"))]
					unreachable!();

					#[cfg(target_arch = "wasm32")]
					{
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[resource-drop]output-channel"]
							fn drop(_:u32);
						}

						drop(_handle);
					}
				}
			}

			#[derive(Clone)]
			pub enum GlobPattern {
				Pattern(_rt::String),
			}
			impl ::core::fmt::Debug for GlobPattern {
				fn fmt(
					&self,
					f:&mut ::core::fmt::Formatter<'_>,
				) -> ::core::fmt::Result {
					match self {
						GlobPattern::Pattern(e) => {
							f.debug_tuple("GlobPattern::Pattern")
								.field(e)
								.finish()
						},
					}
				}
			}
			#[derive(Clone)]
			pub struct DocumentFilter {
				pub language:Option<_rt::String>,
				pub scheme:Option<_rt::String>,
				pub notebook_type:Option<_rt::String>,
				pub pattern:Option<GlobPattern>,
			}
			impl ::core::fmt::Debug for DocumentFilter {
				fn fmt(
					&self,
					f:&mut ::core::fmt::Formatter<'_>,
				) -> ::core::fmt::Result {
					f.debug_struct("DocumentFilter")
						.field("language", &self.language)
						.field("scheme", &self.scheme)
						.field("notebook-type", &self.notebook_type)
						.field("pattern", &self.pattern)
						.finish()
				}
			}
			#[derive(Clone)]
			pub enum DocumentSelector {
				Many(_rt::Vec<DocumentFilter>),
				Single(DocumentFilter),
			}
			impl ::core::fmt::Debug for DocumentSelector {
				fn fmt(
					&self,
					f:&mut ::core::fmt::Formatter<'_>,
				) -> ::core::fmt::Result {
					match self {
						DocumentSelector::Many(e) => {
							f.debug_tuple("DocumentSelector::Many")
								.field(e)
								.finish()
						},
						DocumentSelector::Single(e) => {
							f.debug_tuple("DocumentSelector::Single")
								.field(e)
								.finish()
						},
					}
				}
			}
			impl TextDocument {
				#[allow(unused_unsafe, clippy::all)]
				pub fn uri(&self) -> _rt::String {
					unsafe {
						#[repr(align(4))]
						struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
						let mut ret_area =
							RetArea([::core::mem::MaybeUninit::uninit(); 8]);
						let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]text-document.uri"]
							fn wit_import(_:i32, _:*mut u8);
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32, _:*mut u8) { unreachable!() }
						wit_import((self).handle() as i32, ptr0);
						let l1 = *ptr0.add(0).cast::<*mut u8>();
						let l2 = *ptr0.add(4).cast::<usize>();
						let len3 = l2;
						let bytes3 =
							_rt::Vec::from_raw_parts(l1.cast(), len3, len3);
						_rt::string_lift(bytes3)
					}
				}
			}
			impl TextDocument {
				#[allow(unused_unsafe, clippy::all)]
				pub fn language_id(&self) -> _rt::String {
					unsafe {
						#[repr(align(4))]
						struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
						let mut ret_area =
							RetArea([::core::mem::MaybeUninit::uninit(); 8]);
						let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]text-document.language-id"]
							fn wit_import(_:i32, _:*mut u8);
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32, _:*mut u8) { unreachable!() }
						wit_import((self).handle() as i32, ptr0);
						let l1 = *ptr0.add(0).cast::<*mut u8>();
						let l2 = *ptr0.add(4).cast::<usize>();
						let len3 = l2;
						let bytes3 =
							_rt::Vec::from_raw_parts(l1.cast(), len3, len3);
						_rt::string_lift(bytes3)
					}
				}
			}
			impl TextDocument {
				#[allow(unused_unsafe, clippy::all)]
				pub fn version(&self) -> u32 {
					unsafe {
						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]text-document.version"]
							fn wit_import(_:i32) -> i32;
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32) -> i32 { unreachable!() }
						let ret = wit_import((self).handle() as i32);
						ret as u32
					}
				}
			}
			impl TextDocument {
				#[allow(unused_unsafe, clippy::all)]
				pub fn get_text(&self) -> _rt::String {
					unsafe {
						#[repr(align(4))]
						struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
						let mut ret_area =
							RetArea([::core::mem::MaybeUninit::uninit(); 8]);
						let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]text-document.get-text"]
							fn wit_import(_:i32, _:*mut u8);
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32, _:*mut u8) { unreachable!() }
						wit_import((self).handle() as i32, ptr0);
						let l1 = *ptr0.add(0).cast::<*mut u8>();
						let l2 = *ptr0.add(4).cast::<usize>();
						let len3 = l2;
						let bytes3 =
							_rt::Vec::from_raw_parts(l1.cast(), len3, len3);
						_rt::string_lift(bytes3)
					}
				}
			}
			impl OutputChannel {
				#[allow(unused_unsafe, clippy::all)]
				pub fn name(&self) -> _rt::String {
					unsafe {
						#[repr(align(4))]
						struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
						let mut ret_area =
							RetArea([::core::mem::MaybeUninit::uninit(); 8]);
						let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]output-channel.name"]
							fn wit_import(_:i32, _:*mut u8);
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32, _:*mut u8) { unreachable!() }
						wit_import((self).handle() as i32, ptr0);
						let l1 = *ptr0.add(0).cast::<*mut u8>();
						let l2 = *ptr0.add(4).cast::<usize>();
						let len3 = l2;
						let bytes3 =
							_rt::Vec::from_raw_parts(l1.cast(), len3, len3);
						_rt::string_lift(bytes3)
					}
				}
			}
			impl OutputChannel {
				#[allow(unused_unsafe, clippy::all)]
				pub fn append(&self, value:&str) {
					unsafe {
						let vec0 = value;
						let ptr0 = vec0.as_ptr().cast::<u8>();
						let len0 = vec0.len();

						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]output-channel.append"]
							fn wit_import(_:i32, _:*mut u8, _:usize);
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32, _:*mut u8, _:usize) {
							unreachable!()
						}
						wit_import(
							(self).handle() as i32,
							ptr0.cast_mut(),
							len0,
						);
					}
				}
			}
			impl OutputChannel {
				#[allow(unused_unsafe, clippy::all)]
				pub fn append_line(&self, value:&str) {
					unsafe {
						let vec0 = value;
						let ptr0 = vec0.as_ptr().cast::<u8>();
						let len0 = vec0.len();

						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]output-channel.append-line"]
							fn wit_import(_:i32, _:*mut u8, _:usize);
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32, _:*mut u8, _:usize) {
							unreachable!()
						}
						wit_import(
							(self).handle() as i32,
							ptr0.cast_mut(),
							len0,
						);
					}
				}
			}
			impl OutputChannel {
				#[allow(unused_unsafe, clippy::all)]
				pub fn clear(&self) {
					unsafe {
						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]output-channel.clear"]
							fn wit_import(_:i32);
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32) { unreachable!() }
						wit_import((self).handle() as i32);
					}
				}
			}
			impl OutputChannel {
				#[allow(unused_unsafe, clippy::all)]
				pub fn show(&self) {
					unsafe {
						#[cfg(target_arch = "wasm32")]
						#[link(wasm_import_module = "host:api/types")]
						extern {
							#[link_name = "[method]output-channel.show"]
							fn wit_import(_:i32);
						}

						#[cfg(not(target_arch = "wasm32"))]
						fn wit_import(_:i32) { unreachable!() }
						wit_import((self).handle() as i32);
					}
				}
			}
		}

		#[allow(clippy::all)]
		pub mod workspace {
			#[used]
			#[doc(hidden)]
			#[cfg(target_arch = "wasm32")]
			static __FORCE_SECTION_REF:fn() =
				super::super::super::__link_custom_section_describing_imports;
			use super::super::super::_rt;
			pub type TextDocument =
				super::super::super::host::api::types::TextDocument;
			#[allow(unused_unsafe, clippy::all)]
			pub fn text_documents() -> _rt::Vec<TextDocument> {
				unsafe {
					#[repr(align(4))]
					struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
					let mut ret_area =
						RetArea([::core::mem::MaybeUninit::uninit(); 8]);
					let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
					#[cfg(target_arch = "wasm32")]
					#[link(wasm_import_module = "host:api/workspace")]
					extern {
						#[link_name = "text-documents"]
						fn wit_import(_:*mut u8);
					}

					#[cfg(not(target_arch = "wasm32"))]
					fn wit_import(_:*mut u8) { unreachable!() }
					wit_import(ptr0);
					let l1 = *ptr0.add(0).cast::<*mut u8>();
					let l2 = *ptr0.add(4).cast::<usize>();
					let base4 = l1;
					let len4 = l2;
					let mut result4 = _rt::Vec::with_capacity(len4);
					for i in 0..len4 {
						let base = base4.add(i * 4);
						let e4 = {
							let l3 = *base.add(0).cast::<i32>();

							super::super::super::host::api::types::TextDocument::from_handle(l3 as u32)
						};
						result4.push(e4);
					}
					_rt::cabi_dealloc(base4, len4 * 4, 4);
					result4
				}
			}
			#[allow(unused_unsafe, clippy::all)]
			pub fn register_on_did_change_text_document() {
				unsafe {
					#[cfg(target_arch = "wasm32")]
					#[link(wasm_import_module = "host:api/workspace")]
					extern {
						#[link_name = "register-on-did-change-text-document"]
						fn wit_import();
					}

					#[cfg(not(target_arch = "wasm32"))]
					fn wit_import() { unreachable!() }
					wit_import();
				}
			}
			#[allow(unused_unsafe, clippy::all)]
			pub fn unregister_on_did_change_text_document() {
				unsafe {
					#[cfg(target_arch = "wasm32")]
					#[link(wasm_import_module = "host:api/workspace")]
					extern {
						#[link_name = "unregister-on-did-change-text-document"]
						fn wit_import();
					}

					#[cfg(not(target_arch = "wasm32"))]
					fn wit_import() { unreachable!() }
					wit_import();
				}
			}
		}

		#[allow(clippy::all)]
		pub mod commands {
			#[used]
			#[doc(hidden)]
			#[cfg(target_arch = "wasm32")]
			static __FORCE_SECTION_REF:fn() =
				super::super::super::__link_custom_section_describing_imports;
			#[allow(unused_unsafe, clippy::all)]
			pub fn register_command(command:&str) {
				unsafe {
					let vec0 = command;
					let ptr0 = vec0.as_ptr().cast::<u8>();
					let len0 = vec0.len();

					#[cfg(target_arch = "wasm32")]
					#[link(wasm_import_module = "host:api/commands")]
					extern {
						#[link_name = "register-command"]
						fn wit_import(_:*mut u8, _:usize);
					}

					#[cfg(not(target_arch = "wasm32"))]
					fn wit_import(_:*mut u8, _:usize) { unreachable!() }
					wit_import(ptr0.cast_mut(), len0);
				}
			}
			#[allow(unused_unsafe, clippy::all)]
			pub fn unregister_command(command:&str) {
				unsafe {
					let vec0 = command;
					let ptr0 = vec0.as_ptr().cast::<u8>();
					let len0 = vec0.len();

					#[cfg(target_arch = "wasm32")]
					#[link(wasm_import_module = "host:api/commands")]
					extern {
						#[link_name = "unregister-command"]
						fn wit_import(_:*mut u8, _:usize);
					}

					#[cfg(not(target_arch = "wasm32"))]
					fn wit_import(_:*mut u8, _:usize) { unreachable!() }
					wit_import(ptr0.cast_mut(), len0);
				}
			}
		}

		#[allow(clippy::all)]
		pub mod window {
			#[used]
			#[doc(hidden)]
			#[cfg(target_arch = "wasm32")]
			static __FORCE_SECTION_REF:fn() =
				super::super::super::__link_custom_section_describing_imports;
			pub type OutputChannel =
				super::super::super::host::api::types::OutputChannel;
			#[allow(unused_unsafe, clippy::all)]
			pub fn create_output_channel(
				name:&str,
				language_id:Option<&str>,
			) -> OutputChannel {
				unsafe {
					let vec0 = name;
					let ptr0 = vec0.as_ptr().cast::<u8>();
					let len0 = vec0.len();
					let (result2_0, result2_1, result2_2) = match language_id {
						Some(e) => {
							let vec1 = e;
							let ptr1 = vec1.as_ptr().cast::<u8>();
							let len1 = vec1.len();

							(1i32, ptr1.cast_mut(), len1)
						},
						None => (0i32, ::core::ptr::null_mut(), 0usize),
					};
					#[cfg(target_arch = "wasm32")]
					#[link(wasm_import_module = "host:api/window")]
					extern {
						#[link_name = "create-output-channel"]
						fn wit_import(
							_:*mut u8,
							_:usize,
							_:i32,
							_:*mut u8,
							_:usize,
						) -> i32;
					}

					#[cfg(not(target_arch = "wasm32"))]
					fn wit_import(
						_:*mut u8,
						_:usize,
						_:i32,
						_:*mut u8,
						_:usize,
					) -> i32 {
						unreachable!()
					}
					let ret = wit_import(
						ptr0.cast_mut(),
						len0,
						result2_0,
						result2_1,
						result2_2,
					);
					super::super::super::host::api::types::OutputChannel::from_handle(ret as u32)
				}
			}
		}

		#[allow(clippy::all)]
		pub mod languages {
			#[used]
			#[doc(hidden)]
			#[cfg(target_arch = "wasm32")]
			static __FORCE_SECTION_REF:fn() =
				super::super::super::__link_custom_section_describing_imports;
			use super::super::super::_rt;
			pub type DocumentSelector =
				super::super::super::host::api::types::DocumentSelector;
			pub type TextDocument =
				super::super::super::host::api::types::TextDocument;
			#[allow(unused_unsafe, clippy::all)]
			pub fn match_selector(
				selector:&DocumentSelector,
				document:TextDocument,
			) -> u32 {
				unsafe {
					let mut cleanup_list = _rt::Vec::new();
					use super::super::super::host::api::types::DocumentSelector as V18;
					let (
						result19_0,
						result19_1,
						result19_2,
						result19_3,
						result19_4,
						result19_5,
						result19_6,
						result19_7,
						result19_8,
						result19_9,
						result19_10,
						result19_11,
						result19_12,
						result19_13,
					) = match selector {
						V18::Many(e) => {
							let vec6 = e;
							let len6 = vec6.len();
							let layout6 =
								_rt::alloc::Layout::from_size_align_unchecked(
									vec6.len() * 52,
									4,
								);
							let result6 = if layout6.size() != 0 {
								let ptr =
									_rt::alloc::alloc(layout6).cast::<u8>();
								if ptr.is_null() {
									_rt::alloc::handle_alloc_error(layout6);
								}
								ptr
							} else {
								{ ::core::ptr::null_mut() }
							};
							for (i, e) in vec6.into_iter().enumerate() {
								let base = result6.add(i * 52);
								{
									let super::super::super::host::api::types::DocumentFilter{ language:language0, scheme:scheme0, notebook_type:notebook_type0, pattern:pattern0, } = e;
									match language0 {
										Some(e) => {
											*base.add(0).cast::<u8>() =
												(1i32) as u8;
											let vec1 = e;
											let ptr1 =
												vec1.as_ptr().cast::<u8>();
											let len1 = vec1.len();
											*base.add(8).cast::<usize>() = len1;
											*base.add(4).cast::<*mut u8>() =
												ptr1.cast_mut();
										},
										None => {
											*base.add(0).cast::<u8>() =
												(0i32) as u8;
										},
									};
									match scheme0 {
										Some(e) => {
											*base.add(12).cast::<u8>() =
												(1i32) as u8;
											let vec2 = e;
											let ptr2 =
												vec2.as_ptr().cast::<u8>();
											let len2 = vec2.len();
											*base.add(20).cast::<usize>() =
												len2;
											*base.add(16).cast::<*mut u8>() =
												ptr2.cast_mut();
										},
										None => {
											*base.add(12).cast::<u8>() =
												(0i32) as u8;
										},
									};
									match notebook_type0 {
										Some(e) => {
											*base.add(24).cast::<u8>() =
												(1i32) as u8;
											let vec3 = e;
											let ptr3 =
												vec3.as_ptr().cast::<u8>();
											let len3 = vec3.len();
											*base.add(32).cast::<usize>() =
												len3;
											*base.add(28).cast::<*mut u8>() =
												ptr3.cast_mut();
										},
										None => {
											*base.add(24).cast::<u8>() =
												(0i32) as u8;
										},
									};
									match pattern0 {
										Some(e) => {
											*base.add(36).cast::<u8>() =
												(1i32) as u8;
											use super::super::super::host::api::types::GlobPattern as V5;
											match e {
												V5::Pattern(e) => {
													*base
														.add(40)
														.cast::<u8>() = (0i32) as u8;
													let vec4 = e;
													let ptr4 = vec4
														.as_ptr()
														.cast::<u8>();
													let len4 = vec4.len();
													*base
														.add(48)
														.cast::<usize>() = len4;
													*base
														.add(44)
														.cast::<*mut u8>() = ptr4.cast_mut();
												},
											}
										},
										None => {
											*base.add(36).cast::<u8>() =
												(0i32) as u8;
										},
									};
								}
							}
							cleanup_list
								.extend_from_slice(&[(result6, layout6)]);

							(
								0i32,
								result6,
								len6 as *mut u8,
								0usize,
								0i32,
								::core::ptr::null_mut(),
								0usize,
								0i32,
								::core::ptr::null_mut(),
								0usize,
								0i32,
								0i32,
								::core::ptr::null_mut(),
								0usize,
							)
						},
						V18::Single(e) => {
							let super::super::super::host::api::types::DocumentFilter{ language:language7, scheme:scheme7, notebook_type:notebook_type7, pattern:pattern7, } = e;
							let (result9_0, result9_1, result9_2) =
								match language7 {
									Some(e) => {
										let vec8 = e;
										let ptr8 = vec8.as_ptr().cast::<u8>();
										let len8 = vec8.len();

										(1i32, ptr8.cast_mut(), len8)
									},
									None => {
										(0i32, ::core::ptr::null_mut(), 0usize)
									},
								};
							let (result11_0, result11_1, result11_2) =
								match scheme7 {
									Some(e) => {
										let vec10 = e;
										let ptr10 = vec10.as_ptr().cast::<u8>();
										let len10 = vec10.len();

										(1i32, ptr10.cast_mut(), len10)
									},
									None => {
										(0i32, ::core::ptr::null_mut(), 0usize)
									},
								};
							let (result13_0, result13_1, result13_2) =
								match notebook_type7 {
									Some(e) => {
										let vec12 = e;
										let ptr12 = vec12.as_ptr().cast::<u8>();
										let len12 = vec12.len();

										(1i32, ptr12.cast_mut(), len12)
									},
									None => {
										(0i32, ::core::ptr::null_mut(), 0usize)
									},
								};
							let (
								result17_0,
								result17_1,
								result17_2,
								result17_3,
							) = match pattern7 {
								Some(e) => {
									use super::super::super::host::api::types::GlobPattern as V15;
									let (result16_0, result16_1, result16_2) =
										match e {
											V15::Pattern(e) => {
												let vec14 = e;
												let ptr14 =
													vec14.as_ptr().cast::<u8>();
												let len14 = vec14.len();

												(0i32, ptr14.cast_mut(), len14)
											},
										};

									(1i32, result16_0, result16_1, result16_2)
								},
								None => {
									(
										0i32,
										0i32,
										::core::ptr::null_mut(),
										0usize,
									)
								},
							};
							(
								1i32,
								result9_0 as *mut u8,
								result9_1,
								result9_2,
								result11_0,
								result11_1,
								result11_2,
								result13_0,
								result13_1,
								result13_2,
								result17_0,
								result17_1,
								result17_2,
								result17_3,
							)
						},
					};

					#[cfg(target_arch = "wasm32")]
					#[link(wasm_import_module = "host:api/languages")]
					extern {
						#[link_name = "match-selector"]
						fn wit_import(
							_:i32,
							_:*mut u8,
							_:*mut u8,
							_:usize,
							_:i32,
							_:*mut u8,
							_:usize,
							_:i32,
							_:*mut u8,
							_:usize,
							_:i32,
							_:i32,
							_:*mut u8,
							_:usize,
							_:i32,
						) -> i32;
					}

					#[cfg(not(target_arch = "wasm32"))]
					fn wit_import(
						_:i32,
						_:*mut u8,
						_:*mut u8,
						_:usize,
						_:i32,
						_:*mut u8,
						_:usize,
						_:i32,
						_:*mut u8,
						_:usize,
						_:i32,
						_:i32,
						_:*mut u8,
						_:usize,
						_:i32,
					) -> i32 {
						unreachable!()
					}
					let ret = wit_import(
						result19_0,
						result19_1,
						result19_2,
						result19_3,
						result19_4,
						result19_5,
						result19_6,
						result19_7,
						result19_8,
						result19_9,
						result19_10,
						result19_11,
						result19_12,
						result19_13,
						(&document).take_handle() as i32,
					);
					for (ptr, layout) in cleanup_list {
						if layout.size() != 0 {
							_rt::alloc::dealloc(ptr.cast(), layout);
						}
					}
					ret as u32
				}
			}
		}
	}
}
pub mod exports {
	pub mod host {
		pub mod api {
			#[allow(clippy::all)]
			pub mod callbacks {
				#[used]
				#[doc(hidden)]
				#[cfg(target_arch = "wasm32")]
				static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
				use super::super::super::super::_rt;
				pub type TextDocumentChangeEvent = super::super::super::super::host::api::types::TextDocumentChangeEvent;
				#[doc(hidden)]
				#[allow(non_snake_case)]
				pub unsafe fn _export_did_change_text_document_cabi<T:Guest>(
					arg0:i32,
					arg1:*mut u8,
					arg2:usize,
					arg3:i32,
					arg4:i32,
				) {
					let base9 = arg1;
					let len9 = arg2;
					let mut result9 = _rt::Vec::with_capacity(len9);
					for i in 0..len9 {
						let base = base9.add(i * 32);
						let e9 = {
							let l0 = *base.add(0).cast::<i32>();
							let l1 = *base.add(4).cast::<i32>();
							let l2 = *base.add(8).cast::<i32>();
							let l3 = *base.add(12).cast::<i32>();
							let l4 = *base.add(16).cast::<i32>();
							let l5 = *base.add(20).cast::<i32>();
							let l6 = *base.add(24).cast::<*mut u8>();
							let l7 = *base.add(28).cast::<usize>();
							let len8 = l7;
							let bytes8 =
								_rt::Vec::from_raw_parts(l6.cast(), len8, len8);

							super::super::super::super::host::api::types::TextDocumentContentChangeEvent{
                range: super::super::super::super::host::api::types::Range{
                  start: super::super::super::super::host::api::types::Position{
                    line: l0 as u32,
                    character: l1 as u32,
                  },
                  end: super::super::super::super::host::api::types::Position{
                    line: l2 as u32,
                    character: l3 as u32,
                  },
                },
                range_offset: l4 as u32,
                range_length: l5 as u32,
                text: _rt::string_lift(bytes8),
              }
						};
						result9.push(e9);
					}
					_rt::cabi_dealloc(base9, len9 * 32, 4);
					T::did_change_text_document(super::super::super::super::host::api::types::TextDocumentChangeEvent{
            document: super::super::super::super::host::api::types::TextDocument::from_handle(arg0 as u32),
            content_changes: result9,
            reason: match arg3 {
              0 => None,
              1 => {
                let e = super::super::super::super::host::api::types::TextDocumentChangeReason::_lift(arg4 as u8);
                Some(e)
              }
              _ => _rt::invalid_enum_discriminant(),
            },
          });
				}
				#[doc(hidden)]
				#[allow(non_snake_case)]
				pub unsafe fn _export_execute_command_cabi<T:Guest>(
					arg0:*mut u8,
					arg1:usize,
				) {
					let len0 = arg1;
					let bytes0 =
						_rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
					T::execute_command(_rt::string_lift(bytes0));
				}
				pub trait Guest {
					fn did_change_text_document(event:TextDocumentChangeEvent);
					fn execute_command(command:_rt::String);
				}
				#[doc(hidden)]

				macro_rules! __export_host_api_callbacks_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "host:api/callbacks#did-change-text-document"]
          unsafe extern "C" fn export_did_change_text_document(arg0: i32,arg1: *mut u8,arg2: usize,arg3: i32,arg4: i32,) {
            $($path_to_types)*::_export_did_change_text_document_cabi::<$ty>(arg0, arg1, arg2, arg3, arg4)
          }
          #[export_name = "host:api/callbacks#execute-command"]
          unsafe extern "C" fn export_execute_command(arg0: *mut u8,arg1: usize,) {
            $($path_to_types)*::_export_execute_command_cabi::<$ty>(arg0, arg1)
          }
        };);
      }
				#[doc(hidden)]
				pub(crate) use __export_host_api_callbacks_cabi;
			}
		}
	}
}
mod _rt {
	use core::{
		fmt,
		marker,
		sync::atomic::{AtomicU32, Ordering::Relaxed},
	};

	pub use alloc_crate::string::String;

	/// A type which represents a component model resource, either imported or
	/// exported into this component.
	///
	/// This is a low-level wrapper which handles the lifetime of the resource
	/// (namely this has a destructor). The `T` provided defines the component
	/// model intrinsics that this wrapper uses.
	///
	/// One of the chief purposes of this type is to provide `Deref`
	/// implementations to access the underlying data when it is owned.
	///
	/// This type is primarily used in generated code for exported and imported
	/// resources.
	#[repr(transparent)]
	pub struct Resource<T:WasmResource> {
		// NB: This would ideally be `u32` but it is not. The fact that this
		// has interior mutability is not exposed in the API of this type
		// except for the `take_handle` method which is supposed to in theory
		// be private.
		//
		// This represents, almost all the time, a valid handle value. When
		// it's invalid it's stored as `u32::MAX`.
		handle:AtomicU32,
		_marker:marker::PhantomData<T>,
	}

	/// A trait which all wasm resources implement, namely providing the ability
	/// to drop a resource.
	///
	/// This generally is implemented by generated code, not user-facing code.
	pub unsafe trait WasmResource {
		/// Invokes the `[resource-drop]...` intrinsic.
		unsafe fn drop(handle:u32);
	}

	impl<T:WasmResource> Resource<T> {
		#[doc(hidden)]
		pub unsafe fn from_handle(handle:u32) -> Self {
			debug_assert!(handle != u32::MAX);
			Self { handle:AtomicU32::new(handle), _marker:marker::PhantomData }
		}

		/// Takes ownership of the handle owned by `resource`.
		///
		/// Note that this ideally would be `into_handle` taking `Resource<T>`
		/// by ownership. The code generator does not enable that in all
		/// situations, unfortunately, so this is provided instead.
		///
		/// Also note that `take_handle` is in theory only ever called on values
		/// owned by a generated function. For example a generated function
		/// might take `Resource<T>` as an argument but then call
		/// `take_handle` on a reference to that argument. In that sense the
		/// dynamic nature of `take_handle` should only be exposed internally
		/// to generated code, not to user code.
		#[doc(hidden)]
		pub fn take_handle(resource:&Resource<T>) -> u32 {
			resource.handle.swap(u32::MAX, Relaxed)
		}

		#[doc(hidden)]
		pub fn handle(resource:&Resource<T>) -> u32 {
			resource.handle.load(Relaxed)
		}
	}

	impl<T:WasmResource> fmt::Debug for Resource<T> {
		fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Resource").field("handle", &self.handle).finish()
		}
	}

	impl<T:WasmResource> Drop for Resource<T> {
		fn drop(&mut self) {
			unsafe {
				match self.handle.load(Relaxed) {
					// If this handle was "taken" then don't do anything in the
					// destructor.
					u32::MAX => {},

					// ... but otherwise do actually destroy it with the
					// imported component model intrinsic as defined
					// through `T`.
					other => T::drop(other),
				}
			}
		}
	}
	pub use alloc_crate::vec::Vec;
	pub unsafe fn string_lift(bytes:Vec<u8>) -> String {
		if cfg!(debug_assertions) {
			String::from_utf8(bytes).unwrap()
		} else {
			String::from_utf8_unchecked(bytes)
		}
	}
	pub unsafe fn cabi_dealloc(ptr:*mut u8, size:usize, align:usize) {
		if size == 0 {
			return;
		}
		let layout = alloc::Layout::from_size_align_unchecked(size, align);
		alloc::dealloc(ptr as *mut u8, layout);
	}
	pub use alloc_crate::alloc;
	pub unsafe fn invalid_enum_discriminant<T>() -> T {
		if cfg!(debug_assertions) {
			panic!("invalid enum discriminant")
		} else {
			core::hint::unreachable_unchecked()
		}
	}
	extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
/// 	// ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_all_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::host::api::callbacks::__export_host_api_callbacks_cabi!($ty with_types_in $($path_to_types_root)*::exports::host::api::callbacks);
  )
}
#[doc(inline)]
pub(crate) use __export_all_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:all:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1708] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb2\x0c\x01A\x02\x01\
A\x10\x01B(\x01r\x02\x04liney\x09charactery\x04\0\x08position\x03\0\0\x01r\x02\x05\
start\x01\x03end\x01\x04\0\x05range\x03\0\x02\x01r\x04\x05range\x03\x0crange-off\
sety\x0crange-lengthy\x04texts\x04\0\"text-document-content-change-event\x03\0\x04\
\x04\0\x0dtext-document\x03\x01\x01m\x02\x04undo\x04redo\x04\0\x1btext-document-\
change-reason\x03\0\x07\x01i\x06\x01p\x05\x01k\x08\x01r\x03\x08document\x09\x0fc\
ontent-changes\x0a\x06reason\x0b\x04\0\x1atext-document-change-event\x03\0\x0c\x04\
\0\x0eoutput-channel\x03\x01\x01q\x01\x07pattern\x01s\0\x04\0\x0cglob-pattern\x03\
\0\x0f\x01ks\x01k\x10\x01r\x04\x08language\x11\x06scheme\x11\x0dnotebook-type\x11\
\x07pattern\x12\x04\0\x0fdocument-filter\x03\0\x13\x01p\x14\x01q\x02\x04many\x01\
\x15\0\x06single\x01\x14\0\x04\0\x11document-selector\x03\0\x16\x01h\x06\x01@\x01\
\x04self\x18\0s\x04\0\x19[method]text-document.uri\x01\x19\x04\0![method]text-do\
cument.language-id\x01\x19\x01@\x01\x04self\x18\0y\x04\0\x1d[method]text-documen\
t.version\x01\x1a\x04\0\x1e[method]text-document.get-text\x01\x19\x01h\x0e\x01@\x01\
\x04self\x1b\0s\x04\0\x1b[method]output-channel.name\x01\x1c\x01@\x02\x04self\x1b\
\x05values\x01\0\x04\0\x1d[method]output-channel.append\x01\x1d\x04\0\"[method]o\
utput-channel.append-line\x01\x1d\x01@\x01\x04self\x1b\x01\0\x04\0\x1c[method]ou\
tput-channel.clear\x01\x1e\x04\0\x1b[method]output-channel.show\x01\x1e\x03\x01\x0e\
host:api/types\x05\0\x02\x03\0\0\x0dtext-document\x01B\x09\x02\x03\x02\x01\x01\x04\
\0\x0dtext-document\x03\0\0\x01i\x01\x01p\x02\x01@\0\0\x03\x04\0\x0etext-documen\
ts\x01\x04\x01@\0\x01\0\x04\0$register-on-did-change-text-document\x01\x05\x04\0\
&unregister-on-did-change-text-document\x01\x05\x03\x01\x12host:api/workspace\x05\
\x02\x01B\x03\x01@\x01\x07commands\x01\0\x04\0\x10register-command\x01\0\x04\0\x12\
unregister-command\x01\0\x03\x01\x11host:api/commands\x05\x03\x02\x03\0\0\x0eout\
put-channel\x01B\x06\x02\x03\x02\x01\x04\x04\0\x0eoutput-channel\x03\0\0\x01ks\x01\
i\x01\x01@\x02\x04names\x0blanguage-id\x02\0\x03\x04\0\x15create-output-channel\x01\
\x04\x03\x01\x0fhost:api/window\x05\x05\x02\x03\0\0\x11document-selector\x01B\x07\
\x02\x03\x02\x01\x06\x04\0\x11document-selector\x03\0\0\x02\x03\x02\x01\x01\x04\0\
\x0dtext-document\x03\0\x02\x01i\x03\x01@\x02\x08selector\x01\x08document\x04\0y\
\x04\0\x0ematch-selector\x01\x05\x03\x01\x12host:api/languages\x05\x07\x02\x03\0\
\0\x1atext-document-change-event\x01B\x06\x02\x03\x02\x01\x08\x04\0\x1atext-docu\
ment-change-event\x03\0\0\x01@\x01\x05event\x01\x01\0\x04\0\x18did-change-text-d\
ocument\x01\x02\x01@\x01\x07commands\x01\0\x04\0\x0fexecute-command\x01\x03\x04\x01\
\x12host:api/callbacks\x05\x09\x04\x01\x0chost:api/all\x04\0\x0b\x09\x01\0\x03al\
l\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.201.0\x10\
wit-bindgen-rust\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
	wit_bindgen::rt::maybe_link_cabi_realloc();
}
