Third-party software incorporated into this crate.

This file lists upstream sources used (in vendored or adapted form) by
`decimal-scaled`. Each section gives the upstream identification, the
specific commit / version pinned, the license terms in full, and a
pointer to where the vendored material lives in this tree.

================================================================================
1. ConstScaleFpdec / primitive_fixed_point_decimal
================================================================================

Upstream:        https://github.com/WuBingzheng/primitive_fixed_point_decimal
Commit pinned:   3a1cd2d61850317609372f23aa0d14cd75c6962f (v1.4.2, 2026-04-18)
Files used:      src/inner_i128.rs (functions `mul2`, `div_exp_fast_2word`,
                 `div_exp_fast_1word`, plus the `MG_EXP_MAGICS` table)
Adapted into:    crates/decimal-scaled/src/mg_divide.rs

The adapted code implements the Moller-Granlund 2011 magic-number
divide algorithm specialised for divisor = 10^SCALE. See the inline
citation at the top of `mg_divide.rs` and the paper:

    Moller, N. and Granlund, T. (2011). "Improved Division by Invariant
    Integers." IEEE Transactions on Computers, 60(2), 165-175.
    DOI: 10.1109/TC.2010.143.

The adaptation is structural (signature reshaped to be const-generic
over `SCALE` rather than runtime-`i: usize`); the magic-table values
and the algorithm body match the upstream verbatim.

The upstream MIT license text follows in full, retained per its
"copies or substantial portions of the Software" clause.

--------------------------------------------------------------------------------
MIT License

Copyright (c) 2023 Wu Bingzheng

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
--------------------------------------------------------------------------------
