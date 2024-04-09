use crate::config::{Config, EvCall, EvDecl, EvSource, Ty, TyDecl};

use super::Output;

struct SharedOutput<'src> {
	config: &'src Config<'src>,
	tabs: u32,
	buf: String,
}

impl<'a> Output for SharedOutput<'a> {
	fn push(&mut self, s: &str) {
		self.buf.push_str(s);
	}

	fn indent(&mut self) {
		self.tabs += 1;
	}

	fn dedent(&mut self) {
		self.tabs -= 1;
	}

	fn push_indent(&mut self) {
		for _ in 0..self.tabs {
			self.push("\t");
		}
	}
}

impl<'a> SharedOutput<'a> {
	pub fn new(config: &'a Config) -> Self {
		Self {
			config,
			tabs: 0,
			buf: String::new(),
		}
	}

	fn push_tydecl(&mut self, tydecl: &TyDecl) {
		let name = &tydecl.name;
		let ty = &tydecl.ty;

		self.push_indent();
		self.push(&format!("type {name} = "));
		self.push_ty(ty);
		self.push("\n");
	}

	fn push_tydecls(&mut self) {
		for tydecl in self.config.tydecls.iter() {
			self.push_tydecl(tydecl);
		}

		if !self.config.tydecls.is_empty() {
			self.push("\n")
		}
	}

	pub fn output(mut self) -> String {
		self.push_file_header("Shared");

		if self.config.evdecls.is_empty() && self.config.fndecls.is_empty() {
			return self.buf;
		};

		if self.config.manual_event_loop {
			self.push_manual_event_loop(self.config);
		}

		self.push_tydecls();

		self.buf
	}
}

pub fn code(config: &Config) -> Option<String> {
	if !config.typescript {
		return None;
	}

	Some(SharedOutput::new(config).output())
}
