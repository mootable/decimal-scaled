# Precision

**`decimal-scaled` is perfectly precise** — correctly rounded to within 0.5 ULP,
i.e. **0 bits of error**.

<!-- BEGIN GENERATED:precision:stats -->
We execute 102,729 specialised inputs across all 30 functions, on 117 widths and scales, under all 6 rounding modes, resulting in 72,115,758 separate checks.
<!-- END GENERATED:precision:stats -->

Each golden case was gathered from the codebase failing, from theory, from
destructive testing of our own code, and from destructive testing of other
libraries — collated into a behemoth of meanness.

The data on this page is generated and committed directly from the
[golden-comprehensive CI job](https://github.com/mootable/decimal-scaled/actions/workflows/golden-comprehensive.yml)
and auto-rendered here; there is **no manual editing**. If we ever fail our
guarantee, we want you to know.

Each row is a function, each column a storage width. A `✓` means `0` bits of
error — the exact correctly-rounded value — across every scale and all six
rounding modes at that width, with the number of checks verified correct beneath
it. A `✗` marks a cell with at least one failure, the count of failing checks
beneath. Inputs outside a function's domain or a tier's range aren't checks, so
they count toward neither. The full per-scale, per-mode surface lives in
[`results/golden/`](https://github.com/mootable/decimal-scaled/tree/main/results/golden).

<!-- BEGIN GENERATED:precision:surface -->
| Function | D18 | D38 | D57 | D76 | D115 | D153 | D230 | D307 | D462 | D616 | D924 | D1232 |
| :-- | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| `acos` | ✓<br>14,412 | ✓<br>75,822 | ✓<br>106,752 | ✓<br>109,260 | ✓<br>118,626 | ✓<br>96,336 | ✓<br>120,288 | ✓<br>216,636 | ✓<br>168,474 | ✓<br>192,588 | ✓<br>192,588 | ✓<br>192,588 |
| `acosh` | ✓<br>34,938 | ✓<br>104,382 | ✓<br>109,968 | ✓<br>110,022 | ✓<br>113,874 | ✓<br>88,074 | ✓<br>113,886 | ✓<br>217,134 | ✓<br>165,510 | ✓<br>191,334 | ✓<br>191,334 | ✓<br>191,334 |
| `add` | ✓<br>372 | ✓<br>1,296 | ✓<br>1,296 | ✓<br>1,464 | ✓<br>1,446 | ✓<br>1,266 | ✓<br>1,758 | ✓<br>3,270 | ✓<br>2,766 | ✓<br>3,144 | ✓<br>3,312 | ✓<br>3,450 |
| `asin` | ✓<br>14,712 | ✓<br>79,692 | ✓<br>114,084 | ✓<br>118,026 | ✓<br>127,770 | ✓<br>104,502 | ✓<br>130,422 | ✓<br>234,948 | ✓<br>182,700 | ✓<br>208,872 | ✓<br>208,872 | ✓<br>208,872 |
| `asinh` | ✓<br>12,180 | ✓<br>74,166 | ✓<br>96,438 | ✓<br>109,158 | ✓<br>113,904 | ✓<br>97,170 | ✓<br>127,926 | ✓<br>235,140 | ✓<br>193,554 | ✓<br>219,258 | ✓<br>224,328 | ✓<br>228,696 |
| `atan` | ✓<br>12,558 | ✓<br>73,050 | ✓<br>94,356 | ✓<br>107,178 | ✓<br>111,450 | ✓<br>95,262 | ✓<br>124,980 | ✓<br>230,082 | ✓<br>189,138 | ✓<br>214,242 | ✓<br>219,090 | ✓<br>223,236 |
| `atan2` | ✓<br>41,946 | ✓<br>118,128 | ✓<br>120,768 | ✓<br>121,494 | ✓<br>125,406 | ✓<br>96,768 | ✓<br>126,162 | ✓<br>243,096 | ✓<br>185,262 | ✓<br>215,112 | ✓<br>215,688 | ✓<br>215,976 |
| `atanh` | ✓<br>14,448 | ✓<br>76,842 | ✓<br>108,990 | ✓<br>112,818 | ✓<br>121,602 | ✓<br>99,474 | ✓<br>124,212 | ✓<br>223,734 | ✓<br>174,006 | ✓<br>198,936 | ✓<br>198,960 | ✓<br>198,960 |
| `cbrt` | ✓<br>25,776 | ✓<br>101,748 | ✓<br>109,314 | ✓<br>118,026 | ✓<br>121,902 | ✓<br>99,234 | ✓<br>134,322 | ✓<br>253,746 | ✓<br>206,046 | ✓<br>234,342 | ✓<br>240,006 | ✓<br>244,578 |
| `cos` | ✓<br>17,514 | ✓<br>80,178 | ✓<br>115,536 | ✓<br>119,958 | ✓<br>125,100 | ✓<br>105,168 | ✓<br>130,770 | ✓<br>235,806 | ✓<br>175,890 | ✓<br>203,400 | ✓<br>207,054 | ✓<br>209,586 |
| `cosh` | ✓<br>28,428 | ✓<br>102,522 | ✓<br>132,384 | ✓<br>137,202 | ✓<br>143,838 | ✓<br>118,614 | ✓<br>149,430 | ✓<br>271,944 | ✓<br>204,012 | ✓<br>236,472 | ✓<br>239,298 | ✓<br>241,068 |
| `div` | ✓<br>246 | ✓<br>846 | ✓<br>828 | ✓<br>1,008 | ✓<br>1,050 | ✓<br>936 | ✓<br>1,356 | ✓<br>2,514 | ✓<br>2,196 | ✓<br>2,484 | ✓<br>2,628 | ✓<br>2,760 |
| `exp` | ✓<br>27,822 | ✓<br>95,568 | ✓<br>120,480 | ✓<br>124,770 | ✓<br>129,618 | ✓<br>107,064 | ✓<br>135,018 | ✓<br>246,024 | ✓<br>183,666 | ✓<br>213,318 | ✓<br>215,910 | ✓<br>217,734 |
| `exp2` | ✓<br>32,160 | ✓<br>102,042 | ✓<br>122,856 | ✓<br>127,698 | ✓<br>130,908 | ✓<br>108,450 | ✓<br>136,884 | ✓<br>250,050 | ✓<br>186,930 | ✓<br>216,834 | ✓<br>219,432 | ✓<br>221,202 |
| `hypot` | ✓<br>1,440 | ✓<br>3,882 | ✓<br>2,958 | ✓<br>3,300 | ✓<br>2,988 | ✓<br>2,592 | ✓<br>3,390 | ✓<br>6,042 | ✓<br>4,956 | ✓<br>5,586 | ✓<br>5,772 | ✓<br>5,934 |
| `ln` | ✓<br>31,182 | ✓<br>116,082 | ✓<br>134,544 | ✓<br>129,720 | ✓<br>135,840 | ✓<br>105,696 | ✓<br>135,960 | ✓<br>256,920 | ✓<br>196,536 | ✓<br>226,902 | ✓<br>226,998 | ✓<br>227,052 |
| `log` | ✓<br>14,586 | ✓<br>46,032 | ✓<br>44,790 | ✓<br>45,768 | ✓<br>44,280 | ✓<br>34,908 | ✓<br>44,778 | ✓<br>84,210 | ✓<br>64,494 | ✓<br>74,358 | ✓<br>74,352 | ✓<br>74,400 |
| `log10` | ✓<br>31,758 | ✓<br>119,568 | ✓<br>139,308 | ✓<br>134,694 | ✓<br>141,468 | ✓<br>110,286 | ✓<br>141,834 | ✓<br>267,432 | ✓<br>205,044 | ✓<br>236,508 | ✓<br>236,718 | ✓<br>236,910 |
| `log2` | ✓<br>32,412 | ✓<br>118,218 | ✓<br>134,340 | ✓<br>129,468 | ✓<br>134,874 | ✓<br>104,958 | ✓<br>135,138 | ✓<br>255,444 | ✓<br>195,570 | ✓<br>225,738 | ✓<br>225,918 | ✓<br>226,032 |
| `mul` | ✓<br>414 | ✓<br>1,752 | ✓<br>1,944 | ✓<br>2,202 | ✓<br>2,406 | ✓<br>2,154 | ✓<br>2,874 | ✓<br>5,394 | ✓<br>4,644 | ✓<br>5,490 | ✓<br>5,754 | ✓<br>5,874 |
| `powf` | ✓<br>32,436 | ✓<br>84,846 | ✓<br>101,244 | ✓<br>108,024 | ✓<br>105,042 | ✓<br>92,472 | ✓<br>119,196 | ✓<br>218,358 | ✓<br>152,184 | ✓<br>177,240 | ✓<br>180,222 | ✓<br>185,424 |
| `rem` | ✓<br>210 | ✓<br>780 | ✓<br>786 | ✓<br>966 | ✓<br>1,014 | ✓<br>906 | ✓<br>1,320 | ✓<br>2,454 | ✓<br>2,148 | ✓<br>2,430 | ✓<br>2,574 | ✓<br>2,706 |
| `sin` | ✓<br>17,544 | ✓<br>80,286 | ✓<br>115,968 | ✓<br>119,988 | ✓<br>125,400 | ✓<br>105,246 | ✓<br>130,998 | ✓<br>236,124 | ✓<br>176,100 | ✓<br>203,634 | ✓<br>207,474 | ✓<br>209,898 |
| `sinh` | ✓<br>27,984 | ✓<br>97,974 | ✓<br>125,184 | ✓<br>129,870 | ✓<br>135,252 | ✓<br>111,882 | ✓<br>141,054 | ✓<br>256,680 | ✓<br>192,072 | ✓<br>222,906 | ✓<br>225,894 | ✓<br>227,598 |
| `sqrt` | ✓<br>27,390 | ✓<br>104,052 | ✓<br>109,236 | ✓<br>117,906 | ✓<br>121,650 | ✓<br>98,514 | ✓<br>133,182 | ✓<br>251,574 | ✓<br>204,144 | ✓<br>232,134 | ✓<br>237,330 | ✓<br>241,698 |
| `sub` | ✓<br>372 | ✓<br>1,296 | ✓<br>1,296 | ✓<br>1,464 | ✓<br>1,446 | ✓<br>1,266 | ✓<br>1,758 | ✓<br>3,270 | ✓<br>2,766 | ✓<br>3,144 | ✓<br>3,312 | ✓<br>3,450 |
| `tan` | ✓<br>17,274 | ✓<br>79,482 | ✓<br>114,594 | ✓<br>120,888 | ✓<br>125,490 | ✓<br>106,128 | ✓<br>131,178 | ✓<br>237,192 | ✓<br>177,090 | ✓<br>204,672 | ✓<br>208,266 | ✓<br>210,750 |
| `tanh` | ✓<br>17,208 | ✓<br>83,532 | ✓<br>118,536 | ✓<br>121,524 | ✓<br>131,232 | ✓<br>106,866 | ✓<br>133,710 | ✓<br>241,296 | ✓<br>183,060 | ✓<br>212,682 | ✓<br>215,142 | ✓<br>215,466 |
<!-- END GENERATED:precision:surface -->
