use super::widgets::{Note, TextField};
use leptos::prelude::*;
use praxis_core::crypto::numtheory::{crt, factorize, lcm, mod_inverse};

#[component]
pub fn CrtLab() -> impl IntoView {
    let spec = RwSignal::new("2 mod 3, 3 mod 5, 2 mod 7".to_string());
    let parsed = Memo::new(move |_| -> Result<Vec<(i64, i64)>, String> {
        spec.get()
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                let parts: Vec<&str> = s.split("mod").map(|p| p.trim()).collect();
                if parts.len() != 2 {
                    return Err(format!("Cannot read “{}”. Use the form r mod m.", s.trim()));
                }
                let r = parts[0]
                    .parse::<i64>()
                    .map_err(|_| format!("“{}” is not a valid residue.", parts[0]))?;
                let m = parts[1]
                    .parse::<i64>()
                    .map_err(|_| format!("“{}” is not a valid modulus.", parts[1]))?;
                if m <= 0 {
                    return Err("Each modulus must be positive.".into());
                }
                Ok((r, m))
            })
            .collect()
    });
    let inv_a = RwSignal::new("7".to_string());
    let inv_m = RwSignal::new("26".to_string());
    view! {
        <div class="lab">
            <div class="lab-controls">
                <TextField label="Congruences" value=spec mono=true hint="Separate them with commas: r mod m"/>
            </div>
            {move || match parsed.get() {
                Err(e) => view! { <p class="lab-error" role="alert">{e}</p> }.into_any(),
                Ok(cs) => {
                    let total = cs.iter().fold(1i64, |acc, &(_, m)| lcm(acc, m));
                    match crt(&cs) {
                        Some((x, m)) => view! {
                            <div class="stat-row">
                                <div class="stat"><div class="stat-label mono">"solution"</div><div class="stat-value mono">{format!("x ≡ {x} (mod {m})")}</div></div>
                                <div class="stat"><div class="stat-label mono">"lcm of moduli"</div><div class="stat-value mono">{total}</div><div class="stat-sub mono">{format!("{:?}", factorize(total as u64))}</div></div>
                            </div>
                            <div class="table-scroll"><table class="mini-table mono"><thead><tr><th>"congruence"</th><th>"check"</th></tr></thead><tbody>
                                {cs.iter().map(|&(r, mm)| view! { <tr><td>{format!("x ≡ {r} (mod {mm})")}</td><td>{format!("{x} mod {mm} = {} {}", x.rem_euclid(mm), if x.rem_euclid(mm) == r.rem_euclid(mm) { "✓" } else { "✗" })}</td></tr> }).collect_view()}
                            </tbody></table></div>
                        }.into_any(),
                        None => view! { <p class="lab-error" role="alert">"There is no solution. Two congruences disagree on a factor shared by their moduli."</p> }.into_any(),
                    }
                }
            }}
            <div class="lab-viz">
                <div class="lab-viz-head mono">"Modular inverse"</div>
                <div class="lab-fields lab-fields-3">
                    <TextField label="a" value=inv_a mono=true/>
                    <TextField label="m" value=inv_m mono=true/>
                    <div class="field">
                        <span class="field-label" id="crt-inverse-label">"a⁻¹ mod m"</span>
                        <output class="field-input field-output mono" aria-labelledby="crt-inverse-label">{move || match (inv_a.get().trim().parse::<i64>(), inv_m.get().trim().parse::<i64>()) { (Ok(a), Ok(m)) => mod_inverse(a, m).map(|i| i.to_string()).unwrap_or_else(|| "No inverse".into()), _ => "—".into() }}</output>
                    </div>
                </div>
            </div>
            <Note>"The Chinese remainder theorem combines residues for pairwise coprime moduli into one residue. If the moduli share factors, the congruences may have no common solution. In cryptanalysis, this can combine partial key results modulo small primes. The same idea explains why mixed periods join at their least common multiple."</Note>
        </div>
    }
}
