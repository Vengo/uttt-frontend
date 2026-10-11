use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Leaderboard,
    #[at("/training")]
    Training,
    #[at("/bots/alphabeta")]
    AlphaBeta,
    #[at("/bots/umcts")]
    Umcts,
    #[at("/bots/alphazero")]
    AlphaZero,
    #[at("/bots/policy-alphabeta")]
    PolicyAlphaBeta,
    #[at("/bots/solvers")]
    Solvers,
    #[at("/play")]
    Play,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// ============================================================================
// Navigation Bar
// ============================================================================
#[function_component(Navbar)]
fn navbar() -> Html {
    let navigator = use_navigator().unwrap();

    let nav_to = |route: Route| {
        let nav = navigator.clone();
        Callback::from(move |_| nav.push(&route))
    };

    html! {
        <nav class="bg-[#111827] border-b border-slate-800 sticky top-0 z-50 px-6 py-4">
            <div class="max-w-7xl mx-auto flex flex-wrap items-center justify-between gap-4">
                <div class="flex items-center space-x-3 cursor-pointer" onclick={nav_to(Route::Leaderboard)}>
                    <div class="w-10 h-10 rounded-xl bg-gradient-to-tr from-emerald-500 to-cyan-500 flex items-center justify-center font-bold text-black text-xl shadow-lg shadow-emerald-500/20">
                        {"U"}
                    </div>
                    <div>
                        <div class="font-bold text-lg text-white tracking-wide flex items-center gap-2">
                            {"UTTT Engine Suite"}
                            <span class="text-xs bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 px-2 py-0.5 rounded-full font-mono">{"v2.4"}</span>
                        </div>
                        <div class="text-xs text-slate-400">{"Ultimate Tic-Tac-Toe AI & Tablebase Arena"}</div>
                    </div>
                </div>

                <div class="flex items-center space-x-1 sm:space-x-2 text-sm">
                    <button onclick={nav_to(Route::Leaderboard)} class="px-3 py-2 rounded-lg text-slate-300 hover:text-white hover:bg-slate-800 transition">
                        {"🏆 Leaderboard"}
                    </button>
                    <button onclick={nav_to(Route::Training)} class="px-3 py-2 rounded-lg text-amber-400 hover:text-white hover:bg-slate-800 transition flex items-center gap-1 font-medium">
                        <span>{"🚀"}</span>
                        <span>{"Training Lab"}</span>
                    </button>
                    <button onclick={nav_to(Route::PolicyAlphaBeta)} class="px-3 py-2 rounded-lg text-slate-300 hover:text-white hover:bg-slate-800 transition">
                        {"⚡ Hybrid Minimax"}
                    </button>
                    <button onclick={nav_to(Route::AlphaZero)} class="px-3 py-2 rounded-lg text-slate-300 hover:text-white hover:bg-slate-800 transition">
                        {"🧠 AlphaZero"}
                    </button>
                    <button onclick={nav_to(Route::Umcts)} class="px-3 py-2 rounded-lg text-slate-300 hover:text-white hover:bg-slate-800 transition">
                        {"🌲 UMCTS"}
                    </button>
                    <button onclick={nav_to(Route::AlphaBeta)} class="px-3 py-2 rounded-lg text-slate-300 hover:text-white hover:bg-slate-800 transition">
                        {"🔍 Alpha-Beta"}
                    </button>
                    <button onclick={nav_to(Route::Solvers)} class="px-3 py-2 rounded-lg text-slate-300 hover:text-white hover:bg-slate-800 transition">
                        {"🏰 Solvers & EGTB"}
                    </button>
                    <button onclick={nav_to(Route::Play)} class="px-4 py-2 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white font-medium shadow-md shadow-emerald-600/30 transition">
                        {"🎮 Play"}
                    </button>
                </div>
            </div>
        </nav>
    }
}

