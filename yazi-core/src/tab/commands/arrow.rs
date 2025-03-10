use yazi_fs::Step;
use yazi_macro::render;
use yazi_proxy::ManagerProxy;
use yazi_shared::event::{CmdCow, Data};

use crate::tab::Tab;

struct Opt {
	step: Step,
}

impl From<CmdCow> for Opt {
	fn from(c: CmdCow) -> Self {
		let step = match c.first() {
			Some(Data::Integer(i)) => Step::from(*i as isize),
			Some(Data::String(s)) => s.parse().unwrap_or_default(),
			_ => Step::default(),
		};

		Self { step }
	}
}

impl From<isize> for Opt {
	fn from(n: isize) -> Self { Self { step: n.into() } }
}

impl Tab {
	#[yazi_codegen::command]
	pub fn arrow(&mut self, opt: Opt) {
		if !self.current.arrow(opt.step) {
			return;
		}

		// Visual selection
		if let Some((start, items)) = self.mode.visual_mut() {
			let after = self.current.cursor;

			items.clear();
			for i in start.min(after)..=after.max(start) {
				items.insert(i);
			}
		}

		ManagerProxy::hover(None, self.id);
		render!();
	}
}
