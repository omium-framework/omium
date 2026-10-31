# Inertia adapter spike report (Phase 6)

**Audience:** project owner (Turkish summary below) and implementers.  
**Status:** spike complete — **owner approval still required** before production use in `omium-inertia` (D-013).

## Owner summary (Türkçe)

Bu rapor, Laravel’de Inertia kullanan bir geliştiricinin sorusuna cevap verir: **“Rust tarafında hangi paketle Inertia protokolünü konuşacağız?”**

İki aday incelendi:

| | **inertia-omega** | **inertia_rs** |
|---|-------------------|----------------|
| Kim | Resmi Inertia.js ekibinin Rust portu (Laravel adapter’dan) | Topluluk / bağımsız crate |
| crates.io | Henüz yok (git/path) | Var (`0.4.0`) |
| Olgunluk | ALPHA | Daha küçük ama yayınlanmış |
| MSRV | **1.99** (çok yeni) | **1.88** |
| Laravel benzerliği | Çok yüksek (`props!`, defer, flash, SSR, Vite yardımcısı) | Orta (temel sayfa + Axum katmanı) |

**Repoda ne yaptık?**

- `examples/inertia-spike`: Axum + `inertia_rs` ile **çalışan mini sunucu** (port 3001). İlk ziyaret HTML, `X-Inertia: true` ile JSON — testlerle doğrulandı.
- `inertia-omega` için kod eklemedik: toolchain MSRV 1.99 ve git bağımlılığı Phase 6 “kanıt” için riskli; davranış README ve resmi dokümantasyondan matrise işlendi.

**Öneri (onayınıza sunulur — henüz APPROVED değil):**

1. **Uzun vadede hedef:** `inertia-omega` — protokol ve Laravel parity resmi tarafta.
2. **MSRV 1.99 kabul edilene kadar veya omega crates.io’ya çıkana kadar:** Phase 7 denemelerinde `inertia_rs` ile ilerlenebilir; `omium-inertia` arkasında **trait adapter** ile ikisi de sarılabilir.
3. **Onayınız olmadan** `sample-app` veya CLI generator’a Inertia bağlamıyoruz.

**Sizden istenen karar:** D-013 için “omega”, “inertia_rs”, veya “önce MSRV politikasını netleştir” yanıtı.

---

## Evidence in this repository

| Artifact | Purpose |
| --- | --- |
| `examples/inertia-spike/` | Runnable Axum POC + protocol tests (`cargo test -p inertia-spike`) |
| `crates/omium-inertia/src/adapter.rs` | Framework-owned trait sketch (wrap target) |
| This document | Protocol matrix and recommendation |

## Candidate profiles

### inertia-omega (official)

- **Source:** https://github.com/inertiajs/inertia-omega  
- **License:** MIT  
- **Distribution:** Git/path; crate name `inertia-omega`, library `inertia`  
- **MSRV:** 1.99 (from upstream `Cargo.toml`, re-check before adoption)  
- **Maintainer:** Inertia.js organization (ALPHA)  
- **Axum:** First-party `InertiaLayer`, extractor, `IntoResponse`  
- **Docs quality:** Extensive README (Laravel mapping, props, SSR, testing)

### inertia_rs

- **Source:** https://github.com/stuarth/inertia-rs  
- **License:** MIT  
- **Distribution:** crates.io `0.4.0`  
- **MSRV:** 1.88  
- **Default feature:** Rocket; Axum behind `features = ["axum"]`  
- **Maintainer:** Community (stuarth)  
- **Axum:** `InertiaRequest`, `VersionLayer`, `SharedProps` (see upstream `examples/axum-minimal`)

### Also noted (out of D-013 scope)

- `ferro-inertia`, `inertia-axum`, `loco-inertia` — Loco/Ferro ecosystem; not evaluated in depth for this spike.

## Protocol coverage matrix (Inertia v3)

Legend: **Y** supported (per upstream docs or spike test), **P** partial, **N** not found / not ported, **?** not verified in spike code

| Capability | inertia-omega | inertia_rs | Spike test |
| --- | --- | --- | --- |
| Initial HTML + embedded page JSON | Y | Y | Y (`inertia-spike` HTML test) |
| Inertia visit JSON (`X-Inertia`) | Y | Y | Y (`inertia-spike` JSON test) |
| Asset version / 409 stale | Y | Y (`VersionLayer`) | Version header in JSON test |
| Shared props | Y (`Config::share`, middleware) | Y (`SharedProps` extension) | Y (`appName` in JSON) |
| Redirects (303 on writes) | Y | Y (API types) | Not exercised |
| External location (`X-Inertia-Location`) | Y | Y (`Inertia::location`) | Not exercised |
| Validation errors / error bags | Y (`back_with_errors`, bags) | P (protocol headers present; session/flash less documented) | Not exercised |
| Partial reloads | Y | P (request context + prop filtering in core) | Not exercised |
| Deferred props | Y (`defer`, groups) | P (`InertiaProps::defer` in axum-minimal) | Defer key present in API, not asserted in test |
| Merge / deep merge / once | Y | P (builder methods on `Inertia`) | Not exercised |
| History encrypt / clear | Y | Y (API on response) | Not exercised |
| Vite manifest / `@vite` parity | Y (`laravel-omega-vite` sibling) | N / manual | N |
| SSR | Y (`HttpGateway`, optional) | N in spike | N |
| Precognition / DevTools | N (documented) | N | N |
| Wrap behind framework trait | Y (core + adapter split) | Y (core + axum module) | Trait stub in `omium-inertia` |

## Vue / React client compatibility (Phase 6 check)

Inertia **client** libraries (Vue/React) speak the same HTTP protocol. Both adapters target v3 page objects. The spike does **not** run Node/Vite in CI; compatibility is **protocol-level**:

- Page JSON shape from `inertia_rs` matches expected fields (`component`, `props`, `url`, `version`) in tests.
- Full browser check remains a **Phase 7** exit criterion (generated app + Vite).

## Maintenance and risk

| Risk | inertia-omega | inertia_rs |
| --- | --- | --- |
| Breaking changes | ALPHA, API may move | Pre-1.0 semver on 0.4 |
| Supply chain | Git pin required | crates.io |
| MSRV pressure | High (1.99) | Moderate (1.88) |
| Official alignment | Strong | Indirect |
| Test quality upstream | `cargo test --all-features`, AssertablePage | Examples + crate tests on GitHub |

## Framework integration plan (after approval)

1. Implement `InertiaAdapter` trait in `omium-inertia` for render, shared props, version, redirects.
2. Phase 7 generator adds Vite + Vue/React with chosen adapter only.
3. Keep `sample-app` on Askama (standard mode) unless owner requests a second demo app.

## Recommendation to owner (English)

**Proposed default path:** Standardize on **`inertia-omega`** once MSRV policy accepts **Rust 1.99+** (or upstream lowers MSRV) and the crate is published or vendored with a pinned git SHA.

**Bridge path:** Use **`inertia_rs`** for early Phase 7 experiments and CI protocol tests already in `inertia-spike`, behind the same trait, without committing the framework to it permanently.

**Reject both:** If neither is acceptable, next step is a custom adapter from the [official protocol spec](https://thetutlage.github.io/inertia-protocol/) — larger cost; requires separate approval.

---

Record owner choice in `docs/DECISIONS.md` D-013 when ready.
