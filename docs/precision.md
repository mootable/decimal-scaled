# Precision

**`decimal-scaled` is perfectly precise** — correctly rounded to within 0.5 ULP,
i.e. **0 bits of error**.

<!-- BEGIN GENERATED:precision:stats -->
We execute 103,133 specialised inputs across all 30 functions, on 117 widths and scales, under all 8 rounding modes. Not every input is representable at every width and scale — one needing more significant digits than a tier holds is filtered before it runs — so the graded total is 51,645,712 separate checks.
<!-- END GENERATED:precision:stats -->

Each golden case was gathered from the codebase failing, from theory, from
destructive testing of our own code, and from destructive testing of other
libraries — collated into a behemoth of meanness.

The data on this page is generated and committed directly from the
[golden-comprehensive CI job](https://github.com/mootable/decimal-scaled/actions/workflows/golden-comprehensive.yml)
and auto-rendered here; there is **no manual editing**. If we ever fail our
guarantee, we want you to know.

Each row is a function, each column a storage width. A `✓` means `0` bits of
error — the exact correctly-rounded value — across every scale and all eight
rounding modes at that width, with the number of checks verified correct beneath
it. A `✗` marks a cell with at least one failure, the count of failing checks
beneath. Inputs outside a function's domain or a tier's range aren't checks, so
they count toward neither. The full per-scale, per-mode surface lives in
[`results/golden/`](https://github.com/mootable/decimal-scaled/tree/main/results/golden).

<!-- BEGIN GENERATED:precision:surface -->
| Function | D18 | D38 | D57 | D76 | D115 | D153 | D230 | D307 | D462 | D616 | D924 | D1232 |
| :-- | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| `acos` | ✓<br>19,216 | ✓<br>101,096 | ✓<br>142,336 | ✓<br>145,680 | ✓<br>158,168 | ✓<br>128,448 | ✓<br>160,384 | ✓<br>288,848 | ✓<br>224,632 | ✓<br>256,784 | ✓<br>256,784 | ✓<br>256,784 |
| `acosh` | ✓<br>46,584 | ✓<br>139,176 | ✓<br>146,624 | ✓<br>146,744 | ✓<br>152,016 | ✓<br>117,712 | ✓<br>152,184 | ✓<br>290,152 | ✓<br>221,352 | ✓<br>255,920 | ✓<br>255,952 | ✓<br>255,960 |
| `add` | ✓<br>496 | ✓<br>1,728 | ✓<br>1,728 | ✓<br>1,952 | ✓<br>1,928 | ✓<br>1,688 | ✓<br>2,344 | ✓<br>4,360 | ✓<br>3,688 | ✓<br>4,192 | ✓<br>4,416 | ✓<br>4,600 |
| `asin` | ✓<br>19,616 | ✓<br>106,256 | ✓<br>152,112 | ✓<br>157,368 | ✓<br>170,360 | ✓<br>139,336 | ✓<br>173,896 | ✓<br>313,264 | ✓<br>243,600 | ✓<br>278,496 | ✓<br>278,496 | ✓<br>278,496 |
| `asinh` | ✓<br>16,240 | ✓<br>98,888 | ✓<br>128,584 | ✓<br>145,544 | ✓<br>151,872 | ✓<br>129,560 | ✓<br>170,568 | ✓<br>313,520 | ✓<br>258,072 | ✓<br>292,344 | ✓<br>299,104 | ✓<br>304,928 |
| `atan` | ✓<br>16,744 | ✓<br>97,400 | ✓<br>125,808 | ✓<br>142,904 | ✓<br>148,600 | ✓<br>127,016 | ✓<br>166,640 | ✓<br>306,776 | ✓<br>252,184 | ✓<br>285,656 | ✓<br>292,120 | ✓<br>297,648 |
| `atan2` | ✓<br>55,928 | ✓<br>157,504 | ✓<br>161,024 | ✓<br>161,992 | ✓<br>167,208 | ✓<br>129,024 | ✓<br>168,216 | ✓<br>324,128 | ✓<br>247,016 | ✓<br>286,816 | ✓<br>287,584 | ✓<br>287,968 |
| `atanh` | ✓<br>19,264 | ✓<br>102,456 | ✓<br>145,320 | ✓<br>150,432 | ✓<br>162,152 | ✓<br>132,656 | ✓<br>165,640 | ✓<br>298,360 | ✓<br>232,056 | ✓<br>265,304 | ✓<br>265,336 | ✓<br>265,336 |
| `cbrt` | ✓<br>34,368 | ✓<br>135,664 | ✓<br>145,752 | ✓<br>157,368 | ✓<br>162,536 | ✓<br>132,312 | ✓<br>179,096 | ✓<br>338,328 | ✓<br>274,728 | ✓<br>312,456 | ✓<br>320,008 | ✓<br>326,104 |
| `cos` | ✓<br>23,352 | ✓<br>106,904 | ✓<br>154,048 | ✓<br>159,952 | ✓<br>166,816 | ✓<br>140,248 | ✓<br>174,384 | ✓<br>314,464 | ✓<br>234,568 | ✓<br>271,256 | ✓<br>276,128 | ✓<br>279,504 |
| `cosh` | ✓<br>37,904 | ✓<br>136,696 | ✓<br>176,512 | ✓<br>182,936 | ✓<br>191,784 | ✓<br>158,152 | ✓<br>199,240 | ✓<br>362,592 | ✓<br>272,016 | ✓<br>315,296 | ✓<br>319,064 | ✓<br>321,424 |
| `div` | ✓<br>328 | ✓<br>1,128 | ✓<br>1,104 | ✓<br>1,344 | ✓<br>1,400 | ✓<br>1,248 | ✓<br>1,808 | ✓<br>3,352 | ✓<br>2,928 | ✓<br>3,312 | ✓<br>3,504 | ✓<br>3,680 |
| `exp` | ✓<br>37,096 | ✓<br>127,424 | ✓<br>160,640 | ✓<br>166,368 | ✓<br>172,840 | ✓<br>142,776 | ✓<br>180,048 | ✓<br>328,104 | ✓<br>244,984 | ✓<br>284,624 | ✓<br>288,128 | ✓<br>290,608 |
| `exp2` | ✓<br>42,880 | ✓<br>136,056 | ✓<br>163,808 | ✓<br>170,264 | ✓<br>174,544 | ✓<br>144,600 | ✓<br>182,512 | ✓<br>333,400 | ✓<br>249,240 | ✓<br>289,112 | ✓<br>292,576 | ✓<br>294,936 |
| `expm1` | ✓<br>1,984 | ✓<br>5,880 | ✓<br>6,120 | ✓<br>7,080 | ✓<br>7,520 | ✓<br>7,072 | ✓<br>9,536 | ✓<br>18,496 | ✓<br>16,176 | ✓<br>20,912 | ✓<br>22,880 | ✓<br>24,368 |
| `hypot` | ✓<br>1,920 | ✓<br>5,176 | ✓<br>3,944 | ✓<br>4,400 | ✓<br>3,984 | ✓<br>3,456 | ✓<br>4,520 | ✓<br>8,056 | ✓<br>6,608 | ✓<br>7,448 | ✓<br>7,696 | ✓<br>7,912 |
| `ln` | ✓<br>41,576 | ✓<br>154,776 | ✓<br>179,392 | ✓<br>172,976 | ✓<br>181,152 | ✓<br>140,992 | ✓<br>181,400 | ✓<br>342,984 | ✓<br>262,576 | ✓<br>303,448 | ✓<br>303,784 | ✓<br>304,000 |
| `log` | ✓<br>22,664 | ✓<br>69,408 | ✓<br>66,368 | ✓<br>68,392 | ✓<br>66,384 | ✓<br>52,960 | ✓<br>68,152 | ✓<br>127,912 | ✓<br>99,448 | ✓<br>115,912 | ✓<br>117,416 | ✓<br>118,632 |
| `log10` | ✓<br>42,344 | ✓<br>159,424 | ✓<br>185,744 | ✓<br>179,592 | ✓<br>188,624 | ✓<br>147,048 | ✓<br>189,112 | ✓<br>356,576 | ✓<br>273,392 | ✓<br>315,344 | ✓<br>315,624 | ✓<br>315,880 |
| `log1p` | ✓<br>2,232 | ✓<br>6,640 | ✓<br>6,472 | ✓<br>7,312 | ✓<br>7,752 | ✓<br>7,232 | ✓<br>9,592 | ✓<br>18,432 | ✓<br>16,120 | ✓<br>20,568 | ✓<br>21,984 | ✓<br>22,912 |
| `log2` | ✓<br>43,216 | ✓<br>157,624 | ✓<br>179,120 | ✓<br>172,624 | ✓<br>179,832 | ✓<br>139,944 | ✓<br>180,184 | ✓<br>340,592 | ✓<br>260,760 | ✓<br>300,984 | ✓<br>301,224 | ✓<br>301,376 |
| `mul` | ✓<br>552 | ✓<br>2,336 | ✓<br>2,592 | ✓<br>2,936 | ✓<br>3,208 | ✓<br>2,872 | ✓<br>3,832 | ✓<br>7,192 | ✓<br>6,192 | ✓<br>7,320 | ✓<br>7,672 | ✓<br>7,832 |
| `powf` | ✓<br>43,248 | ✓<br>113,128 | ✓<br>134,992 | ✓<br>144,032 | ✓<br>140,056 | ✓<br>123,296 | ✓<br>158,928 | ✓<br>291,144 | ✓<br>202,912 | ✓<br>236,320 | ✓<br>240,296 | ✓<br>247,232 |
| `rem` | ✓<br>280 | ✓<br>1,040 | ✓<br>1,048 | ✓<br>1,288 | ✓<br>1,352 | ✓<br>1,208 | ✓<br>1,760 | ✓<br>3,272 | ✓<br>2,864 | ✓<br>3,240 | ✓<br>3,432 | ✓<br>3,608 |
| `sin` | ✓<br>23,392 | ✓<br>107,048 | ✓<br>154,624 | ✓<br>159,984 | ✓<br>167,200 | ✓<br>140,328 | ✓<br>174,664 | ✓<br>314,864 | ✓<br>234,832 | ✓<br>271,576 | ✓<br>276,712 | ✓<br>279,960 |
| `sinh` | ✓<br>37,312 | ✓<br>130,632 | ✓<br>166,912 | ✓<br>173,160 | ✓<br>180,336 | ✓<br>149,176 | ✓<br>188,072 | ✓<br>342,240 | ✓<br>256,096 | ✓<br>297,208 | ✓<br>301,192 | ✓<br>303,464 |
| `sqrt` | ✓<br>36,520 | ✓<br>138,736 | ✓<br>145,648 | ✓<br>157,208 | ✓<br>162,200 | ✓<br>131,352 | ✓<br>177,576 | ✓<br>335,432 | ✓<br>272,192 | ✓<br>309,512 | ✓<br>316,440 | ✓<br>322,264 |
| `sub` | ✓<br>496 | ✓<br>1,728 | ✓<br>1,728 | ✓<br>1,952 | ✓<br>1,928 | ✓<br>1,688 | ✓<br>2,344 | ✓<br>4,360 | ✓<br>3,688 | ✓<br>4,192 | ✓<br>4,416 | ✓<br>4,600 |
| `tan` | ✓<br>23,032 | ✓<br>105,976 | ✓<br>152,792 | ✓<br>161,184 | ✓<br>167,320 | ✓<br>141,504 | ✓<br>174,904 | ✓<br>316,256 | ✓<br>236,120 | ✓<br>272,896 | ✓<br>277,688 | ✓<br>281,000 |
| `tanh` | ✓<br>22,944 | ✓<br>111,376 | ✓<br>158,048 | ✓<br>162,040 | ✓<br>174,992 | ✓<br>142,512 | ✓<br>178,304 | ✓<br>321,776 | ✓<br>244,128 | ✓<br>283,632 | ✓<br>286,912 | ✓<br>287,344 |
<!-- END GENERATED:precision:surface -->
