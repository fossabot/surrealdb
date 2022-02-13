use crate::dbs::Executor;
use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::dbs::Statement;
use crate::doc::Document;
use crate::err::Error;

impl<'a> Document<'a> {
	pub async fn check(
		&self,
		ctx: &Runtime,
		opt: &Options,
		exe: &Executor<'_>,
		stm: &Statement<'_>,
	) -> Result<(), Error> {
		// Extract statement clause
		let cond = match stm {
			Statement::Select(stm) => stm.cond.as_ref(),
			Statement::Update(stm) => stm.cond.as_ref(),
			_ => unreachable!(),
		};
		// Match clause
		match cond {
			Some(v) => {
				match v.expr.compute(ctx, opt, exe, Some(&self.current)).await?.is_truthy() {
					false => Err(Error::IgnoreError),
					true => Ok(()),
				}
			}
			None => Ok(()),
		}
	}
}
