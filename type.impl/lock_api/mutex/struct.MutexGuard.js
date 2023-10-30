(function() {var type_impls = {
"spin":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MutexGuard%3C'a,+R,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#497\">source</a><a href=\"#impl-MutexGuard%3C'a,+R,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R, T&gt; <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + 'a,\n    T: 'a + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.mutex\" class=\"method\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#499\">source</a><h4 class=\"code-header\">pub fn <a href=\"lock_api/mutex/struct.MutexGuard.html#tymethod.mutex\" class=\"fn\">mutex</a>(s: &amp;<a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;) -&gt; &amp;'a <a class=\"struct\" href=\"lock_api/mutex/struct.Mutex.html\" title=\"struct lock_api::mutex::Mutex\">Mutex</a>&lt;R, T&gt;</h4></section></summary><div class=\"docblock\"><p>Returns a reference to the original <code>Mutex</code> object.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.map\" class=\"method\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#512-514\">source</a><h4 class=\"code-header\">pub fn <a href=\"lock_api/mutex/struct.MutexGuard.html#tymethod.map\" class=\"fn\">map</a>&lt;U, F&gt;(s: <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;, f: F) -&gt; <a class=\"struct\" href=\"lock_api/mutex/struct.MappedMutexGuard.html\" title=\"struct lock_api::mutex::MappedMutexGuard\">MappedMutexGuard</a>&lt;'a, R, U&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;mut T</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;mut U</a>,\n    U: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h4></section></summary><div class=\"docblock\"><p>Makes a new <code>MappedMutexGuard</code> for a component of the locked data.</p>\n<p>This operation cannot fail as the <code>MutexGuard</code> passed\nin already locked the mutex.</p>\n<p>This is an associated function that needs to be\nused as <code>MutexGuard::map(...)</code>. A method would interfere with methods of\nthe same name on the contents of the locked data.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_map\" class=\"method\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#536-538\">source</a><h4 class=\"code-header\">pub fn <a href=\"lock_api/mutex/struct.MutexGuard.html#tymethod.try_map\" class=\"fn\">try_map</a>&lt;U, F&gt;(\n    s: <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;,\n    f: F\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"lock_api/mutex/struct.MappedMutexGuard.html\" title=\"struct lock_api::mutex::MappedMutexGuard\">MappedMutexGuard</a>&lt;'a, R, U&gt;, <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;mut T</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;mut U</a>&gt;,\n    U: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h4></section></summary><div class=\"docblock\"><p>Attempts to make a new <code>MappedMutexGuard</code> for a component of the\nlocked data. The original guard is returned if the closure returns <code>None</code>.</p>\n<p>This operation cannot fail as the <code>MutexGuard</code> passed\nin already locked the mutex.</p>\n<p>This is an associated function that needs to be\nused as <code>MutexGuard::try_map(...)</code>. A method would interfere with methods of\nthe same name on the contents of the locked data.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.unlocked\" class=\"method\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#558-560\">source</a><h4 class=\"code-header\">pub fn <a href=\"lock_api/mutex/struct.MutexGuard.html#tymethod.unlocked\" class=\"fn\">unlocked</a>&lt;F, U&gt;(s: &amp;mut <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;, f: F) -&gt; U<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>() -&gt; U,</span></h4></section></summary><div class=\"docblock\"><p>Temporarily unlocks the mutex to execute the given function.</p>\n<p>This is safe because <code>&amp;mut</code> guarantees that there exist no other\nreferences to the data protected by the mutex.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.leak\" class=\"method\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#575\">source</a><h4 class=\"code-header\">pub fn <a href=\"lock_api/mutex/struct.MutexGuard.html#tymethod.leak\" class=\"fn\">leak</a>(s: <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;'a mut T</a></h4></section></summary><div class=\"docblock\"><p>Leaks the mutex guard and returns a mutable reference to the data\nprotected by the mutex.</p>\n<p>This will leave the <code>Mutex</code> in a locked state.</p>\n</div></details></div></details>",0,"spin::lock_api::MutexGuard"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MutexGuard%3C'a,+R,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#582\">source</a><a href=\"#impl-MutexGuard%3C'a,+R,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R, T&gt; <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutexFair.html\" title=\"trait lock_api::mutex::RawMutexFair\">RawMutexFair</a> + 'a,\n    T: 'a + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.unlock_fair\" class=\"method\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#596\">source</a><h4 class=\"code-header\">pub fn <a href=\"lock_api/mutex/struct.MutexGuard.html#tymethod.unlock_fair\" class=\"fn\">unlock_fair</a>(s: <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;)</h4></section></summary><div class=\"docblock\"><p>Unlocks the mutex using a fair unlock protocol.</p>\n<p>By default, mutexes are unfair and allow the current thread to re-lock\nthe mutex before another has the chance to acquire the lock, even if\nthat thread has been blocked on the mutex for a long time. This is the\ndefault because it allows much higher throughput as it avoids forcing a\ncontext switch on every mutex unlock. This can result in one thread\nacquiring a mutex many more times than other threads.</p>\n<p>However in some cases it can be beneficial to ensure fairness by forcing\nthe lock to pass on to a waiting thread if there is one. This is done by\nusing this method instead of dropping the <code>MutexGuard</code> normally.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.unlocked_fair\" class=\"method\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#611-613\">source</a><h4 class=\"code-header\">pub fn <a href=\"lock_api/mutex/struct.MutexGuard.html#tymethod.unlocked_fair\" class=\"fn\">unlocked_fair</a>&lt;F, U&gt;(s: &amp;mut <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;, f: F) -&gt; U<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>() -&gt; U,</span></h4></section></summary><div class=\"docblock\"><p>Temporarily unlocks the mutex to execute the given function.</p>\n<p>The mutex is unlocked using a fair unlock protocol.</p>\n<p>This is safe because <code>&amp;mut</code> guarantees that there exist no other\nreferences to the data protected by the mutex.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.bump\" class=\"method\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#629\">source</a><h4 class=\"code-header\">pub fn <a href=\"lock_api/mutex/struct.MutexGuard.html#tymethod.bump\" class=\"fn\">bump</a>(s: &amp;mut <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;)</h4></section></summary><div class=\"docblock\"><p>Temporarily yields the mutex to a waiting thread if there is one.</p>\n<p>This method is functionally equivalent to calling <code>unlock_fair</code> followed\nby <code>lock</code>, however it can be much more efficient in the case where there\nare no waiting threads.</p>\n</div></details></div></details>",0,"spin::lock_api::MutexGuard"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-MutexGuard%3C'a,+R,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#652\">source</a><a href=\"#impl-Drop-for-MutexGuard%3C'a,+R,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + 'a,\n    T: 'a + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#654\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","spin::lock_api::MutexGuard"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-MutexGuard%3C'a,+R,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#668\">source</a><a href=\"#impl-Display-for-MutexGuard%3C'a,+R,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + 'a,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + 'a + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#669\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","spin::lock_api::MutexGuard"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-MutexGuard%3C'a,+R,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#662\">source</a><a href=\"#impl-Debug-for-MutexGuard%3C'a,+R,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + 'a,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + 'a + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#663\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","spin::lock_api::MutexGuard"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-MutexGuard%3C'a,+R,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#637\">source</a><a href=\"#impl-Deref-for-MutexGuard%3C'a,+R,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + 'a,\n    T: 'a + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = T</h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#640\">source</a><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;T</a></h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","spin::lock_api::MutexGuard"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DerefMut-for-MutexGuard%3C'a,+R,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#645\">source</a><a href=\"#impl-DerefMut-for-MutexGuard%3C'a,+R,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html\" title=\"trait core::ops::deref::DerefMut\">DerefMut</a> for <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + 'a,\n    T: 'a + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#647\">source</a><a href=\"#method.deref_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut\" class=\"fn\">deref_mut</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;mut T</a></h4></section></summary><div class='docblock'>Mutably dereferences the value.</div></details></div></details>","DerefMut","spin::lock_api::MutexGuard"],["<section id=\"impl-Sync-for-MutexGuard%3C'a,+R,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/lock_api/mutex.rs.html#495\">source</a><a href=\"#impl-Sync-for-MutexGuard%3C'a,+R,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"lock_api/mutex/struct.MutexGuard.html\" title=\"struct lock_api::mutex::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'a,\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'a + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span></h3></section>","Sync","spin::lock_api::MutexGuard"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()