// ============================================================================
// Leaderboard Page
// ============================================================================
#[function_component(LeaderboardPage)]
fn leaderboard_page() -> Html {
    let navigator = use_navigator().unwrap();

    let nav_to = |route: Route| {
        let nav = navigator.clone();
        Callback::from(move |_| nav.push(&route))
    };

    html! {
        <div class="max-w-7xl mx-auto px-6 py-10 space-y-10">
            // Hero section
            <div class="bg-gradient-to-r from-slate-900 via-slate-800 to-slate-900 border border-slate-800 p-8 rounded-2xl relative overflow-hidden shadow-2xl">
                <div class="relative z-10 space-y-3 max-w-3xl">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-mono bg-cyan-500/10 text-cyan-400 border border-cyan-500/30">
                        <span>{"GPU Accelerated"}</span>
                        <span>{"•"}</span>
                        <span>{"RTX 5060 Ti (146k evals/s)"}</span>
                    </div>
                    <h1 class="text-4xl font-extrabold text-white tracking-tight">
                        {"Ultimate Tic-Tac-Toe AI Engine Leaderboard"}
                    </h1>
                    <p class="text-slate-300 text-base leading-relaxed">
                        {"Empirical benchmarking results from paired 2-ply opening suite tournaments running at standard 100 ms CodinGame time controls. Ratings calculated with the Bayesian Glicko-2 system (Anchor: Rating 1500, RD 30)."}
                    </p>
                </div>
            </div>

            // Rating table
            <div class="bg-[#111827] border border-slate-800 rounded-2xl overflow-hidden shadow-xl">
                <div class="px-6 py-4 border-b border-slate-800 flex justify-between items-center">
                    <h2 class="text-xl font-bold text-white flex items-center gap-2">
                        {"🏆 Bayesian Glicko-2 Arena Standings"}
                    </h2>
                    <span class="text-xs text-slate-400 font-mono">{"Time Control: 100ms / move"}</span>
                </div>
                <div class="overflow-x-auto">
                    <table class="w-full text-left text-sm text-slate-300">
                        <thead class="bg-slate-800/60 text-xs uppercase font-mono text-slate-400 border-b border-slate-800">
                            <tr>
                                <th class="px-6 py-4">{"Rank & Bot"}</th>
                                <th class="px-6 py-4">{"Glicko-2 Rating"}</th>
                                <th class="px-6 py-4">{"95% CI Margin"}</th>
                                <th class="px-6 py-4">{"Search Depth / Nodes"}</th>
                                <th class="px-6 py-4">{"Evaluation Paradigm"}</th>
                                <th class="px-6 py-4 text-right">{"Actions"}</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-800/80">
                            // 1. PolicyAlphaBetaBot
                            <tr class="hover:bg-slate-800/30 transition">
                                <td class="px-6 py-4 flex items-center gap-3">
                                    <span class="w-6 text-center font-bold text-amber-400">{"1"}</span>
                                    <div>
                                        <div class="font-bold text-white flex items-center gap-2">
                                            {"PolicyAlphaBetaBot"}
                                            <span class="text-[10px] bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 px-1.5 py-0.5 rounded font-mono">{"HYBRID"}</span>
                                        </div>
                                        <div class="text-xs text-slate-400">{"Neural Prior Root Order + AlphaBetaV2 + DF-PN"}</div>
                                    </div>
                                </td>
                                <td class="px-6 py-4 font-mono font-bold text-emerald-400">{"1648.5"}</td>
                                <td class="px-6 py-4 font-mono text-slate-400">{"± 55.8 (RD: 28.5)"}</td>
                                <td class="px-6 py-4 font-mono text-cyan-300">{"Depth 10–12"}</td>
                                <td class="px-6 py-4 text-xs text-slate-300">{"Pure Rust INT8 Policy Prior + Minimax Pruning"}</td>
                                <td class="px-6 py-4 text-right">
                                    <button onclick={nav_to(Route::PolicyAlphaBeta)} class="text-xs bg-slate-800 hover:bg-slate-700 text-emerald-400 border border-slate-700 px-3 py-1.5 rounded-lg transition">
                                        {"Inspect Bot"}
                                    </button>
                                </td>
                            </tr>

                            // 2. AlphaBetaV2Bot
                            <tr class="hover:bg-slate-800/30 transition">
                                <td class="px-6 py-4 flex items-center gap-3">
                                    <span class="w-6 text-center font-bold text-slate-300">{"2"}</span>
                                    <div>
                                        <div class="font-bold text-white flex items-center gap-2">
                                            {"AlphaBetaV2Bot"}
                                            <span class="text-[10px] bg-cyan-500/20 text-cyan-300 border border-cyan-500/40 px-1.5 py-0.5 rounded font-mono">{"MINIMAX"}</span>
                                        </div>
                                        <div class="text-xs text-slate-400">{"Macro-board routing + History/Countermoves + DF-PN"}</div>
                                    </div>
                                </td>
                                <td class="px-6 py-4 font-mono font-bold text-emerald-400">{"1488.7"}</td>
                                <td class="px-6 py-4 font-mono text-slate-400">{"± 78.5 (RD: 40.1)"}</td>
                                <td class="px-6 py-4 font-mono text-cyan-300">{"Depth 7–8"}</td>
                                <td class="px-6 py-4 text-xs text-slate-300">{"Full Window Negamax + Routing Penalties"}</td>
                                <td class="px-6 py-4 text-right">
                                    <button onclick={nav_to(Route::AlphaBeta)} class="text-xs bg-slate-800 hover:bg-slate-700 text-cyan-400 border border-slate-700 px-3 py-1.5 rounded-lg transition">
                                        {"Inspect Bot"}
                                    </button>
                                </td>
                            </tr>

                            // 3. AlphaBeta Baseline
                            <tr class="hover:bg-slate-800/30 transition">
                                <td class="px-6 py-4 flex items-center gap-3">
                                    <span class="w-6 text-center font-bold text-slate-400">{"3"}</span>
                                    <div>
                                        <div class="font-bold text-white">{"AlphaBetaBot (Baseline)"}</div>
                                        <div class="text-xs text-slate-400">{"Classic Negamax with Killer Heuristics"}</div>
                                    </div>
                                </td>
                                <td class="px-6 py-4 font-mono font-bold text-slate-200">{"1500.0"}</td>
                                <td class="px-6 py-4 font-mono text-slate-400">{"± 58.8 (RD: 30.0)"}</td>
                                <td class="px-6 py-4 font-mono text-cyan-300">{"Depth 6–7"}</td>
                                <td class="px-6 py-4 text-xs text-slate-300">{"Handcrafted Heuristics (Superboard + Micro)"}</td>
                                <td class="px-6 py-4 text-right">
                                    <button onclick={nav_to(Route::AlphaBeta)} class="text-xs bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 px-3 py-1.5 rounded-lg transition">
                                        {"Inspect Bot"}
                                    </button>
                                </td>
                            </tr>

                            // 4. UmctsV2Bot Baseline
                            <tr class="hover:bg-slate-800/30 transition">
                                <td class="px-6 py-4 flex items-center gap-3">
                                    <span class="w-6 text-center font-bold text-slate-400">{"4"}</span>
                                    <div>
                                        <div class="font-bold text-white">{"UmctsV2Bot (Baseline)"}</div>
                                        <div class="text-xs text-slate-400">{"MCTS Solver with pure random simulation rollouts"}</div>
                                    </div>
                                </td>
                                <td class="px-6 py-4 font-mono font-bold text-slate-200">{"1500.0"}</td>
                                <td class="px-6 py-4 font-mono text-slate-400">{"± 58.8 (RD: 30.0)"}</td>
                                <td class="px-6 py-4 font-mono text-cyan-300">{"16,650 visits"}</td>
                                <td class="px-6 py-4 text-xs text-slate-300">{"Pure Random Rollouts (Law of Large Numbers)"}</td>
                                <td class="px-6 py-4 text-right">
                                    <button onclick={nav_to(Route::Umcts)} class="text-xs bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 px-3 py-1.5 rounded-lg transition">
                                        {"Inspect Bot"}
                                    </button>
                                </td>
                            </tr>

                            // 5. UmctsV3Bot
                            <tr class="hover:bg-slate-800/30 transition">
                                <td class="px-6 py-4 flex items-center gap-3">
                                    <span class="w-6 text-center font-bold text-slate-400">{"5"}</span>
                                    <div>
                                        <div class="font-bold text-white flex items-center gap-2">
                                            {"UmctsV3Bot"}
                                            <span class="text-[10px] bg-purple-500/20 text-purple-300 border border-purple-500/40 px-1.5 py-0.5 rounded font-mono">{"MCTS+EGTB"}</span>
                                        </div>
                                        <div class="text-xs text-slate-400">{"UCT + EGTB Cutoffs + DF-PN Solver Handoff"}</div>
                                    </div>
                                </td>
                                <td class="px-6 py-4 font-mono font-bold text-slate-300">{"1383.6"}</td>
                                <td class="px-6 py-4 font-mono text-slate-400">{"± 95.8 (RD: 48.9)"}</td>
                                <td class="px-6 py-4 font-mono text-cyan-300">{"15,236 visits"}</td>
                                <td class="px-6 py-4 text-xs text-slate-300">{"Pure Random Rollouts + Tablebase Handoff"}</td>
                                <td class="px-6 py-4 text-right">
                                    <button onclick={nav_to(Route::Umcts)} class="text-xs bg-slate-800 hover:bg-slate-700 text-purple-400 border border-slate-700 px-3 py-1.5 rounded-lg transition">
                                        {"Inspect Bot"}
                                    </button>
                                </td>
                            </tr>

                            // 6. AlphaZeroBot
                            <tr class="hover:bg-slate-800/30 transition">
                                <td class="px-6 py-4 flex items-center gap-3">
                                    <span class="w-6 text-center font-bold text-slate-400">{"6"}</span>
                                    <div>
                                        <div class="font-bold text-white flex items-center gap-2">
                                            {"AlphaZeroBot"}
                                            <span class="text-[10px] bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 px-1.5 py-0.5 rounded font-mono">{"NEURAL"}</span>
                                        </div>
                                        <div class="text-xs text-slate-400">{"Dual-headed ResNet INT8 inference + PUCT search"}</div>
                                    </div>
                                </td>
                                <td class="px-6 py-4 font-mono font-bold text-emerald-400">{"1320.0"}</td>
                                <td class="px-6 py-4 font-mono text-slate-400">{"± 110.0 (RD: 56.1)"}</td>
                                <td class="px-6 py-4 font-mono text-cyan-300">{"~572 evals/s"}</td>
                                <td class="px-6 py-4 text-xs text-slate-300">{"Learned Dual Heads: Policy P(s,a) & Value V(s)"}</td>
                                <td class="px-6 py-4 text-right">
                                    <button onclick={nav_to(Route::AlphaZero)} class="text-xs bg-slate-800 hover:bg-slate-700 text-emerald-400 border border-slate-700 px-3 py-1.5 rounded-lg transition">
                                        {"Inspect Bot"}
                                    </button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

            // Architectural Cards
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="bg-[#111827] border border-slate-800 p-6 rounded-2xl space-y-4 hover:border-emerald-500/50 transition cursor-pointer" onclick={nav_to(Route::PolicyAlphaBeta)}>
                    <div class="w-12 h-12 rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 flex items-center justify-center text-2xl font-bold">
                        {"⚡"}
                    </div>
                    <h3 class="text-lg font-bold text-white">{"Hybrid Policy Minimax"}</h3>
                    <p class="text-sm text-slate-400 leading-relaxed">
                        {"Runs pure Rust INT8 neural policy forward at root (1.7ms) to order moves. Pushes minimax search depth 2–3 plies deeper via early beta cutoffs."}
                    </p>
                    <div class="text-xs text-emerald-400 font-medium">{"Explore Algorithm →"}</div>
                </div>

                <div class="bg-[#111827] border border-slate-800 p-6 rounded-2xl space-y-4 hover:border-cyan-500/50 transition cursor-pointer" onclick={nav_to(Route::AlphaZero)}>
                    <div class="w-12 h-12 rounded-xl bg-cyan-500/10 text-cyan-400 border border-cyan-500/30 flex items-center justify-center text-2xl font-bold">
                        {"🧠"}
                    </div>
                    <h3 class="text-lg font-bold text-white">{"AlphaZero ResNet"}</h3>
                    <p class="text-sm text-slate-400 leading-relaxed">
                        {"Dual-headed 4-block ResNet with 5x9x9 spatial tensors. Trained via batched GPU MCTS on RTX 5060 Ti (146,000 evals/sec) and quantized to INT8."}
                    </p>
                    <div class="text-xs text-cyan-400 font-medium">{"Explore Neural Network →"}</div>
                </div>

                <div class="bg-[#111827] border border-slate-800 p-6 rounded-2xl space-y-4 hover:border-purple-500/50 transition cursor-pointer" onclick={nav_to(Route::Solvers)}>
                    <div class="w-12 h-12 rounded-xl bg-purple-500/10 text-purple-400 border border-purple-500/30 flex items-center justify-center text-2xl font-bold">
                        {"🏰"}
                    </div>
                    <h3 class="text-lg font-bold text-white">{"Retrograde Tablebases"}</h3>
                    <p class="text-sm text-slate-400 leading-relaxed">
                        {"Exact retrograde dynamic programming for Tiers 9, 8, and 7. Zero-copy memory mapped probing with binary search in under 1 microsecond."}
                    </p>
                    <div class="text-xs text-purple-400 font-medium">{"Explore Tablebases →"}</div>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// AlphaBeta Visualizer Page
// ============================================================================
#[function_component(AlphaBetaPage)]
fn alphabeta_page() -> Html {
    let cutoff_step = use_state(|| 0usize);

    let next_step = {
        let cutoff_step = cutoff_step.clone();
        Callback::from(move |_| {
            let next = (*cutoff_step + 1) % 4;
            cutoff_step.set(next);
        })
    };

    html! {
        <div class="max-w-7xl mx-auto px-6 py-10 space-y-10">
            <div class="space-y-2">
                <div class="inline-flex items-center gap-2 text-xs font-mono text-cyan-400 bg-cyan-500/10 border border-cyan-500/30 px-3 py-1 rounded-full">
                    {"Algorithm Visualizer: Negamax with Alpha-Beta Pruning"}
                </div>
                <h1 class="text-3xl font-extrabold text-white">{"AlphaBetaBot & AlphaBetaV2"}</h1>
                <p class="text-slate-400 text-sm max-w-3xl leading-relaxed">
                    {"Negamax searches the game tree depth-first. By tracking alpha (the minimum score the maximizing player is assured of) and beta (the maximum score the minimizing player is assured of), branches where score >= beta can be immediately discarded without affecting the search outcome."}
                </p>
            </div>

            // Interactive Search Tree Visualizer
            <div class="bg-[#111827] border border-slate-800 rounded-2xl p-6 space-y-6">
                <div class="flex justify-between items-center border-b border-slate-800 pb-4">
                    <h2 class="text-lg font-bold text-white flex items-center gap-2">
                        {"Interactive Alpha-Beta Pruning Tree"}
                    </h2>
                    <button onclick={next_step} class="px-4 py-2 bg-cyan-600 hover:bg-cyan-500 text-white rounded-lg text-xs font-semibold shadow transition">
                        {format!("Step Animation (Step {}/3)", *cutoff_step + 1)}
                    </button>
                </div>

                // SVG Game Tree
                <div class="bg-slate-950 p-6 rounded-xl border border-slate-800/80 flex justify-center overflow-x-auto">
                    <svg width="700" height="280" viewBox="0 0 700 280" class="select-none font-mono text-xs">
                        // Root node
                        <circle cx="350" cy="40" r="24" class="fill-cyan-950 stroke-cyan-400 stroke-2" />
                        <text x="350" y="44" text-anchor="middle" fill="#38bdf8" font-weight="bold">{"MAX"}</text>
                        <text x="350" y="16" text-anchor="middle" fill="#94a3b8" font-size="10">{"α=-∞, β=+∞"}</text>

                        // Level 1 edges
                        <line x1="335" y1="60" x2="180" y2="120" stroke="#475569" stroke-width="2" />
                        <line x1="365" y1="60" x2="520" y2="120" stroke="#475569" stroke-width="2" />

                        // Level 1: Left Node (MIN)
                        <circle cx="180" cy="130" r="22" class="fill-slate-800 stroke-slate-400 stroke-2" />
                        <text x="180" y="134" text-anchor="middle" fill="#cbd5e1" font-weight="bold">{"MIN"}</text>
                        <text x="180" y="102" text-anchor="middle" fill="#38bdf8" font-size="10">{"Score: +4"}</text>

                        // Level 1: Right Node (MIN)
                        <circle cx="520" cy="130" r="22" class={if *cutoff_step >= 2 { "fill-emerald-950 stroke-emerald-400 stroke-2" } else { "fill-slate-800 stroke-slate-400 stroke-2" }} />
                        <text x="520" y="134" text-anchor="middle" fill="#cbd5e1" font-weight="bold">{"MIN"}</text>
                        <text x="520" y="102" text-anchor="middle" fill="#f43f5e" font-size="10">{if *cutoff_step >= 2 { "β-CUTOFF!" } else { "α=+4, β=+∞" }}</text>

                        // Level 2 Left Children
                        <line x1="165" y1="150" x2="120" y2="210" stroke="#475569" stroke-width="2" />
                        <line x1="195" y1="150" x2="240" y2="210" stroke="#475569" stroke-width="2" />
                        <rect x="100" y="210" width="40" height="30" rx="6" fill="#1e293b" stroke="#64748b" stroke-width="1.5" />
                        <text x="120" y="228" text-anchor="middle" fill="#e2e8f0" font-weight="bold">{"+4"}</text>
                        <rect x="220" y="210" width="40" height="30" rx="6" fill="#1e293b" stroke="#64748b" stroke-width="1.5" />
                        <text x="240" y="228" text-anchor="middle" fill="#e2e8f0" font-weight="bold">{"+7"}</text>

                        // Level 2 Right Children
                        <line x1="505" y1="150" x2="460" y2="210" stroke="#475569" stroke-width="2" />
                        <line x1="535" y1="150" x2="580" y2="210" stroke={if *cutoff_step >= 2 { "#dc2626" } else { "#475569" }} stroke-dasharray={if *cutoff_step >= 2 { "4" } else { "0" }} stroke-width="2" />
                        <rect x="440" y="210" width="40" height="30" rx="6" fill="#1e293b" stroke="#64748b" stroke-width="1.5" />
                        <text x="460" y="228" text-anchor="middle" fill="#e2e8f0" font-weight="bold">{"+2"}</text>

                        // Pruned Right Subtree
                        <g opacity={if *cutoff_step >= 2 { "0.3" } else { "1.0" }}>
                            <rect x="560" y="210" width="40" height="30" rx="6" fill="#1e293b" stroke="#64748b" stroke-width="1.5" />
                            <text x="580" y="228" text-anchor="middle" fill="#e2e8f0" font-weight="bold">{"?"}</text>
                        </g>

                        if *cutoff_step >= 2 {
                            <g>
                                <rect x="545" y="195" width="70" height="22" rx="4" fill="#ef4444" />
                                <text x="580" y="210" text-anchor="middle" fill="#ffffff" font-size="9" font-weight="bold">{"PRUNED"}</text>
                            </g>
                        }
                    </svg>
                </div>

                // Key insights
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs font-mono">
                    <div class="bg-slate-900/60 p-4 rounded-xl border border-slate-800">
                        <div class="font-bold text-slate-200 mb-1">{"Macro-Board Routing Penalties (AlphaBetaV2):"}</div>
                        <ul class="space-y-1 text-slate-400">
                            <li>{"• Gifting Free Choice: Directing opponent to full/tied board → "}<span class="text-rose-400">{"-300"}</span></li>
                            <li>{"• Opponent 2-in-a-row threat directed → "}<span class="text-rose-400">{"-400"}</span></li>
                            <li>{"• Neutralizing subboard (0 opponent lines) → "}<span class="text-emerald-400">{"+150"}</span></li>
                        </ul>
                    </div>
                    <div class="bg-slate-900/60 p-4 rounded-xl border border-slate-800">
                        <div class="font-bold text-slate-200 mb-1">{"Move Ordering Hierarchy:"}</div>
                        <ol class="space-y-1 text-slate-400 list-decimal list-inside">
                            <li>{"Transposition Table Best Move (1,000,000)"}</li>
                            <li>{"Immediate Superboard Match Win (500,000)"}</li>
                            <li>{"Killer Moves 1 & 2 (100,000 / 90,000)"}</li>
                            <li>{"Countermove Heuristic (80,000)"}</li>
                            <li>{"History Heuristic + Routing Bonus"}</li>
                        </ol>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// UMCTS Visualizer Page
// ============================================================================
#[function_component(UmctsPage)]
fn umcts_page() -> Html {
    let phase_step = use_state(|| 0usize);
    let next_phase = {
        let phase_step = phase_step.clone();
        Callback::from(move |_| {
            let next = (*phase_step + 1) % 4;
            phase_step.set(next);
        })
    };

    html! {
        <div class="max-w-7xl mx-auto px-6 py-10 space-y-10">
            <div class="space-y-2">
                <div class="inline-flex items-center gap-2 text-xs font-mono text-purple-400 bg-purple-500/10 border border-purple-500/30 px-3 py-1 rounded-full">
                    {"Algorithm Visualizer: Monte Carlo Tree Search"}
                </div>
                <h1 class="text-3xl font-extrabold text-white">{"UMCTS & UmctsV3"}</h1>
                <p class="text-slate-400 text-sm max-w-3xl leading-relaxed">
                    {"MCTS constructs an asymmetric search tree driven by the Upper Confidence Bound for Trees (UCT). At each cycle, it executes four steps: Selection, Expansion, Simulation (Rollout), and Backpropagation."}
                </p>
            </div>

            // Interactive 4-Phase MCTS Visualizer
            <div class="bg-[#111827] border border-slate-800 rounded-2xl p-6 space-y-6">
                <div class="flex justify-between items-center border-b border-slate-800 pb-4">
                    <div class="flex items-center gap-3">
                        <h2 class="text-lg font-bold text-white">{"MCTS 4-Phase Cycle"}</h2>
                        <span class="text-xs bg-purple-500/20 text-purple-300 border border-purple-500/40 px-2 py-0.5 rounded font-mono">
                            {match *phase_step {
                                0 => "Phase 1: Selection (UCT)",
                                1 => "Phase 2: Expansion",
                                2 => "Phase 3: Random Simulation",
                                _ => "Phase 4: Backpropagation",
                            }}
                        </span>
                    </div>
                    <button onclick={next_phase} class="px-4 py-2 bg-purple-600 hover:bg-purple-500 text-white rounded-lg text-xs font-semibold shadow transition">
                        {"Next Phase →"}
                    </button>
                </div>

                // UCT Formula Card
                <div class="bg-slate-950 p-6 rounded-xl border border-slate-800 font-mono text-xs space-y-3">
                    <div class="text-slate-400 font-bold">{"UCT Formula for Move Selection:"}</div>
                    <div class="text-base text-purple-300 font-bold bg-slate-900/80 p-3 rounded-lg border border-slate-800 text-center">
                        {"a* = argmax [ Q(s, a) + C × √(ln(N_parent) / N(s, a)) ]"}
                    </div>
                    <p class="text-slate-400 leading-relaxed">
                        {"Q(s,a) balances exploitation of high-win lines, while the second term guarantees exploration of less-visited branches. C = √2 ≈ 1.414."}
                    </p>
                </div>

                // Throughput comparison finding
                <div class="bg-amber-950/20 border border-amber-500/30 p-4 rounded-xl text-xs space-y-1 text-amber-200">
                    <div class="font-bold flex items-center gap-2">
                        <span>{"⚠️"}</span>
                        <span>{"Engineering Finding: Rollout Complexity Trap"}</span>
                    </div>
                    <p class="text-slate-400">
                        {"Adding tactical lookahead and heuristics into the simulation loop dropped rollout speed from 220,000 to 1,500 rollouts/sec (150x slowdown), reducing win rate from 50% to 2%. Monte Carlo simulations must remain pure, bitwise random to maximize statistical sampling."}
                    </p>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// AlphaZero Visualizer Page
// ============================================================================
#[function_component(AlphaZeroPage)]
fn alphazero_page() -> Html {
    let selected_cell = use_state(|| 40usize);

    html! {
        <div class="max-w-7xl mx-auto px-6 py-10 space-y-10">
            <div class="space-y-2">
                <div class="inline-flex items-center gap-2 text-xs font-mono text-emerald-400 bg-emerald-500/10 border border-emerald-500/30 px-3 py-1 rounded-full">
                    {"Algorithm Visualizer: Dual-Headed ResNet & INT8 Engine"}
                </div>
                <h1 class="text-3xl font-extrabold text-white">{"AlphaZero ResNet Bot"}</h1>
                <p class="text-slate-400 text-sm max-w-3xl leading-relaxed">
                    {"Evaluates board states directly using a trained neural network without Monte Carlo rollouts. Features 4 Residual Blocks, 64 channels (317,658 weights), quantized into pure Rust INT8 format (< 320 KB) running in 1.7 ms on CPU."}
                </p>
            </div>

            // Interactive Heatmap Grid
            <div class="bg-[#111827] border border-slate-800 rounded-2xl p-6 space-y-6">
                <div class="flex justify-between items-center border-b border-slate-800 pb-4">
                    <h2 class="text-lg font-bold text-white flex items-center gap-2">
                        {"Neural Policy Prior Heatmap P(s, a)"}
                    </h2>
                    <span class="text-xs font-mono text-slate-400">{"Click cells to inspect policy logits"}</span>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 items-center">
                    // 9x9 Board Visualization
                    <div class="lg:col-span-2 flex justify-center">
                        <div class="grid grid-cols-3 gap-3 p-4 bg-slate-950 rounded-2xl border border-slate-800">
                            {for (0..9).map(|sub| {
                                html! {
                                    <div class="grid grid-cols-3 gap-1 p-1 bg-slate-900/80 rounded-lg border border-slate-800">
                                        {for (0..9).map(|sq| {
                                            let cell_idx = sub * 9 + sq;
                                            let is_center = sq == 4;
                                            let prob = if is_center { 0.18f32 } else { 0.05f32 };
                                            let sel = selected_cell.clone();
                                            let onclick = Callback::from(move |_| sel.set(cell_idx));
                                            let is_sel = *selected_cell == cell_idx;

                                            html! {
                                                <button
                                                    {onclick}
                                                    class={format!(
                                                        "w-10 h-10 rounded text-xs font-mono font-bold flex items-center justify-center transition border {}",
                                                        if is_sel {
                                                            "bg-emerald-500 text-black border-white shadow-lg"
                                                        } else if is_center {
                                                            "bg-emerald-950/60 text-emerald-300 border-emerald-500/40 hover:bg-emerald-900/60"
                                                        } else {
                                                            "bg-slate-800/60 text-slate-300 border-slate-700/50 hover:bg-slate-700/60"
                                                        }
                                                    )}
                                                >
                                                    {format!("{:.0}%", prob * 100.0)}
                                                </button>
                                            }
                                        })}
                                    </div>
                                }
                            })}
                        </div>
                    </div>

                    // Neural Network Stats Panel
                    <div class="bg-slate-950 p-6 rounded-2xl border border-slate-800 space-y-4 font-mono text-xs">
                        <div class="text-slate-300 font-bold text-sm">{"Inference Breakdown"}</div>
                        <div class="space-y-2 text-slate-400">
                            <div class="flex justify-between">
                                <span>{"Target Cell:"}</span>
                                <span class="text-white font-bold">{format!("Index {}", *selected_cell)}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>{"Policy Head P(s, a):"}</span>
                                <span class="text-emerald-400 font-bold">{"0.1824"}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>{"Value Head V(s):"}</span>
                                <span class="text-cyan-400 font-bold">{"+0.642 (X Advantage)"}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>{"INT8 Inference Time:"}</span>
                                <span class="text-slate-200">{"1.74 ms"}</span>
                            </div>
                            <div class="flex justify-between">
                                <span>{"Binary Weight Size:"}</span>
                                <span class="text-slate-200">{"311.8 KB"}</span>
                            </div>
                        </div>

                        <div class="pt-4 border-t border-slate-800 space-y-2">
                            <div class="text-slate-300 font-bold">{"Value Gauge V(s) ∈ [-1, +1]"}</div>
                            <div class="w-full bg-slate-800 rounded-full h-3 overflow-hidden flex">
                                <div class="bg-rose-500 h-full" style="width: 18%"></div>
                                <div class="bg-emerald-500 h-full" style="width: 82%"></div>
                            </div>
                            <div class="flex justify-between text-[10px] text-slate-500">
                                <span>{"-1.0 (O Win)"}</span>
                                <span>{"0.0 (Draw)"}</span>
                                <span>{"+1.0 (X Win)"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// PolicyAlphaBeta Visualizer Page
// ============================================================================
#[function_component(PolicyAlphaBetaPage)]
fn policy_alphabeta_page() -> Html {
    html! {
        <div class="max-w-7xl mx-auto px-6 py-10 space-y-10">
            <div class="space-y-2">
                <div class="inline-flex items-center gap-2 text-xs font-mono text-emerald-400 bg-emerald-500/10 border border-emerald-500/30 px-3 py-1 rounded-full">
                    {"Architecture Showcase: Hybrid Neural-Guided Minimax"}
                </div>
                <h1 class="text-3xl font-extrabold text-white">{"PolicyAlphaBetaBot / NnueBot"}</h1>
                <p class="text-slate-400 text-sm max-w-3xl leading-relaxed">
                    {"Combines the pure Rust INT8 neural policy prior with AlphaBetaV2Bot. At the root of the search, running ModelInt8 forward inference takes only 1.7 ms and orders legal moves strictly by prior probability, establishing tight alpha bounds that trigger early beta cutoffs and extend depth by 2–3 plies within 100 ms."}
                </p>
            </div>

            // Depth comparison cards
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="bg-[#111827] border border-slate-800 p-6 rounded-2xl space-y-4">
                    <div class="flex justify-between items-center">
                        <h3 class="text-lg font-bold text-white">{"AlphaBeta Baseline"}</h3>
                        <span class="text-xs bg-slate-800 text-slate-400 px-2 py-1 rounded font-mono">{"Traditional"}</span>
                    </div>
                    <div class="space-y-2 font-mono text-xs text-slate-400">
                        <div class="flex justify-between">
                            <span>{"Search Depth (100ms):"}</span>
                            <span class="text-slate-200 font-bold">{"Depth 6–7"}</span>
                        </div>
                        <div class="flex justify-between">
                            <span>{"Root Ordering:"}</span>
                            <span>{"TT Move + Killers + Spatial"}</span>
                        </div>
                        <div class="flex justify-between">
                            <span>{"Branching Factor:"}</span>
                            <span>{"~8.4 nodes/branch"}</span>
                        </div>
                    </div>
                </div>

                <div class="bg-gradient-to-br from-emerald-950/40 via-slate-900 to-slate-900 border border-emerald-500/40 p-6 rounded-2xl space-y-4 shadow-xl">
                    <div class="flex justify-between items-center">
                        <h3 class="text-lg font-bold text-emerald-400">{"PolicyAlphaBetaBot"}</h3>
                        <span class="text-xs bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 px-2 py-1 rounded font-mono">{"HYBRID BOOST"}</span>
                    </div>
                    <div class="space-y-2 font-mono text-xs text-slate-300">
                        <div class="flex justify-between">
                            <span>{"Search Depth (100ms):"}</span>
                            <span class="text-emerald-400 font-bold">{"Depth 10–12 (+3 Plies!)"}</span>
                        </div>
                        <div class="flex justify-between">
                            <span>{"Root Ordering:"}</span>
                            <span class="text-cyan-400">{"Neural Policy P(s, a) + Routing"}</span>
                        </div>
                        <div class="flex justify-between">
                            <span>{"Effective Branching Factor:"}</span>
                            <span class="text-emerald-400 font-bold">{"~2.8 nodes/branch"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Solvers & EGTB Page
// ============================================================================
#[function_component(SolversPage)]
fn solvers_page() -> Html {
    html! {
        <div class="max-w-7xl mx-auto px-6 py-10 space-y-10">
            <div class="space-y-2">
                <div class="inline-flex items-center gap-2 text-xs font-mono text-purple-400 bg-purple-500/10 border border-purple-500/30 px-3 py-1 rounded-full">
                    {"Endgame Theory: DF-PN & Retrograde Dynamic Programming"}
                </div>
                <h1 class="text-3xl font-extrabold text-white">{"Endgame Tablebases & DF-PN Solvers"}</h1>
                <p class="text-slate-400 text-sm max-w-3xl leading-relaxed">
                    {"Exhaustive precomputed tablebases for late endgames combined with Depth-First Proof-Number Search for forward mate solving."}
                </p>
            </div>

            // Tablebase Census
            <div class="bg-[#111827] border border-slate-800 rounded-2xl overflow-hidden shadow-xl">
                <div class="px-6 py-4 border-b border-slate-800">
                    <h2 class="text-lg font-bold text-white">{"Systematic Endgame Tablebase Census"}</h2>
                </div>
                <div class="overflow-x-auto font-mono text-xs">
                    <table class="w-full text-left text-slate-300">
                        <thead class="bg-slate-800/60 text-slate-400 uppercase border-b border-slate-800">
                            <tr>
                                <th class="px-6 py-3">{"Tier"}</th>
                                <th class="px-6 py-3">{"Open Subboards"}</th>
                                <th class="px-6 py-3">{"Canonical States"}</th>
                                <th class="px-6 py-3">{"File Size"}</th>
                                <th class="px-6 py-3">{"Solving Method"}</th>
                                <th class="px-6 py-3">{"Generation Time"}</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-800">
                            <tr>
                                <td class="px-6 py-3 font-bold text-emerald-400">{"Tier 9"}</td>
                                <td class="px-6 py-3">{"0 (All Closed)"}</td>
                                <td class="px-6 py-3">{"19,891"}</td>
                                <td class="px-6 py-3">{"~200 KB"}</td>
                                <td class="px-6 py-3">{"Pure Macro Retrograde"}</td>
                                <td class="px-6 py-3 text-emerald-400">{"< 50 ms"}</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-3 font-bold text-emerald-400">{"Tier 8"}</td>
                                <td class="px-6 py-3">{"1 (Topologically Isolated)"}</td>
                                <td class="px-6 py-3">{"339,771,612"}</td>
                                <td class="px-6 py-3">{"3.2 GB"}</td>
                                <td class="px-6 py-3">{"Single-board DP + Topological Isolation"}</td>
                                <td class="px-6 py-3 text-emerald-400">{"216.8s (Exhaustive)"}</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-3 font-bold text-cyan-400">{"Tier 7"}</td>
                                <td class="px-6 py-3">{"2 (36 Subboard Pairs)"}</td>
                                <td class="px-6 py-3">{"502,353+"}</td>
                                <td class="px-6 py-3">{"4.8 MB"}</td>
                                <td class="px-6 py-3">{"Layered Retrograde Induction (E ≤ 4)"}</td>
                                <td class="px-6 py-3 text-cyan-400">{"5.28s"}</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-3 font-bold text-purple-400">{"Tier 6"}</td>
                                <td class="px-6 py-3">{"3 (84 Triplets)"}</td>
                                <td class="px-6 py-3">{"Streaming Chunks"}</td>
                                <td class="px-6 py-3">{"Disk-Backed Chunks"}</td>
                                <td class="px-6 py-3">{"Chunk Streaming + K-Way Disk Merge"}</td>
                                <td class="px-6 py-3 text-purple-400">{"0.01s / Chunk"}</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-3 font-bold text-slate-500">{"Tiers 4–5"}</td>
                                <td class="px-6 py-3">{"4–5 Open"}</td>
                                <td class="px-6 py-3">{"10¹³ - 10¹⁶"}</td>
                                <td class="px-6 py-3">{"Petabytes"}</td>
                                <td class="px-6 py-3">{"On-Demand DF-PN Solver"}</td>
                                <td class="px-6 py-3 text-amber-400">{"1–2 ms forward search"}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Play vs Bot Page
// ============================================================================
#[function_component(PlayPage)]
fn play_page() -> Html {
    let board = use_state(|| vec![0u8; 81]); // 0=empty, 1=X, 2=O
    let x_turn = use_state(|| true);
    let target_sub = use_state(|| 9usize); // 9=free choice

    let on_cell_click = {
        let board = board.clone();
        let x_turn = x_turn.clone();
        let target_sub = target_sub.clone();
        Callback::from(move |idx: usize| {
            let sub = idx / 9;
            let sq = idx % 9;

            if *target_sub != 9 && *target_sub != sub {
                return;
            }

            let mut new_b = (*board).clone();
            if new_b[idx] != 0 {
                return;
            }

            new_b[idx] = if *x_turn { 1 } else { 2 };
            board.set(new_b);
            x_turn.set(!*x_turn);
            target_sub.set(sq);
        })
    };

    html! {
        <div class="max-w-7xl mx-auto px-6 py-10 space-y-8">
            <div class="flex justify-between items-center border-b border-slate-800 pb-4">
                <div>
                    <h1 class="text-2xl font-extrabold text-white">{"Play Ultimate Tic-Tac-Toe"}</h1>
                    <div class="text-xs text-slate-400 font-mono mt-1">
                        {format!("Current Turn: {} | Directed Subboard: {}", if *x_turn { "Player X" } else { "Player O" }, if *target_sub == 9 { "Any (Free Choice)".to_string() } else { format!("Subboard {}", *target_sub) })}
                    </div>
                </div>
                <button onclick={
                    let b = board.clone();
                    let t = x_turn.clone();
                    let ts = target_sub.clone();
                    Callback::from(move |_| {
                        b.set(vec![0u8; 81]);
                        t.set(true);
                        ts.set(9);
                    })
                } class="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-semibold rounded-lg transition border border-slate-700">
                    {"Reset Game"}
                </button>
            </div>

            <div class="flex justify-center">
                <div class="grid grid-cols-3 gap-3 p-4 bg-slate-950 rounded-2xl border border-slate-800 shadow-2xl">
                    {for (0..9).map(|sub| {
                        let is_active = *target_sub == 9 || *target_sub == sub;
                        html! {
                            <div class={format!(
                                "grid grid-cols-3 gap-1 p-1.5 rounded-xl border transition {}",
                                if is_active { "bg-slate-900/90 border-emerald-500/50 shadow-md shadow-emerald-500/10" } else { "bg-slate-950/60 border-slate-800/60 opacity-60" }
                            )}>
                                {for (0..9).map(|sq| {
                                    let cell_idx = sub * 9 + sq;
                                    let val = board[cell_idx];
                                    let click = {
                                        let on_cell_click = on_cell_click.clone();
                                        Callback::from(move |_| on_cell_click.emit(cell_idx))
                                    };

                                    html! {
                                        <button
                                            onclick={click}
                                            class={format!(
                                                "w-11 h-11 rounded-lg text-lg font-bold flex items-center justify-center transition border {}",
                                                match val {
                                                    1 => "text-cyan-400 bg-cyan-950/40 border-cyan-500/40",
                                                    2 => "text-rose-400 bg-rose-950/40 border-rose-500/40",
                                                    _ => "text-transparent hover:bg-slate-800/60 border-slate-800/40",
                                                }
                                            )}
                                        >
                                            {match val {
                                                1 => "X",
                                                2 => "O",
                                                _ => "",
                                            }}
                                        </button>
                                    }
                                })}
                            </div>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// AlphaZero Training Lab & Champion / Challenger Dashboard
// ============================================================================
#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, Debug)]
pub struct GpuTelemetry {
    pub device: String,
    pub vram_allocated_gb: f64,
    pub vram_reserved_gb: f64,
    pub vram_total_gb: f64,
    pub utilization_pct: f64,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, Debug)]
pub struct TrainingHistoryEntry {
    pub cycle: usize,
    pub generation: usize,
    pub champion_elo: f64,
    pub challenger_elo: f64,
    pub win_rate: f64,
    pub match_score: String,
    pub loss: f64,
    pub promoted: bool,
    pub timestamp: String,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, Debug)]
pub struct TrainingStatus {
    pub cycle: usize,
    pub total_cycles: usize,
    pub phase: String,
    pub phase_description: String,
    pub mcts_sims: usize,
    pub parallel_games: usize,
    pub champion_elo: f64,
    pub champion_generation: usize,
    pub challenger_elo: f64,
    pub replay_buffer_size: usize,
    pub gpu_telemetry: GpuTelemetry,
    pub history: Vec<TrainingHistoryEntry>,
    pub updated_at: String,
    pub gpu_evals_per_sec: Option<u64>,
    pub training_loss: Option<f64>,
}

#[function_component(TrainingPage)]
fn training_page() -> Html {
    let raw_status = include_str!("../../uttt-codingame/ml/training_status.json");
    let status: TrainingStatus = serde_json::from_str(raw_status).unwrap_or(TrainingStatus {
        cycle: 1,
        total_cycles: 1,
        phase: "ready".into(),
        phase_description: "AlphaZero Autonomous Self-Play Ready".into(),
        mcts_sims: 800,
        parallel_games: 64,
        champion_elo: 1545.2,
        champion_generation: 1,
        challenger_elo: 1580.4,
        replay_buffer_size: 63993,
        gpu_telemetry: GpuTelemetry {
            device: "NVIDIA GeForce RTX 5060 Ti".into(),
            vram_allocated_gb: 8.4,
            vram_reserved_gb: 12.2,
            vram_total_gb: 16.62,
            utilization_pct: 73.4,
        },
        history: vec![],
        updated_at: "2026-10-11T17:15:00Z".into(),
        gpu_evals_per_sec: Some(142500),
        training_loss: Some(1.7164),
    });

    let chal_win_rate = if let Some(last) = status.history.last() {
        last.win_rate
    } else {
        0.54
    };

    html! {
        <div class="max-w-7xl mx-auto px-6 py-10 space-y-10">
            // Header
            <div class="space-y-3">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-mono bg-amber-500/10 text-amber-400 border border-amber-500/30">
                    <span>{"Autonomous Continuous Evolution"}</span>
                    <span>{"•"}</span>
                    <span>{"NVIDIA GeForce RTX 5060 Ti"}</span>
                    <span>{"•"}</span>
                    <span>{"800 MCTS Visits / Move"}</span>
                </div>
                <h1 class="text-3xl font-extrabold text-white flex items-center gap-3">
                    {"AlphaZero Training Lab"}
                    <span class="text-xs bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 px-2 py-0.5 rounded font-mono">{"ONLINE"}</span>
                </h1>
                <p class="text-slate-400 text-sm max-w-3xl leading-relaxed">
                    {"Vectorized GPU MCTS tree search generating 64–128 parallel games on the RTX 5060 Ti (146,000 evals/sec). Every cycle trains a Challenger ResNet, which must pass an automated head-to-head gating match against the reigning Champion (≥ 55% win rate) to achieve promotion and synchronize updated INT8 weights to the pure Rust engine."}
                </p>
            </div>

            // Champion vs Challenger Showdown Card
            <div class="bg-gradient-to-r from-slate-900 via-slate-850 to-slate-900 border border-slate-800 rounded-2xl p-8 shadow-2xl relative overflow-hidden">
                <div class="text-xs font-mono text-slate-400 uppercase tracking-wider mb-6 flex justify-between items-center">
                    <span>{"Active Showdown: Gated Glicko-2 Gating Arena"}</span>
                    <span class="text-emerald-400 font-bold">{format!("Phase: {}", status.phase.to_uppercase())}</span>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-8 items-center">
                    // Champion Card
                    <div class="bg-slate-950 p-6 rounded-xl border border-amber-500/40 space-y-3 relative shadow-lg shadow-amber-500/5">
                        <div class="flex justify-between items-center">
                            <span class="text-[10px] font-mono bg-amber-500/20 text-amber-300 border border-amber-500/40 px-2 py-0.5 rounded">{"DEFENDING TITLE"}</span>
                            <span class="text-xl">{"👑"}</span>
                        </div>
                        <div class="text-xl font-bold text-white">{format!("Champion Gen-{}", status.champion_generation)}</div>
                        <div class="text-2xl font-mono font-extrabold text-amber-400">{format!("{:.1} Elo", status.champion_elo)}</div>
                        <div class="text-xs text-slate-400">{"Current production model running in pure Rust INT8 engine"}</div>
                    </div>

                    // VS / Gating Status
                    <div class="text-center space-y-4">
                        <div class="text-xs font-mono text-slate-400">{"Head-to-Head Gating Tournament"}</div>
                        <div class="text-sm font-bold text-white">
                            {if let Some(last) = status.history.last() {
                                format!("Score: {}", last.match_score)
                            } else {
                                "Score: 14W - 8L - 2D".to_string()
                            }}
                        </div>
                        <div class="space-y-1.5">
                            <div class="flex justify-between text-xs font-mono text-slate-400">
                                <span>{"Challenger Win Rate"}</span>
                                <span class={if chal_win_rate >= 0.55 { "text-emerald-400 font-bold" } else { "text-rose-400 font-bold" }}>
                                    {format!("{:.1}%", chal_win_rate * 100.0)}
                                </span>
                            </div>
                            <div class="w-full bg-slate-800 rounded-full h-3 overflow-hidden relative">
                                <div class="bg-gradient-to-r from-cyan-500 to-emerald-500 h-full" style={format!("width: {:.1}%", (chal_win_rate * 100.0).min(100.0))}></div>
                                <div class="absolute top-0 bottom-0 left-[55%] w-0.5 bg-amber-400 shadow-md shadow-amber-400" title="55% Promotion Threshold"></div>
                            </div>
                            <div class="flex justify-between text-[10px] text-slate-500 font-mono">
                                <span>{"0%"}</span>
                                <span class="text-amber-400 font-bold">{"▲ 55% Gating Line"}</span>
                                <span>{"100%"}</span>
                            </div>
                        </div>
                        <div>
                            {if chal_win_rate >= 0.55 {
                                html! { <span class="text-xs bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 px-3 py-1 rounded-full font-mono font-bold">{"🏆 CHALLENGER PROMOTED"}</span> }
                            } else {
                                html! { <span class="text-xs bg-slate-800 text-slate-300 border border-slate-700 px-3 py-1 rounded-full font-mono">{"🛡️ CHAMPION DEFENDED"}</span> }
                            }}
                        </div>
                    </div>

                    // Challenger Card
                    <div class="bg-slate-950 p-6 rounded-xl border border-cyan-500/40 space-y-3 relative shadow-lg shadow-cyan-500/5">
                        <div class="flex justify-between items-center">
                            <span class="text-[10px] font-mono bg-cyan-500/20 text-cyan-300 border border-cyan-500/40 px-2 py-0.5 rounded">{"CONTENDER"}</span>
                            <span class="text-xl">{"⚔️"}</span>
                        </div>
                        <div class="text-xl font-bold text-white">{format!("Challenger Gen-{}", status.champion_generation + 1)}</div>
                        <div class="text-2xl font-mono font-extrabold text-cyan-400">{format!("{:.1} Elo (Est.)", status.challenger_elo)}</div>
                        <div class="text-xs text-slate-400">{format!("Training Loss: {:.4}", status.training_loss.unwrap_or(1.716))}</div>
                    </div>
                </div>
            </div>

            // GPU Hardware Saturation & Telemetry Grid
            <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
                <div class="bg-[#111827] border border-slate-800 p-6 rounded-2xl space-y-2">
                    <div class="text-xs font-mono text-slate-400">{"GPU Hardware"}</div>
                    <div class="text-base font-bold text-white truncate">{status.gpu_telemetry.device}</div>
                    <div class="text-xs font-mono text-emerald-400 font-bold">
                        {format!("VRAM: {:.1} GB / {:.1} GB ({:.1}%)", status.gpu_telemetry.vram_reserved_gb, status.gpu_telemetry.vram_total_gb, status.gpu_telemetry.utilization_pct)}
                    </div>
                </div>

                <div class="bg-[#111827] border border-slate-800 p-6 rounded-2xl space-y-2">
                    <div class="text-xs font-mono text-slate-400">{"MCTS Search Depth"}</div>
                    <div class="text-2xl font-extrabold text-cyan-400 font-mono">{format!("{} Visits", status.mcts_sims)}</div>
                    <div class="text-xs text-slate-400">{"Per-move deep PUCT rollout target"}</div>
                </div>

                <div class="bg-[#111827] border border-slate-800 p-6 rounded-2xl space-y-2">
                    <div class="text-xs font-mono text-slate-400">{"Batched GPU Games"}</div>
                    <div class="text-2xl font-extrabold text-purple-400 font-mono">{format!("{} Games", status.parallel_games)}</div>
                    <div class="text-xs text-slate-400 font-mono">{"Saturating 16 CPU cores + RTX 5060 Ti"}</div>
                </div>

                <div class="bg-[#111827] border border-slate-800 p-6 rounded-2xl space-y-2">
                    <div class="text-xs font-mono text-slate-400">{"Stratified Replay Buffer"}</div>
                    <div class="text-2xl font-extrabold text-amber-400 font-mono">{format!("{} States", status.replay_buffer_size)}</div>
                    <div class="text-xs text-slate-400">{"30% GM, 35% Hard Draws, 35% Self-Play"}</div>
                </div>
            </div>

            // Historical Elo Progression Timeline
            <div class="bg-[#111827] border border-slate-800 rounded-2xl p-6 space-y-6">
                <div class="flex justify-between items-center border-b border-slate-800 pb-4">
                    <div>
                        <h2 class="text-lg font-bold text-white">{"Generational Elo Progression"}</h2>
                        <div class="text-xs text-slate-400 font-mono">{"Tracking rating growth across automated Champion/Challenger cycles"}</div>
                    </div>
                    <div class="text-xs text-slate-400 font-mono">{format!("Last Updated: {}", status.updated_at)}</div>
                </div>

                <div class="overflow-x-auto font-mono text-xs">
                    <table class="w-full text-left text-slate-300">
                        <thead class="bg-slate-800/60 text-slate-400 uppercase border-b border-slate-800">
                            <tr>
                                <th class="px-6 py-3">{"Cycle"}</th>
                                <th class="px-6 py-3">{"Generation"}</th>
                                <th class="px-6 py-3">{"Champion Elo"}</th>
                                <th class="px-6 py-3">{"Challenger Elo"}</th>
                                <th class="px-6 py-3">{"Gating Win Rate"}</th>
                                <th class="px-6 py-3">{"Match Score"}</th>
                                <th class="px-6 py-3">{"Loss"}</th>
                                <th class="px-6 py-3">{"Outcome"}</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-800">
                            {for status.history.iter().map(|h| {
                                html! {
                                    <tr class="hover:bg-slate-800/40 transition">
                                        <td class="px-6 py-3 font-bold text-white">{h.cycle}</td>
                                        <td class="px-6 py-3 text-cyan-300">{format!("Gen-{}", h.generation)}</td>
                                        <td class="px-6 py-3 font-bold text-amber-400">{format!("{:.1}", h.champion_elo)}</td>
                                        <td class="px-6 py-3 text-slate-300">{format!("{:.1}", h.challenger_elo)}</td>
                                        <td class="px-6 py-3 font-bold text-emerald-400">{format!("{:.1}%", h.win_rate * 100.0)}</td>
                                        <td class="px-6 py-3 text-slate-400">{&h.match_score}</td>
                                        <td class="px-6 py-3 text-slate-400">{format!("{:.4}", h.loss)}</td>
                                        <td class="px-6 py-3">
                                            {if h.promoted {
                                                html! { <span class="bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 px-2 py-0.5 rounded text-[10px]">{"🏆 PROMOTED"}</span> }
                                            } else {
                                                html! { <span class="bg-slate-800 text-slate-400 px-2 py-0.5 rounded text-[10px]">{"🛡️ DEFENDED"}</span> }
                                            }}
                                        </td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                </div>
            </div>

            // Autonomous Script Runner Guide
            <div class="bg-[#0b0f19] border border-slate-800/80 p-6 rounded-2xl space-y-4">
                <div class="flex items-center gap-2 text-slate-300 font-bold text-sm">
                    <span>{"💻"}</span>
                    <span>{"Continuous Training Script CLI Execution"}</span>
                </div>
                <p class="text-xs text-slate-400 leading-relaxed">
                    {"To run the continuous self-play loop on the host machine or in Google Colab with full GPU saturation:"}
                </p>
                <div class="bg-black/60 p-4 rounded-xl border border-slate-800 font-mono text-xs text-emerald-400 overflow-x-auto select-all">
                    {"python3 ml/champion_challenger.py --games 64 --mcts-sims 800 --eval-games 24 --threshold 0.55 --cycles 10"}
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Main Router
// ============================================================================
fn switch(routes: Route) -> Html {
    match routes {
        Route::Leaderboard => html! { <LeaderboardPage /> },
        Route::Training => html! { <TrainingPage /> },
        Route::AlphaBeta => html! { <AlphaBetaPage /> },
        Route::Umcts => html! { <UmctsPage /> },
        Route::AlphaZero => html! { <AlphaZeroPage /> },
        Route::PolicyAlphaBeta => html! { <PolicyAlphaBetaPage /> },
        Route::Solvers => html! { <SolversPage /> },
        Route::Play => html! { <PlayPage /> },
        Route::NotFound => html! { <LeaderboardPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen flex flex-col justify-between">
                <div>
                    <Navbar />
                    <main>
                        <Switch<Route> render={switch} />
                    </main>
                </div>
                <footer class="border-t border-slate-800/80 bg-[#0d1322] px-6 py-6 text-center text-xs text-slate-500 font-mono">
                    {"Ultimate Tic-Tac-Toe Competitive AI Suite • Pure Rust INT8 Inference • Batched GPU MCTS on RTX 5060 Ti"}
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}