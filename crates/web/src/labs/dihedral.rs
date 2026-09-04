use super::widgets::{Note, NumberField};
use leptos::prelude::*;
use praxis_core::crypto::groups::Dihedral;

#[component]
pub fn DihedralLab() -> impl IntoView {
    let n = RwSignal::new(8i64);
    let a_ref = RwSignal::new(false);
    let a_k = RwSignal::new(1i64);
    let b_ref = RwSignal::new(true);
    let b_k = RwSignal::new(0i64);
    let elem = move |reflect: bool, k: i64| {
        let nn = n.get().max(3) as usize;
        if reflect {
            Dihedral::reflection(nn, k.rem_euclid(nn as i64) as usize)
        } else {
            Dihedral::rotation(nn, k.rem_euclid(nn as i64) as usize)
        }
    };
    let a = Memo::new(move |_| elem(a_ref.get(), a_k.get()));
    let b = Memo::new(move |_| elem(b_ref.get(), b_k.get()));
    let ab = Memo::new(move |_| a.get().compose(&b.get()));
    let ba = Memo::new(move |_| b.get().compose(&a.get()));
    view! {
        <div class="lab">
            <div class="lab-controls">
                <div class="lab-row">
                    <NumberField label="n (positions on the ring)" value=n min=3 max=60/>
                    <div class="field"><span class="field-label">"a"</span><div class="lab-row tight"><label class="filter-toggle"><input type="checkbox" prop:checked=move || a_ref.get() on:change=move |ev| a_ref.set(event_target_checked(&ev))/><span>"reflect"</span></label><NumberField label="k" value=a_k min=0 max=59/></div></div>
                    <div class="field"><span class="field-label">"b"</span><div class="lab-row tight"><label class="filter-toggle"><input type="checkbox" prop:checked=move || b_ref.get() on:change=move |ev| b_ref.set(event_target_checked(&ev))/><span>"reflect"</span></label><NumberField label="k" value=b_k min=0 max=59/></div></div>
                </div>
            </div>
            <div class="stat-row">
                {[("a", a), ("b", b), ("a ∘ b", ab), ("b ∘ a", ba)].into_iter().map(|(label, e)| view! {
                    <div class="stat">
                        <div class="stat-label mono">{label}</div>
                        <div class="stat-value mono">{move || e.get().word()}</div>
                        <div class="stat-sub mono">{move || { let d = e.get(); format!("x ↦ {}x + {} · order {}", if d.reflect { "−" } else { "" }, d.shift, d.order()) }}</div>
                    </div>
                }).collect_view()}
            </div>
            <div class="lab-viz">
                <div class="lab-viz-head mono">{move || format!("Action of a ∘ b on positions 0…{} · |D_{}| = {}", n.get() - 1, n.get(), 2 * n.get())}</div>
                <div class="perm-rows mono">
                    {move || {
                        let e = ab.get();
                        let nn = e.n;
                        view! {
                            <div class="perm-row"><span class="perm-label">"x"</span>{(0..nn).map(|x| view! { <i>{x}</i> }).collect_view()}</div>
                            <div class="perm-row"><span class="perm-label">"(a∘b)(x)"</span>{(0..nn).map(|x| view! { <i class:hot=move || ab.get().apply(x) == x>{e.apply(x)}</i> }).collect_view()}</div>
                            <div class="perm-row"><span class="perm-label">"cycles"</span><span class="perm-cycles">{e.to_perm().cycle_notation()}</span></div>
                        }
                    }}
                </div>
            </div>
            <Note>"Rotations commute; a rotation and a reflection do not, so a ∘ b and b ∘ a generally differ. Dihedral symmetries of a ring of positions are the natural candidates when a cipher is suspected of reading a text around a circular or reversed layout."</Note>
        </div>
    }
}
