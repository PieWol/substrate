(function() {var implementors = {
"kitchensink_runtime":[["impl&lt;__SrApiBlock__: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>, RuntimeApiImplCall: <a class=\"trait\" href=\"sp_api/trait.CallApiAt.html\" title=\"trait sp_api::CallApiAt\">CallApiAt</a>&lt;__SrApiBlock__&gt; + 'static&gt; <a class=\"trait\" href=\"frame_benchmarking/utils/trait.Benchmark.html\" title=\"trait frame_benchmarking::utils::Benchmark\">Benchmark</a>&lt;__SrApiBlock__&gt; for <a class=\"struct\" href=\"kitchensink_runtime/struct.RuntimeApiImpl.html\" title=\"struct kitchensink_runtime::RuntimeApiImpl\">RuntimeApiImpl</a>&lt;__SrApiBlock__, RuntimeApiImplCall&gt;<span class=\"where fmt-newline\">where\n    RuntimeApiImplCall::<a class=\"associatedtype\" href=\"sp_api/trait.CallApiAt.html#associatedtype.StateBackend\" title=\"type sp_api::CallApiAt::StateBackend\">StateBackend</a>: <a class=\"trait\" href=\"sp_state_machine/backend/trait.Backend.html\" title=\"trait sp_state_machine::backend::Backend\">StateBackend</a>&lt;<a class=\"type\" href=\"sp_runtime/traits/type.HashFor.html\" title=\"type sp_runtime::traits::HashFor\">HashFor</a>&lt;__SrApiBlock__&gt;&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.reference.html\">&amp;'static RuntimeApiImplCall</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.bool.html\">bool</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    (<a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"frame_benchmarking/utils/struct.BenchmarkList.html\" title=\"struct frame_benchmarking::utils::BenchmarkList\">BenchmarkList</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"frame_support/traits/storage/struct.StorageInfo.html\" title=\"struct frame_support::traits::storage::StorageInfo\">StorageInfo</a>&gt;): <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"struct\" href=\"frame_benchmarking/utils/struct.BenchmarkConfig.html\" title=\"struct frame_benchmarking::utils::BenchmarkConfig\">BenchmarkConfig</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.69.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"frame_benchmarking/utils/struct.BenchmarkBatch.html\" title=\"struct frame_benchmarking::utils::BenchmarkBatch\">BenchmarkBatch</a>&gt;, <a class=\"enum\" href=\"sp_runtime/runtime_string/enum.RuntimeString.html\" title=\"enum sp_runtime::runtime_string::RuntimeString\">RuntimeString</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    __SrApiBlock__::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Block.html#associatedtype.Header\" title=\"type sp_runtime::traits::Block::Header\">Header</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>"]],
"node_template_runtime":[["impl&lt;__SrApiBlock__: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>, RuntimeApiImplCall: <a class=\"trait\" href=\"sp_api/trait.CallApiAt.html\" title=\"trait sp_api::CallApiAt\">CallApiAt</a>&lt;__SrApiBlock__&gt; + 'static&gt; <a class=\"trait\" href=\"frame_benchmarking/utils/trait.Benchmark.html\" title=\"trait frame_benchmarking::utils::Benchmark\">Benchmark</a>&lt;__SrApiBlock__&gt; for <a class=\"struct\" href=\"node_template_runtime/struct.RuntimeApiImpl.html\" title=\"struct node_template_runtime::RuntimeApiImpl\">RuntimeApiImpl</a>&lt;__SrApiBlock__, RuntimeApiImplCall&gt;<span class=\"where fmt-newline\">where\n    RuntimeApiImplCall::<a class=\"associatedtype\" href=\"sp_api/trait.CallApiAt.html#associatedtype.StateBackend\" title=\"type sp_api::CallApiAt::StateBackend\">StateBackend</a>: <a class=\"trait\" href=\"sp_state_machine/backend/trait.Backend.html\" title=\"trait sp_state_machine::backend::Backend\">StateBackend</a>&lt;<a class=\"type\" href=\"sp_runtime/traits/type.HashFor.html\" title=\"type sp_runtime::traits::HashFor\">HashFor</a>&lt;__SrApiBlock__&gt;&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.reference.html\">&amp;'static RuntimeApiImplCall</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/std/primitive.bool.html\">bool</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    (<a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"frame_benchmarking/utils/struct.BenchmarkList.html\" title=\"struct frame_benchmarking::utils::BenchmarkList\">BenchmarkList</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"node_template_runtime/struct.StorageInfo.html\" title=\"struct node_template_runtime::StorageInfo\">StorageInfo</a>&gt;): <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"struct\" href=\"frame_benchmarking/utils/struct.BenchmarkConfig.html\" title=\"struct frame_benchmarking::utils::BenchmarkConfig\">BenchmarkConfig</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.69.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.69.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"frame_benchmarking/utils/struct.BenchmarkBatch.html\" title=\"struct frame_benchmarking::utils::BenchmarkBatch\">BenchmarkBatch</a>&gt;, <a class=\"enum\" href=\"sp_runtime/runtime_string/enum.RuntimeString.html\" title=\"enum sp_runtime::runtime_string::RuntimeString\">RuntimeString</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    __SrApiBlock__::<a class=\"associatedtype\" href=\"sp_runtime/traits/trait.Block.html#associatedtype.Header\" title=\"type sp_runtime::traits::Block::Header\">Header</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()