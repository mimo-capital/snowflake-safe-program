use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::instructions::{do_execute_multisig_flow, ExecuteMultisigFlow};
use crate::state::static_config::ProposalStateType;

pub fn handler(ctx: Context<ExecuteMultisigFlow>) -> Result<()> {
    let safe = &ctx.accounts.safe;
    let flow = &ctx.accounts.flow;
    let caller = &ctx.accounts.caller;
    let execute_by_safe_owner = safe.is_owner(&caller.key());

    require!(safe.key() == flow.safe, ErrorCode::InvalidSafe);
    require!(execute_by_safe_owner, ErrorCode::InvalidOwner);
    require!(
        flow.proposal_stage != ProposalStateType::Complete as u8
            && flow.proposal_stage != ProposalStateType::Failed as u8,
        ErrorCode::RequestIsExecutedAlready
    );
    require!(
        flow.proposal_stage != ProposalStateType::Rejected as u8,
        ErrorCode::RequestIsRejected
    );
    require!(
        flow.get_approvals() >= safe.approvals_required,
        ErrorCode::FlowNotEnoughApprovals
    );
    require!(
        flow.proposal_stage == ProposalStateType::Approved as u8,
        ErrorCode::RequestIsNotApprovedYet
    );

    let result = do_execute_multisig_flow::handler(&ctx);
    let flow = &mut ctx.accounts.flow;
    flow.proposal_stage = ProposalStateType::Complete as u8;
    result
}
