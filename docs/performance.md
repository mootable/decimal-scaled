# Performance

<div class="bench-header" markdown>
<div markdown>

How fast each operation is, by storage width and scale.

The numbers on this page are **generated from CI**: the `bench-branch-compare`
run measures every `(operation, width, scale)` on a GitHub-hosted runner,
compares it against the previous release, and commits the medians to
`results/timing/`, which this page renders. They are refreshed on each release
PR.

> Absolute timings are machine-dependent — the *ratios* between operations and
> widths, measured in the same run, are what to read. Operands are `black_box`-ed
> so the optimiser can't fold the work away.

Times are the unit shown in each cell; the legend maps each unit to its size in
nanoseconds. Each function's graph plots median time (log scale) against storage
width: the two solid lines are scale `0` and the maximum scale, the dashed lines
the band-edge scales in between, and the shaded band is the spread between scale
`0` and the maximum.

<!-- BEGIN GENERATED:performance:units -->
| Unit | In nanoseconds |
| :-- | --: |
| ns | 10⁰ ns |
| µs | 10³ ns |
| ms | 10⁶ ns |
<!-- END GENERATED:performance:units -->

</div>

<!-- BEGIN GENERATED:performance:widths -->
| Width | Decimals | Integer | Bits |
| :-- | --: | :-- | --: |
| D18 | 18 | `Int<1>` | 64 |
| D38 | 38 | `Int<2>` | 128 |
| D57 | 57 | `Int<3>` | 192 |
| D76 | 76 | `Int<4>` | 256 |
| D115 | 115 | `Int<6>` | 384 |
| D153 | 153 | `Int<8>` | 512 |
| D230 | 230 | `Int<12>` | 768 |
| D307 | 307 | `Int<16>` | 1024 |
| D462 | 462 | `Int<24>` | 1536 |
| D616 | 616 | `Int<32>` | 2048 |
| D924 | 924 | `Int<48>` | 3072 |
| D1232 | 1232 | `Int<64>` | 4096 |
<!-- END GENERATED:performance:widths -->

</div>

