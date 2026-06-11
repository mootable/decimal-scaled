import io

# ---- 1) WideTrigCore: the ln_fixed_routed_agm trait method ----
p = 'src/algos/support/wide_trig_core.rs'
s = io.open(p, encoding='utf-8', newline='').read()
old = """    /// Directed-rounding narrowing with Ziv escalation, forcing a
    /// confirm recompute even in nearest modes — the acosh / atanh
    /// near-special path (the residual can sit on a rounding boundary).
    fn round_to_storage_directed_near_special("""
new = """    /// Tang/Series-ROUTED working-scale natural log on the wide
    /// composition integer [`Self::Wagm`] — the per-tier
    /// `ln_fixed_routed_agm` (Tang where `policy::ln::is_tang` routes
    /// it, Series otherwise; the per-tier Tang CAP is a macro literal,
    /// which is why this is a trait binding rather than a free generic).
    /// Consumed by the acosh / atanh canonical kernels.
    fn ln_fixed_routed_agm<const SCALE: u32>(v_w: Self::Wagm, w: u32) -> Self::Wagm;

    /// Directed-rounding narrowing with Ziv escalation, forcing a
    /// confirm recompute even in nearest modes — the acosh / atanh
    /// near-special path (the residual can sit on a rounding boundary).
    fn round_to_storage_directed_near_special("""
assert s.count(old) == 1
s = s.replace(old, new)
io.open(p, 'w', encoding='utf-8', newline='').write(s)
print("trait ok")

# ---- 2) macro: the trait impl forwarder + shell delegation ----
p = 'src/macros/wide_transcendental.rs'
s = io.open(p, encoding='utf-8', newline='').read()

old = """                fn round_to_storage_directed_near_special("""
new = """                fn ln_fixed_routed_agm<const SCALE: u32>(v_w: Wagm, w: u32) -> Wagm {
                    ln_fixed_routed_agm::<SCALE>(v_w, w)
                }
                fn round_to_storage_directed_near_special("""
assert s.count(old) == 1
s = s.replace(old, new)

# Shell delegation: replace the six bodies with dispatch one-liners.
start = s.index("""            pub fn sinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let raw = self.to_bits();
                let szero = <$Storage>::from_i128(0);""")
# back up to the doc comment start for sinh_strict_with
doc_start = s.rindex("            /// Mode-aware sibling of [`Self::sinh_strict`].", 0, start)
end = s.index("""            /// Mode-aware sibling of [`Self::to_degrees_strict`].""")

new_block = """            /// Mode-aware sibling of [`Self::sinh_strict`].
            ///
            /// Delegates to the policy dispatch exactly as the default-
            /// mode sibling does (`policy::trig::sinh_dispatch`), so BOTH
            /// public entries share the one canonical kernel
            /// (`hyper_schoolbook::sinh_schoolbook`, which now carries
            /// this shell's former analytic small-argument band, exact
            /// pins, and `never_exact` two-width widening).
            #[inline]
            #[must_use]
            pub fn sinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::sinh_dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::cosh_strict`] — policy
            /// dispatch, see [`Self::sinh_strict_with`].
            #[inline]
            #[must_use]
            pub fn cosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::cosh_dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::tanh_strict`] — policy
            /// dispatch, see [`Self::sinh_strict_with`] (the canonical
            /// kernel carries this shell's former cubic band, all-nines
            /// saturation fast path, and capped exp lift).
            #[inline]
            #[must_use]
            pub fn tanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::tanh_dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::asinh_strict`] — policy
            /// dispatch, see [`Self::sinh_strict_with`].
            #[inline]
            #[must_use]
            pub fn asinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::asinh_dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::acosh_strict`] — policy
            /// dispatch, see [`Self::sinh_strict_with`].
            #[inline]
            #[must_use]
            pub fn acosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::acosh_dispatch::<_, SCALE>(self.to_bits(), mode))
            }

            /// Mode-aware sibling of [`Self::atanh_strict`] — policy
            /// dispatch, see [`Self::sinh_strict_with`].
            #[inline]
            #[must_use]
            pub fn atanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::trig::atanh_dispatch::<_, SCALE>(self.to_bits(), mode))
            }

"""
s = s[:doc_start] + new_block + s[end:]
io.open(p, 'w', encoding='utf-8', newline='').write(s)
print("macro ok")

# ---- 3) policy bound updates: hyper_rung + extra_rung ----
p = 'src/policy/trig.rs'
s = io.open(p, encoding='utf-8', newline='').read()

old = """    ) -> C::Storage
    where
        <C::Wexp as BigInt>::Scratch: ComputeLimbs,
        <C::W as BigInt>::Scratch: ComputeLimbs,
    {"""
n = s.count(old)
assert n == 3, n
new = """    ) -> C::Storage
    where
        <C::Wagm as BigInt>::Scratch: ComputeLimbs,
        <C::Wexp as BigInt>::Scratch: ComputeLimbs,
    {"""
s = s.replace(old, new)

old = """    ) -> C::Storage
    where
        <C::W as crate::int::types::traits::BigInt>::Scratch:
            crate::int::types::compute_limbs::ComputeLimbs,
    {"""
n = s.count(old)
assert n == 3, n  # extra_rung asinh/acosh/atanh
new = """    ) -> C::Storage
    where
        <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
            crate::int::types::compute_limbs::ComputeLimbs,
    {"""
s = s.replace(old, new)
io.open(p, 'w', encoding='utf-8', newline='').write(s)
print("policy bounds ok")
