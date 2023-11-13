(function() {var type_impls = {
"x86":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Index%3CI%3E-for-%5BT;+N%5D\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.50.0\">1.50.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/array/mod.rs.html#340-342\">source</a></span><a href=\"#impl-Index%3CI%3E-for-%5BT;+N%5D\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, I, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">[T; N]</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.slice.html\">[T]</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt;,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = &lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.slice.html\">[T]</a> as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" title=\"type core::ops::index::Index::Output\">Output</a></h4></section></summary><div class='docblock'>The returned type after indexing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.index\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/core/array/mod.rs.html#347\">source</a><a href=\"#method.index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index\" class=\"fn\">index</a>(&amp;self, index: I) -&gt; &amp;&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">[T; N]</a> as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" title=\"type core::ops::index::Index::Output\">Output</a></h4></section></summary><div class='docblock'>Performs the indexing (<code>container[index]</code>) operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index\">Read more</a></div></details></div></details>","Index<I>","x86::bits32::paging::PD","x86::bits32::paging::PT","x86::bits64::paging::PML4","x86::bits64::paging::PML5","x86::bits64::paging::PDPT","x86::bits64::paging::PD","x86::bits64::paging::PT"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-%5BT;+N%5D\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/array/mod.rs.html#313\">source</a></span><a href=\"#impl-Debug-for-%5BT;+N%5D\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">[T; N]</a><span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/core/array/mod.rs.html#314\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","x86::bits32::paging::PD","x86::bits32::paging::PT","x86::bits64::paging::PML4","x86::bits64::paging::PML5","x86::bits64::paging::PDPT","x86::bits64::paging::PD","x86::bits64::paging::PT"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IndexMut%3CI%3E-for-%5BT;+N%5D\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.50.0\">1.50.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/array/mod.rs.html#353-355\">source</a></span><a href=\"#impl-IndexMut%3CI%3E-for-%5BT;+N%5D\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, I, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;I&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">[T; N]</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.slice.html\">[T]</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;I&gt;,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.index_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/nightly/src/core/array/mod.rs.html#358\">source</a><a href=\"#method.index_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut\" class=\"fn\">index_mut</a>(&amp;mut self, index: I) -&gt; &amp;mut &lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.array.html\">[T; N]</a> as <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;I&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" title=\"type core::ops::index::Index::Output\">Output</a></h4></section></summary><div class='docblock'>Performs the mutable indexing (<code>container[index]</code>) operation. <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut\">Read more</a></div></details></div></details>","IndexMut<I>","x86::bits32::paging::PD","x86::bits32::paging::PT","x86::bits64::paging::PML4","x86::bits64::paging::PML5","x86::bits64::paging::PDPT","x86::bits64::paging::PD","x86::bits64::paging::PT"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()