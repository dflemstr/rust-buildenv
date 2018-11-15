// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rustc::traits::query::CanonicalPredicateGoal;
use rustc::traits::{
    EvaluationResult, Obligation, ObligationCause, OverflowError, SelectionContext, TraitQueryMode,
};
use rustc::ty::query::Providers;
use rustc::ty::{ParamEnvAnd, TyCtxt};
use syntax::source_map::DUMMY_SP;

crate fn provide(p: &mut Providers) {
    *p = Providers {
        evaluate_obligation,
        ..*p
    };
}

fn evaluate_obligation<'tcx>(
    tcx: TyCtxt<'_, 'tcx, 'tcx>,
    canonical_goal: CanonicalPredicateGoal<'tcx>,
) -> Result<EvaluationResult, OverflowError> {
    tcx.infer_ctxt().enter_with_canonical(
        DUMMY_SP,
        &canonical_goal,
        |ref infcx, goal, _canonical_inference_vars| {
            let ParamEnvAnd {
                param_env,
                value: predicate,
            } = goal;

            let mut selcx = SelectionContext::with_query_mode(&infcx, TraitQueryMode::Canonical);
            let obligation = Obligation::new(ObligationCause::dummy(), param_env, predicate);

            selcx.evaluate_obligation_recursively(&obligation)
        },
    )
}