<!-- BEGIN GENERATED:performance:body -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.17 ns | 21.8 µs | 13.5 µs | 30.3 µs | 39.4 µs |
| D38 | 3.95 µs | 7.79 µs | 6.72 µs | 11.6 µs | 16 µs |
| D57 | 4.68 µs | 8.29 µs | 11 µs | 16.3 µs | 19.3 µs |
| D76 | 5.17 µs | 9.62 µs | 14.3 µs | 16 µs | 22 µs |
| D115 | 5.35 µs | 12 µs | 24.3 µs | 33.2 µs | 41.5 µs |
| D153 | 5.19 µs | 14.1 µs | 26.9 µs | 40.8 µs | 63.6 µs |
| D230 | 4.23 µs | 24 µs | 43.9 µs | 68.8 µs | 106 µs |
| D307 | 4.79 µs | 26.9 µs | 59.1 µs | 121 µs | 178 µs |
| D462 | 4.44 µs | 41.8 µs | 106 µs | 251 µs | 375 µs |
| D616 | 5.38 µs | 59.2 µs | 202 µs | 423 µs | 657 µs |
| D924 | 4.45 µs | 106 µs | 420 µs | 858 µs | 1.47 ms |
| D1232 | 4.88 µs | 213 µs | 610 µs | 1.63 ms | 3.15 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.6 88.2,107.2 124.4,105.1 160.5,103.9 196.7,103.5 232.9,103.9 269.1,106.4 305.3,104.8 341.5,105.8 377.6,103.4 413.8,105.8 450.0,104.6 450.0,24.3 413.8,33.8 377.6,43.8 341.5,50.8 305.3,60.0 269.1,66.5 232.9,72.8 196.7,78.1 160.5,85.9 124.4,87.6 88.2,89.9 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.6 88.2,107.2 124.4,105.1 160.5,103.9 196.7,103.5 232.9,103.9 269.1,106.4 305.3,104.8 341.5,105.8 377.6,103.4 413.8,105.8 450.0,104.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,98.8 124.4,98.0 160.5,96.2 196.7,93.4 232.9,91.4 269.1,84.8 305.3,83.5 341.5,78.0 377.6,73.6 413.8,66.4 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.0 88.2,100.6 124.4,94.5 160.5,91.3 196.7,84.7 232.9,83.4 269.1,77.4 305.3,73.7 341.5,66.4 377.6,58.4 413.8,49.3 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.9 124.4,89.7 160.5,89.9 196.7,80.8 232.9,78.3 269.1,71.8 305.3,64.8 341.5,55.7 377.6,49.3 413.8,40.5 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,89.9 124.4,87.6 160.5,85.9 196.7,78.1 232.9,72.8 269.1,66.5 305.3,60.0 341.5,50.8 377.6,43.8 413.8,33.8 450.0,24.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 25.7 µs | 29.2 µs | 17.1 µs | 36.4 µs | 38.4 µs |
| D38 | 12.1 µs | 35.5 µs | 21.3 µs | 53.3 µs | 66.1 µs |
| D57 | 3.31 µs | 4.31 µs | 4.98 µs | 6.65 µs | 8.19 µs |
| D76 | 3.69 µs | 5.11 µs | 5.69 µs | 7.07 µs | 10.1 µs |
| D115 | 6.69 µs | 9.39 µs | 12.2 µs | 18 µs | 21.4 µs |
| D153 | 6.62 µs | 9.81 µs | 14.7 µs | 21.6 µs | 33.3 µs |
| D230 | 7.28 µs | 16 µs | 28.4 µs | 43.1 µs | 58.7 µs |
| D307 | 12.4 µs | 26 µs | 48.1 µs | 84.3 µs | 131 µs |
| D462 | 10.8 µs | 37.1 µs | 71.2 µs | 168 µs | 261 µs |
| D616 | 22.5 µs | 71.1 µs | 171 µs | 316 µs | 520 µs |
| D924 | 26.6 µs | 130 µs | 379 µs | 750 µs | 1.26 ms |
| D1232 | 35.1 µs | 271 µs | 610 µs | 1.53 ms | 2.8 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,139.5 88.2,155.8 124.4,184.0 160.5,181.6 196.7,168.7 232.9,168.9 269.1,166.9 305.3,155.3 341.5,158.2 377.6,142.4 413.8,138.8 450.0,132.7 450.0,37.6 413.8,54.9 377.6,74.2 341.5,89.2 305.3,104.1 269.1,121.6 232.9,133.9 196.7,143.4 160.5,159.7 124.4,164.3 88.2,119.0 52.0,130.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,139.5 88.2,155.8 124.4,184.0 160.5,181.6 196.7,168.7 232.9,168.9 269.1,166.9 305.3,155.3 341.5,158.2 377.6,142.4 413.8,138.8 450.0,132.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,136.8 88.2,132.5 124.4,178.3 160.5,174.6 196.7,161.4 232.9,160.4 269.1,149.7 305.3,139.2 341.5,131.5 377.6,117.4 413.8,104.3 450.0,88.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,148.4 88.2,143.6 124.4,175.2 160.5,172.2 196.7,155.7 232.9,151.6 269.1,137.3 305.3,125.9 341.5,117.4 377.6,98.4 413.8,81.1 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,123.6 124.4,168.9 160.5,167.5 196.7,147.2 232.9,143.3 269.1,128.3 305.3,113.7 341.5,98.7 377.6,85.0 413.8,66.2 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.8 88.2,119.0 124.4,164.3 160.5,159.7 196.7,143.4 232.9,133.9 269.1,121.6 305.3,104.1 341.5,89.2 377.6,74.2 413.8,54.9 450.0,37.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.936 ns | 0.937 ns | 1.15 ns | 0.935 ns | 1.06 ns |
| D38 | 1.44 ns | 1.83 ns | 1.44 ns | 1.61 ns | 1.83 ns |
| D57 | 2.25 ns | 2.25 ns | 3.15 ns | 2.5 ns | 2.49 ns |
| D76 | 3.48 ns | 3.5 ns | 3.08 ns | 2.32 ns | 3.49 ns |
| D115 | 5.01 ns | 4.99 ns | 4.39 ns | 4.99 ns | 4.39 ns |
| D153 | 6.64 ns | 5.94 ns | 5.96 ns | 5.93 ns | 6.63 ns |
| D230 | 11.9 ns | 13.8 ns | 15.3 ns | 13.9 ns | 12 ns |
| D307 | 18.6 ns | 18.6 ns | 18.5 ns | 18.6 ns | 18.7 ns |
| D462 | 23 ns | 31 ns | 27.3 ns | 32.7 ns | 29.9 ns |
| D616 | 68.4 ns | 45.3 ns | 48.9 ns | 49.7 ns | 45.3 ns |
| D924 | 62.5 ns | 71.7 ns | 75 ns | 74.7 ns | 62.7 ns |
| D1232 | 70.6 ns | 107 ns | 71.2 ns | 107 ns | 70.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,152.1 124.4,142.3 160.5,133.0 196.7,125.0 232.9,118.9 269.1,106.2 305.3,96.6 341.5,91.9 377.6,68.2 413.8,70.2 450.0,67.6 450.0,67.7 413.8,70.1 377.6,77.2 341.5,86.2 305.3,96.5 269.1,106.0 232.9,118.9 196.7,127.9 160.5,132.8 124.4,140.2 88.2,146.8 52.0,158.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,152.1 124.4,142.3 160.5,133.0 196.7,125.0 232.9,118.9 269.1,106.2 305.3,96.6 341.5,91.9 377.6,68.2 413.8,70.2 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,146.8 124.4,142.4 160.5,132.8 196.7,125.1 232.9,121.3 269.1,102.9 305.3,96.5 341.5,85.4 377.6,77.2 413.8,67.2 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.9 88.2,152.1 124.4,135.1 160.5,135.6 196.7,127.9 232.9,121.2 269.1,100.7 305.3,96.6 341.5,88.2 377.6,75.5 413.8,66.3 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.6 124.4,140.1 160.5,141.8 196.7,125.1 232.9,121.3 269.1,102.8 305.3,96.5 341.5,84.3 377.6,75.2 413.8,66.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,146.8 124.4,140.2 160.5,132.8 196.7,127.9 232.9,118.9 269.1,106.0 305.3,96.5 341.5,86.2 377.6,77.2 413.8,70.1 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 21.8 µs | 13.5 µs | 30.3 µs | 39.4 µs |
| D38 | 3.92 µs | 7.76 µs | 6.69 µs | 11.5 µs | 16 µs |
| D57 | 4.63 µs | 8.37 µs | 10.9 µs | 16.2 µs | 19.2 µs |
| D76 | 5.19 µs | 9.64 µs | 14.3 µs | 16 µs | 22 µs |
| D115 | 5.14 µs | 12 µs | 23.6 µs | 34.8 µs | 40.7 µs |
| D153 | 5.13 µs | 14.2 µs | 26.6 µs | 40.7 µs | 64.2 µs |
| D230 | 4.17 µs | 24.1 µs | 43.9 µs | 68 µs | 106 µs |
| D307 | 4.69 µs | 26.6 µs | 59.6 µs | 121 µs | 178 µs |
| D462 | 4.56 µs | 40.5 µs | 106 µs | 248 µs | 373 µs |
| D616 | 5.33 µs | 59.3 µs | 202 µs | 422 µs | 656 µs |
| D924 | 4.41 µs | 108 µs | 420 µs | 858 µs | 1.47 ms |
| D1232 | 4.6 µs | 213 µs | 610 µs | 1.64 ms | 3.15 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,107.3 124.4,105.3 160.5,103.8 196.7,104.0 232.9,104.0 269.1,106.6 305.3,105.1 341.5,105.5 377.6,103.5 413.8,105.9 450.0,105.4 450.0,24.3 413.8,33.8 377.6,43.8 341.5,50.8 305.3,60.0 269.1,66.5 232.9,72.6 196.7,78.3 160.5,85.9 124.4,87.6 88.2,89.9 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,107.3 124.4,105.3 160.5,103.8 196.7,104.0 232.9,104.0 269.1,106.6 305.3,105.1 341.5,105.5 377.6,103.5 413.8,105.9 450.0,105.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,98.9 124.4,97.9 160.5,96.2 196.7,93.5 232.9,91.4 269.1,84.8 305.3,83.6 341.5,78.4 377.6,73.6 413.8,66.2 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.0 88.2,100.7 124.4,94.6 160.5,91.3 196.7,85.1 232.9,83.6 269.1,77.4 305.3,73.6 341.5,66.4 377.6,58.4 413.8,49.3 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.9 124.4,89.7 160.5,89.9 196.7,80.2 232.9,78.3 269.1,71.9 305.3,64.8 341.5,55.9 377.6,49.3 413.8,40.5 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,89.9 124.4,87.6 160.5,85.9 196.7,78.3 232.9,72.6 269.1,66.5 305.3,60.0 341.5,50.8 377.6,43.8 413.8,33.8 450.0,24.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 21.5 µs | 12.9 µs | 29.7 µs | 38.7 µs |
| D38 | 3.15 ns | 29 µs | 16.2 µs | 39.9 µs | 55.2 µs |
| D57 | 1.95 ns | 5.37 µs | 7.32 µs | 9.71 µs | 12.8 µs |
| D76 | 2.22 ns | 6.93 µs | 8.74 µs | 10.7 µs | 15.2 µs |
| D115 | 12.5 ns | 13.8 µs | 18.9 µs | 27 µs | 32.8 µs |
| D153 | 15.9 ns | 15.2 µs | 22.2 µs | 33.6 µs | 47.5 µs |
| D230 | 24.9 ns | 24.7 µs | 44.8 µs | 62.2 µs | 82.9 µs |
| D307 | 44.7 ns | 36.8 µs | 66.3 µs | 112 µs | 164 µs |
| D462 | 62.8 ns | 54.5 µs | 98.1 µs | 209 µs | 316 µs |
| D616 | 88.6 ns | 107 µs | 228 µs | 402 µs | 621 µs |
| D924 | 75.8 ns | 199 µs | 499 µs | 870 µs | 1.35 ms |
| D1232 | 113 ns | 381 µs | 768 µs | 1.76 ms | 2.65 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,195.8 124.4,201.7 160.5,200.1 196.7,178.7 232.9,175.7 269.1,170.1 305.3,162.8 341.5,158.6 377.6,154.4 413.8,156.3 450.0,151.4 450.0,26.5 413.8,34.9 377.6,44.5 341.5,52.9 305.3,61.0 269.1,69.5 232.9,76.4 196.7,81.0 160.5,90.5 124.4,92.7 88.2,74.5 52.0,78.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,195.8 124.4,201.7 160.5,200.1 196.7,178.7 232.9,175.7 269.1,170.1 305.3,162.8 341.5,158.6 377.6,154.4 413.8,156.3 450.0,151.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.2 88.2,82.5 124.4,103.4 160.5,100.3 196.7,91.7 232.9,90.5 269.1,84.5 305.3,79.6 341.5,74.7 377.6,66.3 413.8,58.6 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.5 88.2,89.8 124.4,99.6 160.5,97.4 196.7,87.8 232.9,85.8 269.1,77.1 305.3,72.2 341.5,67.4 377.6,56.9 413.8,47.2 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,78.5 124.4,96.1 160.5,94.9 196.7,83.4 232.9,80.7 269.1,73.0 305.3,65.7 341.5,58.0 377.6,49.9 413.8,40.3 450.0,31.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.9 88.2,74.5 124.4,92.7 160.5,90.5 196.7,81.0 232.9,76.4 269.1,69.5 305.3,61.0 341.5,52.9 377.6,44.5 413.8,34.9 450.0,26.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.58 µs | 2.41 µs | 2.82 µs | 3.2 µs |
| D38 | 4.79 µs | 8.97 µs | 5.55 µs | 9.86 µs | 14 µs |
| D57 | 3.85 µs | 7.24 µs | 9.49 µs | 14.2 µs | 5.45 µs |
| D76 | 4.27 µs | 8.33 µs | 12.4 µs | 14.2 µs | 19.6 µs |
| D115 | 4.25 µs | 10.3 µs | 21 µs | 30.2 µs | 37.2 µs |
| D153 | 4.28 µs | 12.4 µs | 21.2 µs | 37 µs | 59.5 µs |
| D230 | 3.49 µs | 21.1 µs | 41.6 µs | 63.1 µs | 96.9 µs |
| D307 | 3.9 µs | 24 µs | 48.8 µs | 113 µs | 166 µs |
| D462 | 2.65 µs | 32.7 µs | 89.3 µs | 218 µs | 320 µs |
| D616 | 4.42 µs | 54.5 µs | 189 µs | 393 µs | 621 µs |
| D924 | 3.69 µs | 97.8 µs | 390 µs | 813 µs | 1.39 ms |
| D1232 | 3.81 µs | 197 µs | 575 µs | 1.57 ms | 3.03 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,104.9 124.4,107.5 160.5,106.3 196.7,106.3 232.9,106.3 269.1,108.8 305.3,107.4 341.5,112.2 377.6,105.8 413.8,108.1 450.0,107.7 450.0,24.8 413.8,34.5 377.6,44.5 341.5,52.7 305.3,60.9 269.1,67.5 232.9,73.6 196.7,79.4 160.5,87.4 124.4,103.2 88.2,91.5 52.0,109.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,104.9 124.4,107.5 160.5,106.3 196.7,106.3 232.9,106.3 269.1,108.8 305.3,107.4 341.5,112.2 377.6,105.8 413.8,108.1 450.0,107.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.6 88.2,97.1 124.4,99.7 160.5,98.0 196.7,95.3 232.9,93.1 269.1,86.5 305.3,84.8 341.5,81.0 377.6,74.7 413.8,67.4 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.4 88.2,103.0 124.4,96.4 160.5,93.0 196.7,86.5 232.9,86.4 269.1,78.0 305.3,76.0 341.5,68.5 377.6,59.2 413.8,50.3 450.0,45.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,95.9 124.4,91.3 160.5,91.4 196.7,82.0 232.9,79.5 269.1,72.9 305.3,65.6 341.5,57.5 377.6,50.2 413.8,41.1 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,91.5 124.4,103.2 160.5,87.4 196.7,79.4 232.9,73.6 269.1,67.5 305.3,60.9 341.5,52.7 377.6,44.5 413.8,34.5 450.0,24.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.35 µs | 5.18 µs | 9.69 µs | 11.3 µs |
| D38 | 2.59 ns | 9.4 µs | 6.46 µs | 13 µs | 16.1 µs |
| D57 | 504 ns | 5.45 µs | 6.94 µs | 9.08 µs | 11.9 µs |
| D76 | 614 ns | 6.54 µs | 7.99 µs | 10.3 µs | 15 µs |
| D115 | 1.23 µs | 12.8 µs | 17.9 µs | 27.2 µs | 33.6 µs |
| D153 | 1.22 µs | 13.5 µs | 22.1 µs | 33.4 µs | 54.2 µs |
| D230 | 1.24 µs | 23.5 µs | 44.2 µs | 71.9 µs | 100 µs |
| D307 | 2.18 µs | 39.8 µs | 77.8 µs | 143 µs | 234 µs |
| D462 | 1.9 µs | 57.6 µs | 122 µs | 301 µs | 472 µs |
| D616 | 4.08 µs | 114 µs | 296 µs | 569 µs | 950 µs |
| D924 | 4.38 µs | 214 µs | 674 µs | 1.37 ms | 2.36 ms |
| D1232 | 6.24 µs | 459 µs | 1.1 ms | 2.85 ms | 5.29 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,198.2 124.4,132.8 160.5,130.3 196.7,121.8 232.9,121.8 269.1,121.6 305.3,114.6 341.5,116.3 377.6,106.8 413.8,106.0 450.0,101.6 450.0,17.9 413.8,27.9 377.6,39.2 341.5,47.9 305.3,56.6 269.1,67.1 232.9,74.8 196.7,80.7 160.5,90.7 124.4,93.6 88.2,89.8 52.0,94.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,198.2 124.4,132.8 160.5,130.3 196.7,121.8 232.9,121.8 269.1,121.6 305.3,114.6 341.5,116.3 377.6,106.8 413.8,106.0 450.0,101.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,96.5 124.4,103.2 160.5,101.0 196.7,92.7 232.9,92.0 269.1,85.1 305.3,78.6 341.5,74.0 377.6,65.5 413.8,57.7 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.9 88.2,101.1 124.4,100.2 160.5,98.5 196.7,88.5 232.9,85.9 269.1,77.3 305.3,70.2 341.5,64.7 377.6,53.7 413.8,43.5 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.1 88.2,92.5 124.4,96.9 160.5,95.4 196.7,83.3 232.9,80.8 269.1,71.2 305.3,62.7 341.5,53.5 377.6,45.6 413.8,34.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.2 88.2,89.8 124.4,93.6 160.5,90.7 196.7,80.7 232.9,74.8 269.1,67.1 305.3,56.6 341.5,47.9 377.6,39.2 413.8,27.9 450.0,17.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.07 µs | 5.14 µs | 2.46 µs | 7.7 µs | 8.7 µs |
| D38 | 2.33 µs | 5.79 µs | 3.63 µs | 9.67 µs | 10.9 µs |
| D57 | 329 ns | 608 ns | 717 ns | 1.04 µs | 1.06 µs |
| D76 | 533 ns | 762 ns | 1.19 µs | 1.12 µs | 1.95 µs |
| D115 | 344 ns | 2.1 µs | 2.43 µs | 3.52 µs | 5.19 µs |
| D153 | 366 ns | 2.64 µs | 3.62 µs | 5.76 µs | 6.87 µs |
| D230 | 377 ns | 4.36 µs | 8.36 µs | 10.2 µs | 12.1 µs |
| D307 | 565 ns | 6.1 µs | 11.3 µs | 16.9 µs | 21.1 µs |
| D462 | 365 ns | 9.68 µs | 21.1 µs | 36.2 µs | 47.7 µs |
| D616 | 539 ns | 14.7 µs | 37.3 µs | 57.8 µs | 77.5 µs |
| D924 | 404 ns | 24.1 µs | 82.2 µs | 121 µs | 161 µs |
| D1232 | 562 ns | 48.8 µs | 124 µs | 250 µs | 280 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,124.7 88.2,141.6 124.4,184.1 160.5,173.7 196.7,183.2 232.9,181.8 269.1,181.2 305.3,172.4 341.5,181.9 377.6,173.4 413.8,179.7 450.0,172.5 450.0,37.6 413.8,49.6 377.6,65.5 341.5,76.1 305.3,93.8 269.1,105.9 232.9,118.2 196.7,124.2 160.5,145.5 124.4,158.8 88.2,108.1 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,124.7 88.2,141.6 124.4,184.1 160.5,173.7 196.7,183.2 232.9,181.8 269.1,181.2 305.3,172.4 341.5,181.9 377.6,173.4 413.8,179.7 450.0,172.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.5 88.2,121.9 124.4,170.8 160.5,165.9 196.7,143.9 232.9,138.9 269.1,128.0 305.3,120.7 341.5,110.7 377.6,101.7 413.8,90.9 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,140.5 88.2,132.0 124.4,167.2 160.5,156.3 196.7,140.7 232.9,132.1 269.1,113.9 305.3,107.3 341.5,93.8 377.6,81.4 413.8,64.2 450.0,55.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,110.7 124.4,159.2 160.5,157.5 196.7,132.7 232.9,122.0 269.1,109.5 305.3,98.6 341.5,82.0 377.6,71.9 413.8,55.8 450.0,40.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,108.1 124.4,158.8 160.5,145.5 196.7,124.2 232.9,118.2 269.1,105.9 305.3,93.8 341.5,76.1 377.6,65.5 413.8,49.6 450.0,37.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 3.96 µs | 3.33 µs | 5.91 µs | 6.98 µs |
| D38 | 4.59 ns | 5.96 µs | 4.05 µs | 7.92 µs | 9.7 µs |
| D57 | 2.18 ns | 3.51 µs | 4.38 µs | 5.64 µs | 9.52 µs |
| D76 | 3.14 ns | 3.98 µs | 5.13 µs | 6.38 µs | 9.37 µs |
| D115 | 16.8 ns | 4.68 µs | 9.85 µs | 14.3 µs | 18.1 µs |
| D153 | 22.4 ns | 5.08 µs | 9.49 µs | 18.7 µs | 31.8 µs |
| D230 | 40.9 ns | 9.73 µs | 19.5 µs | 36.4 µs | 58.9 µs |
| D307 | 81.7 ns | 11.8 µs | 25.1 µs | 67.5 µs | 108 µs |
| D462 | 120 ns | 14.9 µs | 53.4 µs | 144 µs | 220 µs |
| D616 | 182 ns | 30.3 µs | 122 µs | 256 µs | 428 µs |
| D924 | 112 ns | 62.2 µs | 254 µs | 570 µs | 1.02 ms |
| D1232 | 258 ns | 126 µs | 402 µs | 1.12 ms | 2.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,191.1 124.4,200.3 160.5,195.8 196.7,175.0 232.9,171.4 269.1,163.9 305.3,155.4 341.5,150.5 377.6,145.4 413.8,151.4 450.0,141.1 450.0,28.4 413.8,38.3 377.6,49.1 341.5,57.4 305.3,66.1 269.1,73.7 232.9,81.4 196.7,88.4 160.5,96.5 124.4,96.3 88.2,96.1 52.0,100.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,191.1 124.4,200.3 160.5,195.8 196.7,175.0 232.9,171.4 269.1,163.9 305.3,155.4 341.5,150.5 377.6,145.4 413.8,151.4 450.0,141.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.2 88.2,102.1 124.4,108.7 160.5,107.2 196.7,105.1 232.9,104.1 269.1,96.1 305.3,93.6 341.5,90.7 377.6,82.0 413.8,73.0 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,106.9 124.4,106.0 160.5,104.0 196.7,95.9 232.9,96.4 269.1,87.4 305.3,84.3 341.5,74.9 377.6,64.7 413.8,55.6 450.0,49.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.2 88.2,98.6 124.4,102.8 160.5,101.3 196.7,91.3 232.9,88.0 269.1,79.7 305.3,72.0 341.5,62.6 377.6,55.5 413.8,45.5 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.2 88.2,96.1 124.4,96.3 160.5,96.5 196.7,88.4 232.9,81.4 269.1,73.7 305.3,66.1 341.5,57.4 377.6,49.1 413.8,38.3 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 7.34 µs | 5.45 µs | 10.2 µs | 12 µs |
| D38 | 2.67 ns | 10.2 µs | 6.62 µs | 13.4 µs | 16.5 µs |
| D57 | 2.49 ns | 5.24 µs | 6.97 µs | 8.38 µs | 10.9 µs |
| D76 | 3.17 ns | 6.04 µs | 7.41 µs | 9.14 µs | 12.8 µs |
| D115 | 10.9 ns | 12.5 µs | 11.4 µs | 22.7 µs | 25.3 µs |
| D153 | 21.6 ns | 7.46 µs | 15 µs | 22.3 µs | 39.6 µs |
| D230 | 40.8 ns | 13.5 µs | 24 µs | 42.8 µs | 67.4 µs |
| D307 | 82.4 ns | 16 µs | 48.7 µs | 78.7 µs | 117 µs |
| D462 | 120 ns | 22.8 µs | 68.3 µs | 164 µs | 230 µs |
| D616 | 184 ns | 36.3 µs | 133 µs | 269 µs | 413 µs |
| D924 | 123 ns | 69.5 µs | 267 µs | 554 µs | 895 µs |
| D1232 | 256 ns | 140 µs | 397 µs | 991 µs | 2.94 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.7 88.2,197.8 124.4,198.7 160.5,195.7 196.7,180.4 232.9,171.9 269.1,164.0 305.3,155.3 341.5,150.6 377.6,145.3 413.8,150.3 450.0,141.2 450.0,25.2 413.8,40.0 377.6,49.5 341.5,56.8 305.3,65.2 269.1,72.0 232.9,78.6 196.7,84.2 160.5,92.7 124.4,94.6 88.2,89.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.7 88.2,197.8 124.4,198.7 160.5,195.7 196.7,180.4 232.9,171.9 269.1,164.0 305.3,155.3 341.5,150.6 377.6,145.3 413.8,150.3 450.0,141.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.6 88.2,95.5 124.4,103.7 160.5,102.0 196.7,92.9 232.9,99.3 269.1,92.0 305.3,89.9 341.5,85.5 377.6,79.7 413.8,71.7 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.2 88.2,100.8 124.4,100.2 160.5,99.4 196.7,94.1 232.9,90.7 269.1,84.8 305.3,76.1 341.5,71.9 377.6,63.6 413.8,54.9 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,92.1 124.4,97.9 160.5,96.8 196.7,85.5 232.9,85.7 269.1,77.7 305.3,70.1 341.5,61.0 377.6,54.9 413.8,45.9 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,89.5 124.4,94.6 160.5,92.7 196.7,84.2 232.9,78.6 269.1,72.0 305.3,65.2 341.5,56.8 377.6,49.5 413.8,40.0 450.0,25.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.97 ns | 5.91 ns | 6.45 ns | 8.53 ns | 9.33 ns |
| D38 | 11.1 ns | 12.4 ns | 15.1 ns | 727 ns | 1.07 µs |
| D57 | 38 ns | 49.1 ns | 64.5 ns | 112 ns | 107 ns |
| D76 | 40.8 ns | 65.9 ns | 76.2 ns | 92 ns | 133 ns |
| D115 | 57.4 ns | 85.8 ns | 105 ns | 186 ns | 210 ns |
| D153 | 68.2 ns | 109 ns | 143 ns | 222 ns | 319 ns |
| D230 | 81.6 ns | 153 ns | 250 ns | 364 ns | 464 ns |
| D307 | 130 ns | 220 ns | 357 ns | 561 ns | 847 ns |
| D462 | 188 ns | 416 ns | 564 ns | 1.1 µs | 1.34 µs |
| D616 | 270 ns | 599 ns | 993 ns | 1.78 µs | 2.2 µs |
| D924 | 278 ns | 948 ns | 2.04 µs | 2.59 µs | 4.54 µs |
| D1232 | 399 ns | 1.89 µs | 3.05 µs | 4.63 µs | 7.62 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.2 88.2,157.7 124.4,131.0 160.5,129.4 196.7,122.0 232.9,118.3 269.1,114.4 305.3,104.2 341.5,96.3 377.6,88.5 413.8,87.8 450.0,79.9 450.0,15.9 413.8,27.2 377.6,42.9 341.5,53.6 305.3,63.6 269.1,76.7 232.9,84.8 196.7,93.9 160.5,103.9 124.4,108.5 88.2,58.6 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.2 88.2,157.7 124.4,131.0 160.5,129.4 196.7,122.0 232.9,118.3 269.1,114.4 305.3,104.2 341.5,96.3 377.6,88.5 413.8,87.8 450.0,79.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,155.3 124.4,125.4 160.5,119.1 196.7,113.3 232.9,108.2 269.1,100.8 305.3,92.9 341.5,79.1 377.6,71.1 413.8,61.2 450.0,46.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.5 88.2,151.0 124.4,119.5 160.5,115.9 196.7,109.0 232.9,102.2 269.1,90.1 305.3,82.4 341.5,72.4 377.6,60.1 413.8,44.5 450.0,35.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.5 88.2,66.9 124.4,107.6 160.5,111.8 196.7,96.5 232.9,92.7 269.1,81.9 305.3,72.6 341.5,57.9 377.6,47.5 413.8,39.3 450.0,26.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,58.6 124.4,108.5 160.5,103.9 196.7,93.9 232.9,84.8 269.1,76.7 305.3,63.6 341.5,53.6 377.6,42.9 413.8,27.2 450.0,15.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 4.41 µs | 4.03 µs | 6.74 µs | 7.88 µs |
| D38 | 1.73 ns | 6.77 µs | 4.84 µs | 8.73 µs | 10.7 µs |
| D57 | 2.49 ns | 3.53 µs | 4.14 µs | 5.95 µs | 9.67 µs |
| D76 | 3.2 ns | 6.03 µs | 6.73 µs | 8.34 µs | 11.8 µs |
| D115 | 17 ns | 6.81 µs | 12.5 µs | 18.6 µs | 21.3 µs |
| D153 | 21.7 ns | 6.86 µs | 14.8 µs | 22.2 µs | 37.4 µs |
| D230 | 44.7 ns | 12.8 µs | 23 µs | 41.1 µs | 65.7 µs |
| D307 | 84.3 ns | 14.7 µs | 34.1 µs | 77.3 µs | 115 µs |
| D462 | 126 ns | 21.7 µs | 66.5 µs | 162 µs | 227 µs |
| D616 | 180 ns | 34.5 µs | 131 µs | 265 µs | 409 µs |
| D924 | 128 ns | 66.9 µs | 263 µs | 550 µs | 887 µs |
| D1232 | 262 ns | 139 µs | 393 µs | 984 µs | 2.93 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,203.2 124.4,198.7 160.5,195.6 196.7,174.8 232.9,171.8 269.1,162.9 305.3,155.0 341.5,150.0 377.6,145.6 413.8,149.8 450.0,140.9 450.0,25.2 413.8,40.1 377.6,49.7 341.5,57.0 305.3,65.4 269.1,72.4 232.9,79.3 196.7,86.3 160.5,93.7 124.4,96.1 88.2,94.8 52.0,98.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,203.2 124.4,198.7 160.5,195.6 196.7,174.8 232.9,171.8 269.1,162.9 305.3,155.0 341.5,150.0 377.6,145.6 413.8,149.8 450.0,140.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.9 88.2,100.6 124.4,108.6 160.5,102.0 196.7,100.5 232.9,100.4 269.1,92.6 305.3,90.9 341.5,86.1 377.6,80.4 413.8,72.1 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,104.7 124.4,106.7 160.5,100.6 196.7,92.9 232.9,90.9 269.1,85.4 305.3,80.5 341.5,72.2 377.6,63.8 413.8,55.2 450.0,50.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.6 88.2,97.4 124.4,102.2 160.5,98.0 196.7,88.0 232.9,85.8 269.1,78.2 305.3,70.3 341.5,61.2 377.6,55.1 413.8,46.0 450.0,38.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.7 88.2,94.8 124.4,96.1 160.5,93.7 196.7,86.3 232.9,79.3 269.1,72.4 305.3,65.4 341.5,57.0 377.6,49.7 413.8,40.1 450.0,25.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 40.2 ns | 40.2 ns | 40.2 ns | 40.1 ns |
| D38 | 15.1 ns | 42.9 ns | 70.1 ns | 67.2 ns | 108 ns |
| D57 | 16.7 ns | 40 ns | 67.4 ns | 609 ns | 700 ns |
| D76 | 17.3 ns | 73.1 ns | 690 ns | 486 ns | 961 ns |
| D115 | 20.5 ns | 79.3 ns | 701 ns | 1.09 µs | 1.39 µs |
| D153 | 23 ns | 698 ns | 1.05 µs | 1.4 µs | 1.98 µs |
| D230 | 21.9 ns | 729 ns | 1.39 µs | 2.35 µs | 2.48 µs |
| D307 | 42.2 ns | 1.09 µs | 2.14 µs | 3.32 µs | 5.51 µs |
| D462 | 49.1 ns | 1.51 µs | 2.86 µs | 6.4 µs | 9.6 µs |
| D616 | 84.1 ns | 2.59 µs | 5.99 µs | 10.8 µs | 16 µs |
| D924 | 83.8 ns | 2.89 µs | 11.1 µs | 23.2 µs | 27.1 µs |
| D1232 | 92.5 ns | 6.31 µs | 18.1 µs | 27.2 µs | 48.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,201.1 124.4,198.8 160.5,198.1 196.7,194.4 232.9,191.9 269.1,193.0 305.3,178.7 341.5,175.4 377.6,163.8 413.8,163.8 450.0,161.7 450.0,25.9 413.8,38.3 377.6,49.9 341.5,60.9 305.3,72.9 269.1,90.3 232.9,95.2 196.7,102.9 160.5,110.9 124.4,117.7 88.2,158.4 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,201.1 124.4,198.8 160.5,198.1 196.7,194.4 232.9,191.9 269.1,193.0 305.3,178.7 341.5,175.4 377.6,163.8 413.8,163.8 450.0,161.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.8 88.2,178.4 124.4,179.9 160.5,166.8 196.7,165.0 232.9,117.8 269.1,116.9 305.3,108.2 341.5,101.1 377.6,89.3 413.8,86.9 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,167.7 124.4,168.6 160.5,118.0 196.7,117.7 232.9,108.9 269.1,102.9 305.3,93.5 341.5,87.2 377.6,71.1 413.8,57.8 450.0,47.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,168.6 124.4,120.8 160.5,125.7 196.7,108.1 232.9,102.7 269.1,91.4 305.3,83.9 341.5,69.7 377.6,58.4 413.8,41.7 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,158.4 124.4,117.7 160.5,110.9 196.7,102.9 232.9,95.2 269.1,90.3 305.3,72.9 341.5,60.9 377.6,49.9 413.8,38.3 450.0,25.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 278 ns | 351 ns | 370 ns | 401 ns |
| D38 | 1.73 ns | 390 ns | 356 ns | 368 ns | 407 ns |
| D57 | 260 ns | 438 ns | 438 ns | 479 ns | 633 ns |
| D76 | 279 ns | 487 ns | 460 ns | 495 ns | 654 ns |
| D115 | 284 ns | 482 ns | 565 ns | 1.05 µs | 932 ns |
| D153 | 329 ns | 493 ns | 590 ns | 978 ns | 1.34 µs |
| D230 | 437 ns | 611 ns | 1.11 µs | 1.2 µs | 1.54 µs |
| D307 | 638 ns | 648 ns | 1.02 µs | 1.23 µs | 9.6 µs |
| D462 | 980 ns | 2.74 µs | 3.01 µs | 4.15 µs | 4.81 µs |
| D616 | 1.52 µs | 1.39 µs | 1.73 µs | 2.64 µs | 3.4 µs |
| D924 | 1.5 µs | 1.72 µs | 2.73 µs | 3.34 µs | 3.64 µs |
| D1232 | 2.37 µs | 3.03 µs | 3.13 µs | 4.87 µs | 5.41 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.0 88.2,198.1 124.4,89.2 160.5,87.7 196.7,87.3 232.9,84.2 269.1,78.0 305.3,69.7 341.5,60.4 377.6,51.0 413.8,51.2 450.0,41.3 450.0,23.3 413.8,31.9 377.6,33.4 341.5,25.9 305.3,10.9 269.1,50.6 232.9,53.6 196.7,61.5 160.5,69.2 124.4,69.9 88.2,79.5 52.0,79.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.0 88.2,198.1 124.4,89.2 160.5,87.7 196.7,87.3 232.9,84.2 269.1,78.0 305.3,69.7 341.5,60.4 377.6,51.0 413.8,51.2 450.0,41.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,87.8 88.2,80.4 124.4,77.9 160.5,75.6 196.7,75.8 232.9,75.4 269.1,70.7 305.3,69.4 341.5,38.1 377.6,52.8 413.8,48.3 450.0,35.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.7 88.2,82.4 124.4,77.9 160.5,76.8 196.7,72.4 232.9,71.5 269.1,57.7 305.3,59.6 341.5,36.1 377.6,48.1 413.8,38.2 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.6 88.2,81.7 124.4,76.0 160.5,75.3 196.7,59.0 232.9,60.5 269.1,56.1 305.3,55.5 341.5,29.1 377.6,38.9 413.8,33.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.9 88.2,79.5 124.4,69.9 160.5,69.2 196.7,61.5 232.9,53.6 269.1,50.6 305.3,10.9 341.5,25.9 377.6,33.4 413.8,31.9 450.0,23.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.72 µs | 8.74 µs | 7.41 µs | 12.7 µs | 14.9 µs |
| D38 | 4.55 µs | 12.4 µs | 9.22 µs | 17.1 µs | 21.2 µs |
| D57 | 4.01 µs | 4.07 µs | 4.37 µs | 4.9 µs | 5.23 µs |
| D76 | 4.46 µs | 4.87 µs | 4.37 µs | 4.02 µs | 5.35 µs |
| D115 | 8.12 µs | 8.93 µs | 8.55 µs | 10.1 µs | 9.66 µs |
| D153 | 8.2 µs | 8.3 µs | 8.98 µs | 9.77 µs | 11.7 µs |
| D230 | 9.12 µs | 12 µs | 14.6 µs | 14.7 µs | 14.4 µs |
| D307 | 15.9 µs | 18.2 µs | 21.2 µs | 23.6 µs | 26.2 µs |
| D462 | 13.9 µs | 20.3 µs | 19.8 µs | 29.9 µs | 31.3 µs |
| D616 | 28.5 µs | 39.7 µs | 50.9 µs | 61.1 µs | 71.5 µs |
| D924 | 34.8 µs | 58.4 µs | 95.7 µs | 123 µs | 134 µs |
| D1232 | 47.3 µs | 114 µs | 139 µs | 218 µs | 225 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.8 88.2,166.1 124.4,169.8 160.5,166.7 196.7,149.4 232.9,149.1 269.1,146.0 305.3,130.0 341.5,133.9 377.6,113.0 413.8,107.3 450.0,98.4 450.0,53.2 413.8,68.2 377.6,86.4 341.5,110.3 305.3,115.5 269.1,132.7 232.9,138.7 196.7,144.3 160.5,161.4 124.4,162.1 88.2,121.6 52.0,131.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,150.8 88.2,166.1 124.4,169.8 160.5,166.7 196.7,149.4 232.9,149.1 269.1,146.0 305.3,130.0 341.5,133.9 377.6,113.0 413.8,107.3 450.0,98.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,147.2 88.2,137.1 124.4,169.4 160.5,164.2 196.7,146.6 232.9,148.7 269.1,138.1 305.3,125.9 341.5,122.9 377.6,103.4 413.8,92.2 450.0,73.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.0 88.2,145.7 124.4,167.3 160.5,167.3 196.7,147.9 232.9,146.5 269.1,132.4 305.3,121.6 341.5,123.5 377.6,96.2 413.8,77.9 450.0,67.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.4 88.2,127.8 124.4,164.0 160.5,169.7 196.7,143.0 232.9,144.0 269.1,132.1 305.3,118.5 341.5,111.7 377.6,90.9 413.8,70.7 450.0,54.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,121.6 124.4,162.1 160.5,161.4 196.7,144.3 232.9,138.7 269.1,132.7 305.3,115.5 341.5,110.3 377.6,86.4 413.8,68.2 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.885 ns | 2.99 ns | 3.02 ns | 4.92 ns | 4.94 ns |
| D38 | 3.5 ns | 13.7 ns | 24.6 ns | 25 ns | 30.3 ns |
| D57 | 6.54 ns | 20.8 ns | 32.9 ns | 77.8 ns | 77.3 ns |
| D76 | 5.64 ns | 38.1 ns | 42.1 ns | 74.8 ns | 107 ns |
| D115 | 13.6 ns | 57.2 ns | 93.4 ns | 214 ns | 226 ns |
| D153 | 16.9 ns | 52.8 ns | 112 ns | 234 ns | 396 ns |
| D230 | 21.6 ns | 122 ns | 371 ns | 517 ns | 841 ns |
| D307 | 44.4 ns | 169 ns | 459 ns | 1.05 µs | 1.41 µs |
| D462 | 55.5 ns | 416 ns | 1.05 µs | 1.86 µs | 2.43 µs |
| D616 | 100 ns | 642 ns | 1.74 µs | 2.72 µs | 3.88 µs |
| D924 | 113 ns | 1.27 µs | 2.96 µs | 4.92 µs | 7.58 µs |
| D1232 | 150 ns | 2.4 µs | 4.56 µs | 8.92 µs | 12.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,158.5 124.4,149.5 160.5,151.6 196.7,138.9 232.9,135.8 269.1,132.2 305.3,121.8 341.5,118.5 377.6,110.0 413.8,108.2 450.0,104.2 450.0,39.6 413.8,47.3 377.6,57.0 341.5,63.8 305.3,71.7 269.1,79.2 232.9,90.1 196.7,98.2 160.5,109.0 124.4,113.7 88.2,127.3 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,158.5 124.4,149.5 160.5,151.6 196.7,138.9 232.9,135.8 269.1,132.2 305.3,121.8 341.5,118.5 377.6,110.0 413.8,108.2 450.0,104.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,160.8 88.2,138.8 124.4,132.7 160.5,124.0 196.7,118.1 232.9,119.2 269.1,107.2 305.3,102.4 341.5,89.4 377.6,83.1 413.8,73.2 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.7 88.2,130.3 124.4,126.1 160.5,122.5 196.7,111.0 232.9,108.3 269.1,91.0 305.3,87.9 341.5,76.0 377.6,68.6 413.8,60.9 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,130.1 124.4,113.6 160.5,114.2 196.7,99.0 232.9,97.7 269.1,86.2 305.3,76.0 341.5,67.7 377.6,62.2 413.8,53.6 450.0,45.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,127.3 124.4,113.7 160.5,109.0 196.7,98.2 232.9,90.1 269.1,79.2 305.3,71.7 341.5,63.8 377.6,57.0 413.8,47.3 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.623 ns | 0.622 ns | 0.346 ns | 0.622 ns | 0.703 ns |
| D38 | 1.15 ns | 1.45 ns | 1.15 ns | 1.33 ns | 1.45 ns |
| D57 | 1.87 ns | 1.87 ns | 1.87 ns | 1.74 ns | 1.74 ns |
| D76 | 2.16 ns | 2.17 ns | 2.1 ns | 1.8 ns | 2.63 ns |
| D115 | 3.17 ns | 3.17 ns | 2.86 ns | 3.55 ns | 3.25 ns |
| D153 | 4.22 ns | 3.82 ns | 4.29 ns | 4.29 ns | 4.6 ns |
| D230 | 5.16 ns | 5.86 ns | 7.24 ns | 7.16 ns | 5.61 ns |
| D307 | 10.9 ns | 11.1 ns | 11.1 ns | 11.1 ns | 11.1 ns |
| D462 | 10.4 ns | 15.3 ns | 13.9 ns | 16.7 ns | 15.3 ns |
| D616 | 23.5 ns | 20.2 ns | 20.2 ns | 20.2 ns | 20 ns |
| D924 | 45.5 ns | 69.1 ns | 76.4 ns | 76.7 ns | 66.9 ns |
| D1232 | 33.7 ns | 69.8 ns | 43.9 ns | 69.8 ns | 43.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.0 88.2,139.2 124.4,125.2 160.5,121.0 196.7,110.0 232.9,101.6 269.1,95.8 305.3,74.2 341.5,75.5 377.6,52.0 413.8,32.8 450.0,41.5 450.0,33.8 413.8,21.7 377.6,56.7 341.5,64.3 305.3,73.7 269.1,93.4 232.9,99.1 196.7,109.2 160.5,115.3 124.4,127.3 88.2,132.6 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.0 88.2,139.2 124.4,125.2 160.5,121.0 196.7,110.0 232.9,101.6 269.1,95.8 305.3,74.2 341.5,75.5 377.6,52.0 413.8,32.8 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,132.6 124.4,125.2 160.5,120.9 196.7,110.0 232.9,104.5 269.1,92.1 305.3,73.7 341.5,64.4 377.6,56.3 413.8,20.7 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.1 88.2,139.2 124.4,125.2 160.5,121.9 196.7,112.9 232.9,101.1 269.1,86.0 305.3,73.7 341.5,67.1 377.6,56.2 413.8,17.8 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,126.4 196.7,106.7 232.9,101.1 269.1,86.3 305.3,73.7 341.5,61.9 377.6,56.2 413.8,17.7 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,127.3 160.5,115.3 196.7,109.2 232.9,99.1 269.1,93.4 305.3,73.7 341.5,64.3 377.6,56.7 413.8,21.7 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 4.86 µs | 4.64 µs | 7.38 µs | 8.6 µs |
| D38 | 5.9 ns | 7.45 µs | 5.44 µs | 9.37 µs | 11.5 µs |
| D57 | 56.3 ns | 4 µs | 4.42 µs | 4.93 µs | 5.49 µs |
| D76 | 78.6 ns | 4.35 µs | 4.5 µs | 4.51 µs | 5.84 µs |
| D115 | 149 ns | 8.64 µs | 8.79 µs | 11 µs | 10.5 µs |
| D153 | 191 ns | 8.43 µs | 9.44 µs | 10.8 µs | 13 µs |
| D230 | 267 ns | 12.3 µs | 16.2 µs | 17.7 µs | 17.8 µs |
| D307 | 367 ns | 18.9 µs | 21 µs | 28.1 µs | 31.7 µs |
| D462 | 530 ns | 69.8 µs | 117 µs | 220 µs | 261 µs |
| D616 | 793 ns | 178 µs | 327 µs | 352 µs | 528 µs |
| D924 | 740 ns | 353 µs | 454 µs | 783 µs | 1.61 ms |
| D1232 | 1.11 µs | 830 µs | 775 µs | 2.31 ms | 2.62 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,188.0 124.4,160.0 160.5,155.9 196.7,147.9 232.9,144.8 269.1,140.7 305.3,136.7 341.5,132.2 377.6,127.2 413.8,128.0 450.0,123.0 450.0,26.6 413.8,32.7 377.6,46.5 341.5,55.2 305.3,81.4 269.1,88.5 232.9,92.5 196.7,95.2 160.5,102.4 124.4,103.1 88.2,94.0 52.0,97.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,188.0 124.4,160.0 160.5,155.9 196.7,147.9 232.9,144.8 269.1,140.7 305.3,136.7 341.5,132.2 377.6,127.2 413.8,128.0 450.0,123.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.7 88.2,99.4 124.4,107.1 160.5,106.0 196.7,97.5 232.9,97.8 269.1,93.1 305.3,87.8 341.5,71.6 377.6,60.0 413.8,51.5 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.2 88.2,103.3 124.4,105.9 160.5,105.6 196.7,97.3 232.9,96.4 269.1,89.7 305.3,86.5 341.5,65.2 377.6,52.4 413.8,48.4 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.5 88.2,96.5 124.4,104.5 160.5,105.6 196.7,94.6 232.9,94.7 269.1,88.6 305.3,82.9 341.5,57.4 377.6,51.5 413.8,41.6 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,94.0 124.4,103.1 160.5,102.4 196.7,95.2 232.9,92.5 269.1,88.5 305.3,81.4 341.5,55.2 377.6,46.5 413.8,32.7 450.0,26.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.87 ns | 1.87 ns | 1.73 ns | 2.18 ns | 2.46 ns |
| D38 | 6.62 ns | 8.09 ns | 10.7 ns | 12.6 ns | 16.5 ns |
| D57 | 7.16 ns | 7.18 ns | 7.18 ns | 8.09 ns | 8.08 ns |
| D76 | 9.84 ns | 9.84 ns | 8.46 ns | 6.76 ns | 9.49 ns |
| D115 | 14.4 ns | 14.4 ns | 12.5 ns | 14.1 ns | 12.4 ns |
| D153 | 20.7 ns | 15.9 ns | 15.9 ns | 16.2 ns | 20.1 ns |
| D230 | 28.4 ns | 32.2 ns | 37.1 ns | 32.4 ns | 28.1 ns |
| D307 | 41.2 ns | 40.1 ns | 44.1 ns | 43.1 ns | 43.4 ns |
| D462 | 57.8 ns | 74.8 ns | 67.5 ns | 87.7 ns | 75.2 ns |
| D616 | 103 ns | 82.1 ns | 82.2 ns | 78.4 ns | 77.2 ns |
| D924 | 75.5 ns | 99 ns | 103 ns | 96.6 ns | 66.3 ns |
| D1232 | 107 ns | 143 ns | 93.2 ns | 128 ns | 84.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.9 88.2,155.3 124.4,153.0 160.5,143.8 196.7,132.8 232.9,122.2 269.1,113.1 305.3,102.3 341.5,92.5 377.6,75.7 413.8,84.8 450.0,74.7 450.0,81.4 413.8,88.6 377.6,84.2 341.5,84.9 305.3,100.8 269.1,113.4 232.9,123.2 196.7,137.0 160.5,144.8 124.4,149.5 88.2,128.8 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.9 88.2,155.3 124.4,153.0 160.5,143.8 196.7,132.8 232.9,122.2 269.1,113.1 305.3,102.3 341.5,92.5 377.6,75.7 413.8,84.8 450.0,74.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,149.5 124.4,152.9 160.5,143.8 196.7,132.7 232.9,129.8 269.1,109.5 305.3,103.1 341.5,85.1 377.6,82.4 413.8,76.9 450.0,66.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.2 88.2,141.5 124.4,152.9 160.5,148.2 196.7,137.0 232.9,130.0 269.1,105.4 305.3,100.4 341.5,88.1 377.6,82.3 413.8,75.7 450.0,78.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,136.6 124.4,149.5 160.5,154.7 196.7,133.5 232.9,129.4 269.1,109.3 305.3,101.0 341.5,80.5 377.6,83.7 413.8,77.7 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,128.8 124.4,149.5 160.5,144.8 196.7,137.0 232.9,123.2 269.1,113.4 305.3,100.8 341.5,84.9 377.6,84.2 413.8,88.6 450.0,81.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 3.84 µs | 3.18 µs | 5.85 µs | 6.9 µs |
| D38 | 3.86 ns | 5.76 µs | 3.89 µs | 7.74 µs | 9.64 µs |
| D57 | 2.19 ns | 3.09 µs | 4.05 µs | 5.6 µs | 9.54 µs |
| D76 | 3.87 ns | 3.69 µs | 4.81 µs | 6.27 µs | 9 µs |
| D115 | 17.1 ns | 4.43 µs | 9.64 µs | 13.8 µs | 17.3 µs |
| D153 | 22.5 ns | 4.85 µs | 9.37 µs | 17.8 µs | 31.3 µs |
| D230 | 40.8 ns | 9.81 µs | 19.5 µs | 35.8 µs | 58.3 µs |
| D307 | 77 ns | 11.4 µs | 24.4 µs | 66.3 µs | 108 µs |
| D462 | 135 ns | 14.9 µs | 51.2 µs | 140 µs | 216 µs |
| D616 | 176 ns | 29.4 µs | 121 µs | 253 µs | 425 µs |
| D924 | 106 ns | 59.9 µs | 250 µs | 562 µs | 1.01 ms |
| D1232 | 251 ns | 124 µs | 400 µs | 1.11 ms | 2.26 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,193.2 124.4,200.3 160.5,193.2 196.7,174.7 232.9,171.4 269.1,164.0 305.3,156.1 341.5,149.1 377.6,145.8 413.8,152.1 450.0,141.4 450.0,28.4 413.8,38.4 377.6,49.2 341.5,57.6 305.3,66.2 269.1,73.8 232.9,81.6 196.7,88.9 160.5,97.0 124.4,96.3 88.2,96.2 52.0,100.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,193.2 124.4,200.3 160.5,193.2 196.7,174.7 232.9,171.4 269.1,164.0 305.3,156.1 341.5,149.1 377.6,145.8 413.8,152.1 450.0,141.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.6 88.2,102.6 124.4,110.3 160.5,108.1 196.7,105.8 232.9,104.7 269.1,96.0 305.3,94.1 341.5,90.8 377.6,82.3 413.8,73.5 450.0,64.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,107.4 124.4,106.9 160.5,104.8 196.7,96.2 232.9,96.5 269.1,87.5 305.3,84.7 341.5,75.4 377.6,64.8 413.8,55.8 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.4 88.2,98.9 124.4,102.9 160.5,101.5 196.7,91.7 232.9,88.6 269.1,79.9 305.3,72.2 341.5,63.0 377.6,55.6 413.8,45.7 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,96.2 124.4,96.3 160.5,97.0 196.7,88.9 232.9,81.6 269.1,73.8 305.3,66.2 341.5,57.6 377.6,49.2 413.8,38.4 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.35 µs | 5.46 µs | 10.2 µs | 11.9 µs |
| D38 | 2.6 ns | 10.2 µs | 6.62 µs | 13.4 µs | 16.5 µs |
| D57 | 10.6 ns | 5.26 µs | 7.03 µs | 8.42 µs | 10.8 µs |
| D76 | 12.1 ns | 6.08 µs | 7.39 µs | 9.13 µs | 12.8 µs |
| D115 | 11.2 ns | 12.7 µs | 11.1 µs | 22.8 µs | 25.4 µs |
| D153 | 20.8 ns | 7.52 µs | 14.7 µs | 22.5 µs | 39.1 µs |
| D230 | 41.2 ns | 13.6 µs | 24.4 µs | 42.5 µs | 68.6 µs |
| D307 | 76.7 ns | 16.2 µs | 48.2 µs | 79 µs | 118 µs |
| D462 | 124 ns | 23 µs | 68.4 µs | 165 µs | 231 µs |
| D616 | 174 ns | 36.5 µs | 133 µs | 268 µs | 414 µs |
| D924 | 129 ns | 68.5 µs | 268 µs | 555 µs | 894 µs |
| D1232 | 260 ns | 141 µs | 398 µs | 992 µs | 2.94 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,198.2 124.4,180.7 160.5,179.1 196.7,180.0 232.9,172.4 269.1,163.9 305.3,156.1 341.5,150.2 377.6,146.0 413.8,149.7 450.0,141.0 450.0,25.2 413.8,40.0 377.6,49.5 341.5,56.7 305.3,65.1 269.1,71.8 232.9,78.8 196.7,84.1 160.5,92.6 124.4,94.7 88.2,89.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,198.2 124.4,180.7 160.5,179.1 196.7,180.0 232.9,172.4 269.1,163.9 305.3,156.1 341.5,150.2 377.6,146.0 413.8,149.7 450.0,141.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,95.5 124.4,103.7 160.5,101.9 196.7,92.7 232.9,99.2 269.1,91.9 305.3,89.7 341.5,85.4 377.6,79.7 413.8,71.8 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.2 88.2,100.8 124.4,100.1 160.5,99.5 196.7,94.4 232.9,90.9 269.1,84.6 305.3,76.2 341.5,71.9 377.6,63.6 413.8,54.9 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.4 88.2,92.1 124.4,97.8 160.5,96.8 196.7,85.5 232.9,85.6 269.1,77.8 305.3,70.1 341.5,61.0 377.6,54.9 413.8,45.9 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,89.5 124.4,94.7 160.5,92.6 196.7,84.1 232.9,78.8 269.1,71.8 305.3,65.1 341.5,56.7 377.6,49.5 413.8,40.0 450.0,25.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.37 ns | 14.5 ns | 20.7 ns | 29.9 ns | 30.7 ns |
| D38 | 8.21 ns | 36.5 ns | 35 ns | 1.47 µs | 3.19 µs |
| D57 | 158 ns | 206 ns | 487 ns | 672 ns | 666 ns |
| D76 | 209 ns | 278 ns | 769 ns | 626 ns | 1.05 µs |
| D115 | 124 ns | 605 ns | 1.01 µs | 1.56 µs | 1.7 µs |
| D153 | 127 ns | 1.11 µs | 1.59 µs | 1.9 µs | 2.6 µs |
| D230 | 144 ns | 1.55 µs | 2.38 µs | 3.5 µs | 3.37 µs |
| D307 | 151 ns | 2.37 µs | 3.65 µs | 4.65 µs | 6.95 µs |
| D462 | 148 ns | 3.68 µs | 4.93 µs | 9.98 µs | 11.9 µs |
| D616 | 269 ns | 6.7 µs | 10.7 µs | 14.3 µs | 20.5 µs |
| D924 | 173 ns | 8.98 µs | 17.3 µs | 26.3 µs | 31.1 µs |
| D1232 | 207 ns | 16.3 µs | 24.1 µs | 40.6 µs | 55.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.8 88.2,173.4 124.4,122.1 160.5,117.2 196.7,126.3 232.9,125.8 269.1,123.7 305.3,122.8 341.5,123.1 377.6,112.8 413.8,120.4 450.0,117.4 450.0,20.3 413.8,30.3 377.6,37.5 341.5,47.0 305.3,56.3 269.1,68.9 232.9,73.4 196.7,80.8 160.5,89.2 124.4,97.0 88.2,69.9 52.0,150.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.8 88.2,173.4 124.4,122.1 160.5,117.2 196.7,126.3 232.9,125.8 269.1,123.7 305.3,122.8 341.5,123.1 377.6,112.8 413.8,120.4 450.0,117.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.5 88.2,147.5 124.4,117.4 160.5,112.3 196.7,98.7 232.9,88.1 269.1,82.4 305.3,75.0 341.5,67.4 377.6,57.0 413.8,51.9 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.4 88.2,148.2 124.4,102.5 160.5,94.6 196.7,89.8 232.9,81.9 269.1,74.9 305.3,67.5 341.5,62.3 377.6,48.8 413.8,40.5 450.0,34.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.0 88.2,83.3 124.4,96.9 160.5,98.1 196.7,82.3 232.9,78.9 269.1,68.2 305.3,63.3 341.5,50.0 377.6,43.7 413.8,33.2 450.0,25.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.5 88.2,69.9 124.4,97.0 160.5,89.2 196.7,80.8 232.9,73.4 269.1,68.9 305.3,56.3 341.5,47.0 377.6,37.5 413.8,30.3 450.0,20.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.936 ns | 0.937 ns | 1.11 ns | 0.935 ns | 1.05 ns |
| D38 | 1.44 ns | 1.81 ns | 1.44 ns | 1.6 ns | 1.81 ns |
| D57 | 2.26 ns | 2.25 ns | 3.2 ns | 2.5 ns | 2.5 ns |
| D76 | 3.47 ns | 3.46 ns | 3.09 ns | 2.45 ns | 3.44 ns |
| D115 | 5.55 ns | 5.55 ns | 4.85 ns | 5.56 ns | 4.83 ns |
| D153 | 8.47 ns | 7.68 ns | 7.64 ns | 7.62 ns | 8.44 ns |
| D230 | 13.7 ns | 16.1 ns | 17.6 ns | 16.1 ns | 13.7 ns |
| D307 | 23.5 ns | 23.5 ns | 23.5 ns | 23.5 ns | 23.5 ns |
| D462 | 29.6 ns | 37.5 ns | 34.1 ns | 40.6 ns | 37.8 ns |
| D616 | 71.7 ns | 45.9 ns | 50.3 ns | 49.9 ns | 45.3 ns |
| D924 | 67.5 ns | 71.4 ns | 75.5 ns | 75.3 ns | 67.5 ns |
| D1232 | 77.9 ns | 106 ns | 78 ns | 106 ns | 77.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.4 88.2,152.1 124.4,142.3 160.5,133.0 196.7,122.8 232.9,113.6 269.1,103.2 305.3,91.4 341.5,86.5 377.6,67.2 413.8,68.5 450.0,65.4 450.0,65.5 413.8,68.5 377.6,77.2 341.5,81.1 305.3,91.4 269.1,103.2 232.9,113.7 196.7,125.8 160.5,133.1 124.4,140.1 88.2,147.1 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.4 88.2,152.1 124.4,142.3 160.5,133.0 196.7,122.8 232.9,113.6 269.1,103.2 305.3,91.4 341.5,86.5 377.6,67.2 413.8,68.5 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,147.1 124.4,142.4 160.5,133.1 196.7,122.8 232.9,115.7 269.1,99.6 305.3,91.5 341.5,81.3 377.6,76.9 413.8,67.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.7 88.2,152.1 124.4,134.7 160.5,135.5 196.7,125.7 232.9,115.8 269.1,97.7 305.3,91.5 341.5,83.4 377.6,74.9 413.8,66.1 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.8 124.4,140.1 160.5,140.6 196.7,122.8 232.9,115.9 269.1,99.6 305.3,91.4 341.5,79.6 377.6,75.1 413.8,66.2 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.1 124.4,140.1 160.5,133.1 196.7,125.8 232.9,113.7 269.1,103.2 305.3,91.4 341.5,81.1 377.6,77.2 413.8,68.5 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 7.64 µs | 6.05 µs | 10.8 µs | 12.7 µs |
| D38 | 3.79 ns | 10.7 µs | 7.4 µs | 14.4 µs | 17.7 µs |
| D57 | 2.8 ns | 4.15 µs | 5.51 µs | 7.43 µs | 9.42 µs |
| D76 | 4.22 ns | 4.92 µs | 6.54 µs | 8.01 µs | 11.3 µs |
| D115 | 16.8 ns | 5.94 µs | 12 µs | 17.5 µs | 21 µs |
| D153 | 22.2 ns | 6.53 µs | 12 µs | 21.1 µs | 36.6 µs |
| D230 | 45.8 ns | 11.9 µs | 23.1 µs | 40.7 µs | 64.9 µs |
| D307 | 74.7 ns | 14.3 µs | 29 µs | 74.7 µs | 120 µs |
| D462 | 101 ns | 18.3 µs | 59.7 µs | 155 µs | 233 µs |
| D616 | 169 ns | 33.6 µs | 134 µs | 278 µs | 460 µs |
| D924 | 101 ns | 70.6 µs | 275 µs | 605 µs | 1.08 ms |
| D1232 | 244 ns | 138 µs | 429 µs | 1.19 ms | 2.38 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,193.5 124.4,197.2 160.5,192.1 196.7,175.0 232.9,171.5 269.1,162.6 305.3,156.5 341.5,152.7 377.6,146.3 413.8,152.7 450.0,141.8 450.0,27.8 413.8,37.7 377.6,48.2 341.5,56.6 305.3,64.9 269.1,72.5 232.9,79.6 196.7,86.5 160.5,94.2 124.4,96.5 88.2,88.6 52.0,92.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,193.5 124.4,197.2 160.5,192.1 196.7,175.0 232.9,171.5 269.1,162.6 305.3,156.5 341.5,152.7 377.6,146.3 413.8,152.7 450.0,141.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.1 88.2,94.8 124.4,106.6 160.5,104.5 196.7,102.2 232.9,101.0 269.1,93.5 305.3,91.3 341.5,88.2 377.6,80.7 413.8,71.5 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.9 88.2,99.5 124.4,103.1 160.5,101.0 196.7,93.4 232.9,93.5 269.1,85.3 305.3,82.5 341.5,73.5 377.6,63.5 413.8,54.6 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,91.2 124.4,99.4 160.5,98.5 196.7,88.7 232.9,86.5 269.1,78.3 305.3,70.8 341.5,61.7 377.6,54.5 413.8,44.8 450.0,36.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.8 88.2,88.6 124.4,96.5 160.5,94.2 196.7,86.5 232.9,79.6 269.1,72.5 305.3,64.9 341.5,56.6 377.6,48.2 413.8,37.7 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.36 µs | 5.61 µs | 10.4 µs | 12.1 µs |
| D38 | 2.82 ns | 10.3 µs | 6.63 µs | 13.4 µs | 16.5 µs |
| D57 | 2.65 µs | 5.35 µs | 7.32 µs | 8.69 µs | 11.3 µs |
| D76 | 2.87 µs | 6.21 µs | 7.75 µs | 9.45 µs | 13.1 µs |
| D115 | 5.87 µs | 13.2 µs | 11.7 µs | 24 µs | 26 µs |
| D153 | 3.08 µs | 7.8 µs | 15.5 µs | 23.1 µs | 39.7 µs |
| D230 | 2.54 µs | 14.3 µs | 24.7 µs | 43.5 µs | 68.4 µs |
| D307 | 3.12 µs | 16.5 µs | 50.1 µs | 79.6 µs | 119 µs |
| D462 | 3.01 µs | 23.5 µs | 69.8 µs | 168 µs | 233 µs |
| D616 | 3.77 µs | 36.8 µs | 135 µs | 271 µs | 419 µs |
| D924 | 3.33 µs | 69.6 µs | 270 µs | 560 µs | 903 µs |
| D1232 | 3.83 µs | 144 µs | 404 µs | 1 ms | 2.95 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,197.2 124.4,112.2 160.5,111.2 196.7,102.3 232.9,110.3 269.1,112.7 305.3,110.2 341.5,110.6 377.6,107.8 413.8,109.3 450.0,107.6 450.0,25.1 413.8,39.8 377.6,49.4 341.5,56.6 305.3,65.0 269.1,71.9 232.9,78.6 196.7,83.8 160.5,92.3 124.4,94.2 88.2,89.5 52.0,93.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,197.2 124.4,112.2 160.5,111.2 196.7,102.3 232.9,110.3 269.1,112.7 305.3,110.2 341.5,110.6 377.6,107.8 413.8,109.3 450.0,107.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,95.3 124.4,103.5 160.5,101.6 196.7,92.3 232.9,98.8 269.1,91.3 305.3,89.5 341.5,85.1 377.6,79.5 413.8,71.6 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.9 88.2,100.8 124.4,99.6 160.5,98.9 196.7,93.7 232.9,90.3 269.1,84.5 305.3,75.7 341.5,71.6 377.6,63.4 413.8,54.8 450.0,49.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,92.0 124.4,97.5 160.5,96.4 196.7,84.9 232.9,85.3 269.1,77.5 305.3,70.0 341.5,60.7 377.6,54.8 413.8,45.8 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.3 88.2,89.5 124.4,94.2 160.5,92.3 196.7,83.8 232.9,78.6 269.1,71.9 305.3,65.0 341.5,56.6 377.6,49.4 413.8,39.8 450.0,25.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.74 ns | 3 µs | 1.46 µs | 3.53 µs | 4.14 µs |
| D38 | 3.76 ns | 3.46 µs | 1.81 µs | 4.72 µs | 5.85 µs |
| D57 | 183 ns | 291 ns | 301 ns | 337 ns | 434 ns |
| D76 | 200 ns | 316 ns | 308 ns | 364 ns | 463 ns |
| D115 | 376 ns | 545 ns | 587 ns | 758 ns | 765 ns |
| D153 | 385 ns | 498 ns | 625 ns | 752 ns | 1.01 µs |
| D230 | 497 ns | 772 ns | 1.04 µs | 1.23 µs | 1.52 µs |
| D307 | 840 ns | 1.14 µs | 1.45 µs | 2.04 µs | 2.54 µs |
| D462 | 781 ns | 1.31 µs | 1.78 µs | 3.17 µs | 3.83 µs |
| D616 | 1.18 µs | 1.74 µs | 2.83 µs | 4.05 µs | 5.56 µs |
| D924 | 1.19 µs | 2.27 µs | 4.32 µs | 6.72 µs | 10 µs |
| D1232 | 1.68 µs | 4.06 µs | 6.27 µs | 11.7 µs | 32.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.1 88.2,187.0 124.4,119.5 160.5,117.9 196.7,107.0 232.9,106.6 269.1,102.1 305.3,93.0 341.5,94.3 377.6,87.2 413.8,87.0 450.0,81.0 450.0,29.5 413.8,50.0 377.6,60.2 341.5,66.7 305.3,73.8 269.1,82.8 232.9,89.8 196.7,94.6 160.5,103.4 124.4,104.5 88.2,59.3 52.0,65.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.1 88.2,187.0 124.4,119.5 160.5,117.9 196.7,107.0 232.9,106.6 269.1,102.1 305.3,93.0 341.5,94.3 377.6,87.2 413.8,87.0 450.0,81.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,70.9 88.2,68.4 124.4,111.4 160.5,110.0 196.7,100.5 232.9,102.1 269.1,94.5 305.3,87.7 341.5,85.3 377.6,80.4 413.8,75.7 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.5 88.2,79.7 124.4,110.8 160.5,110.4 196.7,99.3 232.9,98.2 269.1,89.3 305.3,83.6 341.5,80.0 377.6,71.9 413.8,64.6 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.1 88.2,63.1 124.4,108.9 160.5,107.6 196.7,94.8 232.9,95.0 269.1,86.4 305.3,77.6 341.5,70.0 377.6,65.7 413.8,56.9 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,65.3 88.2,59.3 124.4,104.5 160.5,103.4 196.7,94.6 232.9,89.8 269.1,82.8 305.3,73.8 341.5,66.7 377.6,60.2 413.8,50.0 450.0,29.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 142 ns | 175 ns | 184 ns | 206 ns |
| D38 | 3.98 ns | 201 ns | 172 ns | 187 ns | 206 ns |
| D57 | 281 ns | 408 ns | 407 ns | 448 ns | 528 ns |
| D76 | 306 ns | 407 ns | 418 ns | 479 ns | 599 ns |
| D115 | 614 ns | 713 ns | 802 ns | 1 µs | 971 ns |
| D153 | 630 ns | 714 ns | 869 ns | 963 ns | 1.26 µs |
| D230 | 801 ns | 1.13 µs | 1.39 µs | 1.64 µs | 1.84 µs |
| D307 | 1.39 µs | 1.65 µs | 1.97 µs | 2.57 µs | 3.09 µs |
| D462 | 1.27 µs | 1.81 µs | 2.28 µs | 3.79 µs | 4.47 µs |
| D616 | 1.89 µs | 2.31 µs | 3.54 µs | 4.79 µs | 6.35 µs |
| D924 | 1.88 µs | 3.04 µs | 5.32 µs | 7.76 µs | 10.9 µs |
| D1232 | 2.63 µs | 5.25 µs | 7.29 µs | 13.3 µs | 33.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,186.0 124.4,112.0 160.5,110.6 196.7,98.5 232.9,98.0 269.1,93.9 305.3,84.3 341.5,85.9 377.6,78.9 413.8,79.0 450.0,73.2 450.0,28.8 413.8,48.5 377.6,57.9 341.5,64.0 305.3,70.4 269.1,79.4 232.9,86.0 196.7,90.5 160.5,98.9 124.4,101.1 88.2,117.4 52.0,117.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,186.0 124.4,112.0 160.5,110.6 196.7,98.5 232.9,98.0 269.1,93.9 305.3,84.3 341.5,85.9 377.6,78.9 413.8,79.0 450.0,73.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.9 88.2,117.9 124.4,105.6 160.5,105.6 196.7,95.9 232.9,95.9 269.1,87.9 305.3,81.3 341.5,79.7 377.6,75.5 413.8,70.7 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.3 88.2,120.6 124.4,105.6 160.5,105.2 196.7,93.8 232.9,92.4 269.1,84.3 305.3,78.3 341.5,75.7 377.6,68.0 413.8,61.0 450.0,55.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,119.1 124.4,103.9 160.5,102.8 196.7,90.0 232.9,90.7 269.1,81.4 305.3,73.6 341.5,66.8 377.6,62.8 413.8,54.4 450.0,45.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.5 88.2,117.4 124.4,101.1 160.5,98.9 196.7,90.5 232.9,86.0 269.1,79.4 305.3,70.4 341.5,64.0 377.6,57.9 413.8,48.5 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body -->
