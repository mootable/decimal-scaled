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
| D18 | 7.26 ns | 23.2 µs | 29.7 µs | 32.4 µs | 36.8 µs |
| D38 | 4.65 µs | 5.88 µs | 8.32 µs | 9.62 µs | 13.3 µs |
| D57 | 5.08 µs | 8.46 µs | 12.4 µs | 16.3 µs | 17.4 µs |
| D76 | 4.75 µs | 9.35 µs | 14.3 µs | 16 µs | 19.8 µs |
| D115 | 5.15 µs | 11 µs | 25.6 µs | 31.5 µs | 43.5 µs |
| D153 | 5.17 µs | 16.1 µs | 28.9 µs | 43.6 µs | 60.7 µs |
| D230 | 4.83 µs | 25.6 µs | 25.6 µs | 73.7 µs | 106 µs |
| D307 | 4.23 µs | 26.6 µs | 56.7 µs | 105 µs | 177 µs |
| D462 | 4.89 µs | 40.9 µs | 131 µs | 252 µs | 396 µs |
| D616 | 4.45 µs | 60.3 µs | 215 µs | 424 µs | 655 µs |
| D924 | 4.51 µs | 132 µs | 419 µs | 862 µs | 1.67 ms |
| D1232 | 5.58 µs | 211 µs | 704 µs | 1.5 ms | 3.22 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.4 88.2,105.2 124.4,104.1 160.5,104.9 196.7,104.0 232.9,103.9 269.1,104.8 305.3,106.4 341.5,104.6 377.6,105.8 413.8,105.6 450.0,103.0 450.0,24.0 413.8,32.2 377.6,43.8 341.5,50.1 305.3,60.1 269.1,66.4 232.9,73.3 196.7,77.5 160.5,87.2 124.4,88.8 88.2,92.2 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.4 88.2,105.2 124.4,104.1 160.5,104.9 196.7,104.0 232.9,103.9 269.1,104.8 305.3,106.4 341.5,104.6 377.6,105.8 413.8,105.6 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,102.3 124.4,97.8 160.5,96.5 196.7,94.5 232.9,89.8 269.1,84.0 305.3,83.6 341.5,78.2 377.6,73.4 413.8,63.7 450.0,57.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,98.0 124.4,93.0 160.5,91.3 196.7,84.0 232.9,82.6 269.1,84.1 305.3,74.2 341.5,63.7 377.6,57.7 413.8,49.4 450.0,42.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,96.2 124.4,89.7 160.5,89.9 196.7,81.5 232.9,77.4 269.1,70.9 305.3,66.5 341.5,55.7 377.6,49.2 413.8,40.4 450.0,33.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,92.2 124.4,88.8 160.5,87.2 196.7,77.5 232.9,73.3 269.1,66.4 305.3,60.1 341.5,50.1 377.6,43.8 413.8,32.2 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 27.4 µs | 31 µs | 35.5 µs | 39 µs | 35.9 µs |
| D38 | 27.4 µs | 27.5 µs | 44.3 µs | 44.5 µs | 31.3 µs |
| D57 | 3.7 µs | 4.37 µs | 5.46 µs | 6.65 µs | 7.43 µs |
| D76 | 3.28 µs | 5.05 µs | 5.64 µs | 7.07 µs | 9.22 µs |
| D115 | 6.59 µs | 8.53 µs | 13.4 µs | 16.6 µs | 23.7 µs |
| D153 | 6.7 µs | 10.6 µs | 15.8 µs | 23.4 µs | 30.2 µs |
| D230 | 8.36 µs | 17.2 µs | 15.2 µs | 46.8 µs | 59.6 µs |
| D307 | 10.3 µs | 26.1 µs | 42.1 µs | 70.6 µs | 132 µs |
| D462 | 12.4 µs | 36.8 µs | 89.7 µs | 167 µs | 273 µs |
| D616 | 17.6 µs | 70.9 µs | 179 µs | 314 µs | 524 µs |
| D924 | 27.2 µs | 161 µs | 380 µs | 749 µs | 1.45 ms |
| D1232 | 42.7 µs | 269 µs | 715 µs | 1.43 ms | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,138.1 88.2,138.1 124.4,181.6 160.5,184.2 196.7,169.0 232.9,168.7 269.1,163.9 305.3,159.3 341.5,155.3 377.6,147.7 413.8,138.3 450.0,128.5 450.0,37.3 413.8,52.0 377.6,74.0 341.5,88.2 305.3,104.0 269.1,121.2 232.9,136.0 196.7,141.3 160.5,161.8 124.4,166.4 88.2,135.2 52.0,132.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,138.1 88.2,138.1 124.4,181.6 160.5,184.2 196.7,169.0 232.9,168.7 269.1,163.9 305.3,159.3 341.5,155.3 377.6,147.7 413.8,138.3 450.0,128.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,135.4 88.2,138.0 124.4,178.0 160.5,174.9 196.7,163.5 232.9,158.7 269.1,148.3 305.3,139.2 341.5,131.7 377.6,117.5 413.8,99.7 450.0,88.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.5 88.2,127.7 124.4,173.2 160.5,172.4 196.7,153.6 232.9,150.0 269.1,150.9 305.3,128.8 341.5,112.4 377.6,97.4 413.8,81.0 450.0,67.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,127.6 124.4,168.9 160.5,167.5 196.7,149.0 232.9,141.5 269.1,126.5 305.3,117.6 341.5,98.9 377.6,85.1 413.8,66.3 450.0,52.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.3 88.2,135.2 124.4,166.4 160.5,161.8 196.7,141.3 232.9,136.0 269.1,121.2 305.3,104.0 341.5,88.2 377.6,74.0 413.8,52.0 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.05 ns | 1.06 ns | 1.06 ns | 0.941 ns |
| D38 | 1.83 ns | 1.42 ns | 1.82 ns | 1.41 ns | 1.44 ns |
| D57 | 2.5 ns | 2.25 ns | 2.5 ns | 2.51 ns | 2.24 ns |
| D76 | 3.08 ns | 3.49 ns | 3.08 ns | 2.32 ns | 3.08 ns |
| D115 | 4.98 ns | 4.41 ns | 4.98 ns | 4.42 ns | 4.99 ns |
| D153 | 6.65 ns | 6.65 ns | 6.62 ns | 6.63 ns | 5.94 ns |
| D230 | 13.8 ns | 15.3 ns | 8.03 ns | 15.3 ns | 11.9 ns |
| D307 | 15.2 ns | 18.5 ns | 14.6 ns | 15.2 ns | 18.6 ns |
| D462 | 28.9 ns | 29 ns | 32.7 ns | 32.5 ns | 33.3 ns |
| D616 | 33.4 ns | 45.3 ns | 74.7 ns | 45.2 ns | 45.2 ns |
| D924 | 79.8 ns | 85.5 ns | 75 ns | 76.1 ns | 85.7 ns |
| D1232 | 107 ns | 106 ns | 107 ns | 95.1 ns | 100 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,146.9 124.4,140.1 160.5,135.5 196.7,125.1 232.9,118.9 269.1,102.9 305.3,100.9 341.5,86.9 377.6,83.8 413.8,64.9 450.0,58.6 450.0,60.0 413.8,63.4 377.6,77.2 341.5,83.9 305.3,96.5 269.1,106.2 232.9,121.3 196.7,125.1 160.5,135.6 124.4,142.5 88.2,152.1 52.0,161.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,146.9 124.4,140.1 160.5,135.5 196.7,125.1 232.9,118.9 269.1,102.9 305.3,100.9 341.5,86.9 377.6,83.8 413.8,64.9 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,152.4 124.4,142.4 160.5,132.9 196.7,127.8 232.9,118.9 269.1,100.7 305.3,96.6 341.5,86.9 377.6,77.2 413.8,63.4 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.0 124.4,140.1 160.5,135.5 196.7,125.1 232.9,119.0 269.1,114.8 305.3,101.8 341.5,84.3 377.6,66.3 413.8,66.3 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.5 124.4,140.0 160.5,141.7 196.7,127.7 232.9,118.9 269.1,100.7 305.3,100.9 341.5,84.4 377.6,77.2 413.8,65.9 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.3 88.2,152.1 124.4,142.5 160.5,135.6 196.7,125.1 232.9,121.3 269.1,106.2 305.3,96.5 341.5,83.9 377.6,77.2 413.8,63.4 450.0,60.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 23.2 µs | 29.6 µs | 32.4 µs | 36.8 µs |
| D38 | 4.61 µs | 5.89 µs | 8.23 µs | 9.62 µs | 13.3 µs |
| D57 | 5.02 µs | 8.43 µs | 12.4 µs | 16.3 µs | 17.4 µs |
| D76 | 4.69 µs | 9.34 µs | 14.3 µs | 15.9 µs | 19.8 µs |
| D115 | 5.1 µs | 11 µs | 25.9 µs | 31.7 µs | 46.7 µs |
| D153 | 5.11 µs | 16 µs | 28.5 µs | 43.9 µs | 60.2 µs |
| D230 | 4.73 µs | 25.9 µs | 25.3 µs | 73.6 µs | 106 µs |
| D307 | 4.27 µs | 27 µs | 56 µs | 106 µs | 177 µs |
| D462 | 4.81 µs | 40.7 µs | 131 µs | 249 µs | 397 µs |
| D616 | 4.38 µs | 60.4 µs | 214 µs | 422 µs | 655 µs |
| D924 | 4.36 µs | 131 µs | 419 µs | 862 µs | 1.67 ms |
| D1232 | 5.53 µs | 211 µs | 701 µs | 1.5 ms | 3.22 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,105.3 124.4,104.3 160.5,105.1 196.7,104.1 232.9,104.1 269.1,105.0 305.3,106.3 341.5,104.8 377.6,106.0 413.8,106.0 450.0,103.1 450.0,24.1 413.8,32.2 377.6,43.8 341.5,50.0 305.3,60.0 269.1,66.5 232.9,73.4 196.7,76.6 160.5,87.3 124.4,88.9 88.2,92.2 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,105.3 124.4,104.3 160.5,105.1 196.7,104.1 232.9,104.1 269.1,105.0 305.3,106.3 341.5,104.8 377.6,106.0 413.8,106.0 450.0,103.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.3 88.2,102.3 124.4,97.8 160.5,96.6 196.7,94.5 232.9,89.9 269.1,83.9 305.3,83.4 341.5,78.3 377.6,73.4 413.8,63.8 450.0,57.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,98.1 124.4,93.1 160.5,91.3 196.7,83.9 232.9,82.7 269.1,84.2 305.3,74.3 341.5,63.8 377.6,57.7 413.8,49.4 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,96.2 124.4,89.7 160.5,89.9 196.7,81.4 232.9,77.4 269.1,70.9 305.3,66.4 341.5,55.8 377.6,49.3 413.8,40.4 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,92.2 124.4,88.9 160.5,87.3 196.7,76.6 232.9,73.4 269.1,66.5 305.3,60.0 341.5,50.0 377.6,43.8 413.8,32.2 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 23 µs | 29 µs | 31.8 µs | 36.1 µs |
| D38 | 4.22 ns | 22.5 µs | 36.2 µs | 33.3 µs | 24.5 µs |
| D57 | 2.11 ns | 5.35 µs | 8.02 µs | 9.83 µs | 11.6 µs |
| D76 | 2.01 ns | 6.92 µs | 8.7 µs | 10.6 µs | 13.9 µs |
| D115 | 12.4 ns | 12.5 µs | 20.8 µs | 24.5 µs | 36.2 µs |
| D153 | 15.9 ns | 16 µs | 23.8 µs | 37.1 µs | 44 µs |
| D230 | 28 ns | 26.2 µs | 24.9 µs | 66.9 µs | 83.7 µs |
| D307 | 40.6 ns | 37.2 µs | 58.5 µs | 96.5 µs | 163 µs |
| D462 | 69.5 ns | 55.1 µs | 125 µs | 208 µs | 336 µs |
| D616 | 79.2 ns | 106 µs | 243 µs | 403 µs | 615 µs |
| D924 | 104 ns | 253 µs | 499 µs | 871 µs | 1.61 ms |
| D1232 | 158 ns | 383 µs | 922 µs | 1.62 ms | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,200.7 160.5,201.3 196.7,178.7 232.9,175.7 269.1,168.6 305.3,164.0 341.5,157.4 377.6,155.8 413.8,152.4 450.0,147.2 450.0,25.5 413.8,32.7 377.6,44.6 341.5,52.1 305.3,61.1 269.1,69.4 232.9,77.3 196.7,79.7 160.5,91.7 124.4,93.8 88.2,84.6 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,200.7 160.5,201.3 196.7,178.7 232.9,175.7 269.1,168.6 305.3,164.0 341.5,157.4 377.6,155.8 413.8,152.4 450.0,147.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.4 88.2,85.6 124.4,103.5 160.5,100.3 196.7,92.9 232.9,89.9 269.1,83.7 305.3,79.4 341.5,74.5 377.6,66.4 413.8,55.6 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.5 88.2,79.8 124.4,98.4 160.5,97.4 196.7,86.6 232.9,85.0 269.1,84.4 305.3,73.8 341.5,64.4 377.6,56.1 413.8,47.2 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.3 88.2,80.8 124.4,95.9 160.5,95.0 196.7,84.6 232.9,79.4 269.1,72.1 305.3,67.6 341.5,58.0 377.6,49.9 413.8,40.3 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,84.6 124.4,93.8 160.5,91.7 196.7,79.7 232.9,77.3 269.1,69.4 305.3,61.1 341.5,52.1 377.6,44.6 413.8,32.7 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.63 µs | 2.68 µs | 2.93 µs | 3.06 µs |
| D38 | 6.2 µs | 6.95 µs | 6.88 µs | 8.31 µs | 11.7 µs |
| D57 | 4.14 µs | 7.15 µs | 10.7 µs | 14.2 µs | 4.98 µs |
| D76 | 3.87 µs | 8.11 µs | 12.4 µs | 14.1 µs | 17.8 µs |
| D115 | 4.22 µs | 9.52 µs | 23.3 µs | 28.6 µs | 40 µs |
| D153 | 4.25 µs | 14.1 µs | 22.3 µs | 39.2 µs | 55.7 µs |
| D230 | 3.9 µs | 23.2 µs | 22.7 µs | 69.3 µs | 97.4 µs |
| D307 | 3.5 µs | 23.9 µs | 46 µs | 99.3 µs | 165 µs |
| D462 | 2.98 µs | 32.7 µs | 112 µs | 219 µs | 337 µs |
| D616 | 3.62 µs | 54.9 µs | 200 µs | 393 µs | 622 µs |
| D924 | 3.87 µs | 122 µs | 393 µs | 824 µs | 1.57 ms |
| D1232 | 4.66 µs | 197 µs | 662 µs | 1.44 ms | 3.09 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,101.7 124.4,106.7 160.5,107.5 196.7,106.4 232.9,106.3 269.1,107.4 305.3,108.7 341.5,110.7 377.6,108.3 413.8,107.5 450.0,105.2 450.0,24.6 413.8,33.0 377.6,44.5 341.5,52.1 305.3,60.9 269.1,67.5 232.9,74.4 196.7,78.5 160.5,88.6 124.4,104.4 88.2,93.8 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,101.7 124.4,106.7 160.5,107.5 196.7,106.4 232.9,106.3 269.1,107.4 305.3,108.7 341.5,110.7 377.6,108.3 413.8,107.5 450.0,105.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,100.2 124.4,99.9 160.5,98.3 196.7,96.3 232.9,91.4 269.1,85.3 305.3,84.9 341.5,81.0 377.6,74.6 413.8,64.7 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,100.4 124.4,94.9 160.5,93.1 196.7,85.2 232.9,85.7 269.1,85.5 305.3,76.8 341.5,65.7 377.6,58.5 413.8,50.2 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,98.0 124.4,91.4 160.5,91.5 196.7,82.7 232.9,78.8 269.1,71.7 305.3,67.2 341.5,57.4 377.6,50.2 413.8,41.0 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,93.8 124.4,104.4 160.5,88.6 196.7,78.5 232.9,74.4 269.1,67.5 305.3,60.9 341.5,52.1 377.6,44.5 413.8,33.0 450.0,24.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 7.82 µs | 9.41 µs | 10.4 µs | 10.6 µs |
| D38 | 4.22 ns | 7.3 µs | 11.8 µs | 10.8 µs | 8.74 µs |
| D57 | 616 ns | 5.47 µs | 7.34 µs | 9.07 µs | 11 µs |
| D76 | 499 ns | 6.51 µs | 7.91 µs | 10.3 µs | 13.8 µs |
| D115 | 1.31 µs | 11.7 µs | 19.2 µs | 25 µs | 37 µs |
| D153 | 1.22 µs | 14.7 µs | 24.1 µs | 36.2 µs | 48.1 µs |
| D230 | 1.38 µs | 24.9 µs | 23.1 µs | 76.5 µs | 102 µs |
| D307 | 1.8 µs | 39.5 µs | 68.8 µs | 121 µs | 233 µs |
| D462 | 2.18 µs | 57 µs | 152 µs | 296 µs | 496 µs |
| D616 | 2.88 µs | 114 µs | 309 µs | 562 µs | 958 µs |
| D924 | 4.64 µs | 267 µs | 671 µs | 1.38 ms | 2.7 ms |
| D1232 | 7.6 µs | 460 µs | 1.29 ms | 2.66 ms | 5.36 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,130.3 160.5,132.9 196.7,120.9 232.9,121.9 269.1,120.3 305.3,117.0 341.5,114.6 377.6,111.2 413.8,105.3 450.0,99.1 450.0,17.7 413.8,26.3 377.6,39.1 341.5,47.3 305.3,56.7 269.1,66.9 232.9,76.2 196.7,79.5 160.5,91.7 124.4,94.5 88.2,97.4 52.0,95.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,130.3 160.5,132.9 196.7,120.9 232.9,121.9 269.1,120.3 305.3,117.0 341.5,114.6 377.6,111.2 413.8,105.3 450.0,99.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.8 88.2,99.6 124.4,103.2 160.5,101.0 196.7,93.8 232.9,91.0 269.1,84.4 305.3,78.7 341.5,74.1 377.6,65.5 413.8,54.9 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,93.6 124.4,99.5 160.5,98.6 196.7,87.6 232.9,84.8 269.1,85.4 305.3,71.8 341.5,61.9 377.6,53.1 413.8,43.5 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,94.8 124.4,96.9 160.5,95.4 196.7,84.3 232.9,79.8 269.1,70.5 305.3,64.8 341.5,53.7 377.6,45.7 413.8,34.6 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.0 88.2,97.4 124.4,94.5 160.5,91.7 196.7,79.5 232.9,76.2 269.1,66.9 305.3,56.7 341.5,47.3 377.6,39.1 413.8,26.3 450.0,17.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.73 µs | 5.77 µs | 5.8 µs | 8.69 µs | 7.7 µs |
| D38 | 5.73 µs | 4.5 µs | 8.7 µs | 8.47 µs | 4.67 µs |
| D57 | 348 ns | 594 ns | 659 ns | 1.04 µs | 1.14 µs |
| D76 | 464 ns | 761 ns | 1.19 µs | 1.12 µs | 1.94 µs |
| D115 | 343 ns | 2.31 µs | 2.59 µs | 3.53 µs | 5.34 µs |
| D153 | 357 ns | 2.66 µs | 3.5 µs | 5.83 µs | 6.87 µs |
| D230 | 549 ns | 5.34 µs | 5 µs | 10.6 µs | 12.4 µs |
| D307 | 371 ns | 5.96 µs | 9.46 µs | 14.3 µs | 21 µs |
| D462 | 649 ns | 9.63 µs | 26.3 µs | 36 µs | 51.6 µs |
| D616 | 404 ns | 15.3 µs | 40.5 µs | 57.8 µs | 77.6 µs |
| D924 | 506 ns | 31.2 µs | 82.5 µs | 122 µs | 199 µs |
| D1232 | 830 ns | 49.6 µs | 150 µs | 227 µs | 312 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,122.1 88.2,122.1 124.4,182.9 160.5,176.7 196.7,183.2 232.9,182.3 269.1,173.0 305.3,181.5 341.5,169.4 377.6,179.7 413.8,174.8 450.0,164.0 450.0,35.3 413.8,45.1 377.6,65.5 341.5,74.4 305.3,93.8 269.1,105.3 232.9,118.1 196.7,123.6 160.5,145.7 124.4,157.2 88.2,126.5 52.0,115.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,122.1 88.2,122.1 124.4,182.9 160.5,176.7 196.7,183.2 232.9,182.3 269.1,173.0 305.3,181.5 341.5,169.4 377.6,179.7 413.8,174.8 450.0,164.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,127.3 124.4,171.3 160.5,165.9 196.7,141.8 232.9,138.8 269.1,123.6 305.3,121.2 341.5,110.8 377.6,100.8 413.8,85.3 450.0,75.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.8 88.2,113.0 124.4,169.1 160.5,156.3 196.7,139.3 232.9,132.8 269.1,125.1 305.3,111.2 341.5,89.0 377.6,79.6 413.8,64.2 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,113.6 124.4,159.2 160.5,157.5 196.7,132.6 232.9,121.7 269.1,108.8 305.3,102.2 341.5,82.2 377.6,71.9 413.8,55.7 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,126.5 124.4,157.2 160.5,145.7 196.7,123.6 232.9,118.1 269.1,105.3 305.3,93.8 341.5,74.4 377.6,65.5 413.8,45.1 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 4.26 µs | 5.96 µs | 6.41 µs | 6.44 µs |
| D38 | 5.63 ns | 4.63 µs | 7.33 µs | 6.67 µs | 5.2 µs |
| D57 | 2.81 ns | 3.38 µs | 4.92 µs | 5.62 µs | 8.75 µs |
| D76 | 3.12 ns | 4.14 µs | 5.17 µs | 6.38 µs | 8.81 µs |
| D115 | 17.4 ns | 4.42 µs | 11.1 µs | 14.1 µs | 20 µs |
| D153 | 22.4 ns | 5.56 µs | 10 µs | 19.2 µs | 30 µs |
| D230 | 48.8 ns | 10.3 µs | 11.4 µs | 39.2 µs | 58.9 µs |
| D307 | 74.6 ns | 11.8 µs | 22.8 µs | 59.1 µs | 108 µs |
| D462 | 131 ns | 15 µs | 68.5 µs | 142 µs | 231 µs |
| D616 | 131 ns | 30 µs | 128 µs | 257 µs | 426 µs |
| D924 | 164 ns | 73.1 µs | 256 µs | 573 µs | 1.13 ms |
| D1232 | 389 ns | 126 µs | 459 µs | 1.04 ms | 2.27 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,188.6 124.4,197.2 160.5,195.9 196.7,174.6 232.9,171.4 269.1,161.8 305.3,156.5 341.5,149.6 377.6,149.5 413.8,146.7 450.0,136.0 450.0,28.4 413.8,37.0 377.6,49.1 341.5,56.7 305.3,66.2 269.1,73.7 232.9,82.1 196.7,87.1 160.5,97.3 124.4,97.4 88.2,103.8 52.0,101.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,188.6 124.4,197.2 160.5,195.9 196.7,174.6 232.9,171.4 269.1,161.8 305.3,156.5 341.5,149.6 377.6,149.5 413.8,146.7 450.0,136.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.3 88.2,105.3 124.4,109.2 160.5,106.7 196.7,105.8 232.9,103.0 269.1,95.4 305.3,93.6 341.5,90.7 377.6,82.1 413.8,71.0 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.1 88.2,99.6 124.4,104.5 160.5,103.9 196.7,94.5 232.9,95.7 269.1,94.0 305.3,85.5 341.5,71.8 377.6,64.1 413.8,55.5 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,100.7 124.4,102.9 160.5,101.3 196.7,91.5 232.9,87.6 269.1,78.8 305.3,73.7 341.5,62.8 377.6,55.4 413.8,45.5 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,103.8 124.4,97.4 160.5,97.3 196.7,87.1 232.9,82.1 269.1,73.7 305.3,66.2 341.5,56.7 377.6,49.1 413.8,37.0 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.91 µs | 10.2 µs | 11 µs | 11.1 µs |
| D38 | 4.57 ns | 7.91 µs | 12.5 µs | 11.3 µs | 8.57 µs |
| D57 | 2.47 ns | 5.22 µs | 7.4 µs | 8.36 µs | 10.1 µs |
| D76 | 3.43 ns | 6.09 µs | 7.41 µs | 9.14 µs | 11.8 µs |
| D115 | 10.9 ns | 11.7 µs | 12.6 µs | 21.5 µs | 28.4 µs |
| D153 | 21.6 ns | 8.05 µs | 15.9 µs | 24.1 µs | 35.1 µs |
| D230 | 48.7 ns | 14.3 µs | 13.2 µs | 46.9 µs | 66.4 µs |
| D307 | 78.8 ns | 15.9 µs | 42.5 µs | 68 µs | 117 µs |
| D462 | 137 ns | 23.1 µs | 85.5 µs | 164 µs | 247 µs |
| D616 | 128 ns | 36.1 µs | 142 µs | 268 µs | 413 µs |
| D924 | 173 ns | 86.5 µs | 267 µs | 555 µs | 999 µs |
| D1232 | 406 ns | 142 µs | 448 µs | 911 µs | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.1 124.4,198.8 160.5,194.7 196.7,180.4 232.9,171.9 269.1,161.8 305.3,155.8 341.5,149.0 377.6,149.8 413.8,146.1 450.0,135.5 450.0,26.3 413.8,38.6 377.6,49.6 341.5,55.9 305.3,65.2 269.1,72.2 232.9,80.1 196.7,82.8 160.5,93.7 124.4,95.6 88.2,97.6 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.1 124.4,198.8 160.5,194.7 196.7,180.4 232.9,171.9 269.1,161.8 305.3,155.8 341.5,149.0 377.6,149.8 413.8,146.1 450.0,135.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,98.6 124.4,103.8 160.5,101.9 196.7,93.7 232.9,98.4 269.1,91.3 305.3,89.9 341.5,85.3 377.6,79.8 413.8,68.9 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,93.0 124.4,99.4 160.5,99.4 196.7,92.9 232.9,90.0 269.1,92.2 305.3,77.8 341.5,69.1 377.6,62.8 413.8,54.9 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,94.2 124.4,97.9 160.5,96.8 196.7,86.2 232.9,84.8 269.1,76.5 305.3,71.9 341.5,61.0 377.6,54.9 413.8,45.9 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,97.6 124.4,95.6 160.5,93.7 196.7,82.8 232.9,80.1 269.1,72.2 305.3,65.2 341.5,55.9 377.6,49.6 413.8,38.6 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.5 ns | 6.31 ns | 6.34 ns | 9.21 ns | 8.63 ns |
| D38 | 11.2 ns | 9.6 ns | 14.1 ns | 628 ns | 640 ns |
| D57 | 35.4 ns | 49.2 ns | 74.6 ns | 106 ns | 102 ns |
| D76 | 40.2 ns | 65.9 ns | 76.1 ns | 93.4 ns | 125 ns |
| D115 | 60.5 ns | 82.7 ns | 110 ns | 169 ns | 229 ns |
| D153 | 68.2 ns | 113 ns | 152 ns | 239 ns | 299 ns |
| D230 | 95.7 ns | 169 ns | 141 ns | 404 ns | 460 ns |
| D307 | 110 ns | 219 ns | 332 ns | 473 ns | 854 ns |
| D462 | 224 ns | 418 ns | 713 ns | 1.12 µs | 1.45 µs |
| D616 | 215 ns | 593 ns | 1.06 µs | 1.76 µs | 2.24 µs |
| D924 | 340 ns | 1.18 µs | 2.03 µs | 2.61 µs | 4.67 µs |
| D1232 | 529 ns | 1.92 µs | 3.82 µs | 4.14 µs | 7.03 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.4 88.2,157.6 124.4,132.6 160.5,129.8 196.7,120.9 232.9,118.3 269.1,111.0 305.3,107.9 341.5,92.5 377.6,93.4 413.8,83.4 450.0,73.8 450.0,17.7 413.8,26.5 377.6,42.4 341.5,51.9 305.3,63.4 269.1,76.9 232.9,86.2 196.7,92.0 160.5,105.1 124.4,109.5 88.2,69.7 52.0,163.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.4 88.2,157.6 124.4,132.6 160.5,129.8 196.7,120.9 232.9,118.3 269.1,111.0 305.3,107.9 341.5,92.5 377.6,93.4 413.8,83.4 450.0,73.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,160.9 124.4,125.4 160.5,119.0 196.7,114.1 232.9,107.3 269.1,98.7 305.3,92.9 341.5,78.9 377.6,71.4 413.8,56.4 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.9 88.2,152.6 124.4,116.4 160.5,115.9 196.7,108.0 232.9,100.9 269.1,102.6 305.3,83.9 341.5,67.4 377.6,58.7 413.8,44.6 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,70.1 124.4,108.6 160.5,111.5 196.7,98.6 232.9,91.1 269.1,79.7 305.3,76.3 341.5,57.6 377.6,47.7 413.8,39.2 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.2 88.2,69.7 124.4,109.5 160.5,105.1 196.7,92.0 232.9,86.2 269.1,76.9 305.3,63.4 341.5,51.9 377.6,42.4 413.8,26.5 450.0,17.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.17 ns | 4.79 µs | 6.78 µs | 7.31 µs | 7.26 µs |
| D38 | 2.11 ns | 5.26 µs | 8.23 µs | 7.37 µs | 6.14 µs |
| D57 | 2.81 ns | 3.69 µs | 4.47 µs | 5.94 µs | 8.6 µs |
| D76 | 3.12 ns | 6.05 µs | 6.69 µs | 8.24 µs | 10.9 µs |
| D115 | 17.5 ns | 6.32 µs | 13.8 µs | 17.2 µs | 23.1 µs |
| D153 | 21.7 ns | 7.35 µs | 16 µs | 23.2 µs | 33.9 µs |
| D230 | 51.4 ns | 13.4 µs | 13 µs | 46.4 µs | 65.3 µs |
| D307 | 80.8 ns | 14.8 µs | 31.8 µs | 66.4 µs | 114 µs |
| D462 | 131 ns | 21.8 µs | 83.4 µs | 162 µs | 244 µs |
| D616 | 129 ns | 34.9 µs | 138 µs | 264 µs | 409 µs |
| D924 | 173 ns | 84.3 µs | 264 µs | 554 µs | 984 µs |
| D1232 | 399 ns | 137 µs | 440 µs | 907 µs | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,200.7 124.4,197.2 160.5,195.9 196.7,174.5 232.9,171.8 269.1,161.1 305.3,155.5 341.5,149.5 377.6,149.7 413.8,146.1 450.0,135.7 450.0,26.4 413.8,38.8 377.6,49.7 341.5,56.1 305.3,65.5 269.1,72.4 232.9,80.6 196.7,85.3 160.5,94.6 124.4,97.6 88.2,101.8 52.0,99.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,200.7 124.4,197.2 160.5,195.9 196.7,174.5 232.9,171.8 269.1,161.1 305.3,155.5 341.5,149.5 377.6,149.7 413.8,146.1 450.0,135.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.8 88.2,103.7 124.4,108.1 160.5,102.0 196.7,101.4 232.9,99.5 269.1,92.1 305.3,90.9 341.5,86.1 377.6,80.2 413.8,69.3 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.5 88.2,98.1 124.4,105.7 160.5,100.7 196.7,91.8 232.9,89.9 269.1,92.5 305.3,81.4 341.5,69.4 377.6,63.2 413.8,55.1 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,99.5 124.4,102.2 160.5,98.1 196.7,89.0 232.9,85.3 269.1,76.7 305.3,72.2 341.5,61.2 377.6,55.1 413.8,45.9 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.7 88.2,101.8 124.4,97.6 160.5,94.6 196.7,85.3 232.9,80.6 269.1,72.4 305.3,65.5 341.5,56.1 377.6,49.7 413.8,38.8 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.2 ns | 39.8 ns | 42.9 ns | 40.3 ns | 39.6 ns |
| D38 | 15.4 ns | 33.3 ns | 74.5 ns | 57.4 ns | 97.5 ns |
| D57 | 16.5 ns | 40 ns | 72.9 ns | 617 ns | 712 ns |
| D76 | 17.9 ns | 73.1 ns | 692 ns | 484 ns | 1.04 µs |
| D115 | 20.5 ns | 72.4 ns | 637 ns | 1.22 µs | 1.3 µs |
| D153 | 23.1 ns | 622 ns | 931 ns | 1.31 µs | 2.14 µs |
| D230 | 29.3 ns | 684 ns | 766 ns | 2.26 µs | 2.49 µs |
| D307 | 33.4 ns | 1.08 µs | 1.81 µs | 2.52 µs | 5.52 µs |
| D462 | 75.6 ns | 1.55 µs | 3.65 µs | 6.52 µs | 9.19 µs |
| D616 | 63 ns | 2.44 µs | 6.17 µs | 10.8 µs | 15.9 µs |
| D924 | 103 ns | 3.7 µs | 11.1 µs | 23.2 µs | 28.4 µs |
| D1232 | 109 ns | 6.24 µs | 20.8 µs | 27.5 µs | 50.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,200.6 124.4,199.1 160.5,197.3 196.7,194.4 232.9,191.8 269.1,186.7 305.3,183.8 341.5,166.1 377.6,170.0 413.8,159.3 450.0,158.0 450.0,24.8 413.8,37.3 377.6,49.9 341.5,61.8 305.3,72.9 269.1,90.2 232.9,93.5 196.7,104.3 160.5,109.1 124.4,117.4 88.2,160.6 52.0,180.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,200.6 124.4,199.1 160.5,197.3 196.7,194.4 232.9,191.8 269.1,186.7 305.3,183.8 341.5,166.1 377.6,170.0 413.8,159.3 450.0,158.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,183.9 124.4,179.9 160.5,166.8 196.7,167.0 232.9,120.3 269.1,118.2 305.3,108.3 341.5,100.5 377.6,90.7 413.8,81.6 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,166.4 124.4,166.9 160.5,118.0 196.7,119.8 232.9,111.6 269.1,115.8 305.3,97.1 341.5,81.9 377.6,70.5 413.8,57.7 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,172.1 124.4,120.5 160.5,125.7 196.7,105.7 232.9,104.1 269.1,92.3 305.3,89.9 341.5,69.3 377.6,58.4 413.8,41.7 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.1 88.2,160.6 124.4,117.4 160.5,109.1 196.7,104.3 232.9,93.5 269.1,90.2 305.3,72.9 341.5,61.8 377.6,49.9 413.8,37.3 450.0,24.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 300 ns | 387 ns | 396 ns | 377 ns |
| D38 | 2.11 ns | 303 ns | 403 ns | 311 ns | 361 ns |
| D57 | 270 ns | 439 ns | 457 ns | 483 ns | 555 ns |
| D76 | 270 ns | 480 ns | 437 ns | 495 ns | 586 ns |
| D115 | 283 ns | 439 ns | 649 ns | 929 ns | 1.13 µs |
| D153 | 341 ns | 509 ns | 681 ns | 1.06 µs | 1.23 µs |
| D230 | 475 ns | 715 ns | 632 ns | 1.39 µs | 1.53 µs |
| D307 | 567 ns | 647 ns | 917 ns | 1.19 µs | 9.62 µs |
| D462 | 1.13 µs | 2.79 µs | 3.63 µs | 4.07 µs | 5.1 µs |
| D616 | 1.2 µs | 1.33 µs | 1.84 µs | 2.63 µs | 3.46 µs |
| D924 | 1.7 µs | 1.95 µs | 2.73 µs | 3.32 µs | 4.7 µs |
| D1232 | 3.06 µs | 3.07 µs | 4.01 µs | 4.58 µs | 6.24 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.7 88.2,193.8 124.4,88.4 160.5,88.4 196.7,87.4 232.9,83.4 269.1,76.2 305.3,72.3 341.5,57.3 377.6,56.1 413.8,48.5 450.0,35.7 450.0,20.2 413.8,26.4 377.6,33.0 341.5,24.6 305.3,10.8 269.1,50.8 232.9,55.5 196.7,57.4 160.5,71.6 124.4,72.8 88.2,82.1 52.0,81.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.7 88.2,193.8 124.4,88.4 160.5,88.4 196.7,87.4 232.9,83.4 269.1,76.2 305.3,72.3 341.5,57.3 377.6,56.1 413.8,48.5 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.1 88.2,86.0 124.4,77.9 160.5,75.9 196.7,77.9 232.9,74.7 269.1,67.3 305.3,69.4 341.5,37.8 377.6,53.9 413.8,45.5 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.6 88.2,79.8 124.4,77.0 160.5,78.0 196.7,69.4 232.9,68.3 269.1,70.0 305.3,61.9 341.5,32.0 377.6,46.7 413.8,38.2 450.0,29.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.1 88.2,85.4 124.4,75.8 160.5,75.3 196.7,61.6 232.9,58.7 269.1,52.8 305.3,56.2 341.5,29.5 377.6,39.0 413.8,34.0 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.2 88.2,82.1 124.4,72.8 160.5,71.6 196.7,57.4 232.9,55.5 269.1,50.8 305.3,10.8 341.5,24.6 377.6,33.0 413.8,26.4 450.0,20.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.15 µs | 9.23 µs | 12.4 µs | 13.6 µs | 13.9 µs |
| D38 | 8.15 µs | 9.63 µs | 15.6 µs | 14.2 µs | 12.5 µs |
| D57 | 4.47 µs | 4.13 µs | 4.89 µs | 4.92 µs | 4.6 µs |
| D76 | 3.92 µs | 4.79 µs | 4.33 µs | 4.01 µs | 4.81 µs |
| D115 | 8.18 µs | 8.2 µs | 9.5 µs | 9.37 µs | 10.5 µs |
| D153 | 8.26 µs | 9.08 µs | 9.68 µs | 10.7 µs | 10.5 µs |
| D230 | 10.4 µs | 12.6 µs | 8.26 µs | 15.6 µs | 14.5 µs |
| D307 | 12.9 µs | 18.2 µs | 18.6 µs | 19.7 µs | 26 µs |
| D462 | 15.8 µs | 20.5 µs | 25.3 µs | 29.6 µs | 33.6 µs |
| D616 | 23.2 µs | 39.6 µs | 55.3 µs | 60.7 µs | 71.6 µs |
| D924 | 34.1 µs | 72.6 µs | 95.7 µs | 124 µs | 162 µs |
| D1232 | 55.5 µs | 113 µs | 166 µs | 200 µs | 247 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.2 88.2,149.3 124.4,166.7 160.5,170.5 196.7,149.2 232.9,148.9 269.1,142.3 305.3,136.0 341.5,130.1 377.6,119.0 413.8,107.8 450.0,93.7 450.0,50.4 413.8,62.7 377.6,86.4 341.5,108.3 305.3,115.6 269.1,132.6 232.9,141.9 196.7,142.0 160.5,164.5 124.4,165.8 88.2,137.0 52.0,133.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.2 88.2,149.3 124.4,166.7 160.5,170.5 196.7,149.2 232.9,148.9 269.1,142.3 305.3,136.0 341.5,130.1 377.6,119.0 413.8,107.8 450.0,93.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.7 88.2,144.4 124.4,169.0 160.5,164.6 196.7,149.1 232.9,146.1 269.1,136.6 305.3,125.9 341.5,122.6 377.6,103.5 413.8,85.9 450.0,73.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.0 88.2,130.4 124.4,164.0 160.5,167.5 196.7,144.8 232.9,144.3 269.1,148.9 305.3,125.3 341.5,116.4 377.6,93.8 413.8,77.9 450.0,62.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.4 88.2,133.1 124.4,163.9 160.5,169.8 196.7,145.2 232.9,141.4 269.1,130.4 305.3,123.8 341.5,111.9 377.6,91.1 413.8,70.5 450.0,56.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.8 88.2,137.0 124.4,165.8 160.5,164.5 196.7,142.0 232.9,141.9 269.1,132.6 305.3,115.6 341.5,108.3 377.6,86.4 413.8,62.7 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 3.07 ns | 3.2 ns | 5.02 ns | 5.09 ns |
| D38 | 3.96 ns | 10.6 ns | 30.6 ns | 26.3 ns | 24.8 ns |
| D57 | 4.23 ns | 20.8 ns | 35.8 ns | 78.2 ns | 71.7 ns |
| D76 | 8.1 ns | 38 ns | 42.5 ns | 74.5 ns | 103 ns |
| D115 | 13.6 ns | 48 ns | 89.7 ns | 194 ns | 251 ns |
| D153 | 16.9 ns | 57.4 ns | 121 ns | 258 ns | 353 ns |
| D230 | 27.2 ns | 125 ns | 193 ns | 573 ns | 846 ns |
| D307 | 41.8 ns | 169 ns | 417 ns | 854 ns | 1.41 µs |
| D462 | 89.3 ns | 432 ns | 1.31 µs | 1.87 µs | 2.65 µs |
| D616 | 71.7 ns | 643 ns | 1.84 µs | 2.7 µs | 3.88 µs |
| D924 | 135 ns | 1.6 µs | 2.96 µs | 4.93 µs | 8.28 µs |
| D1232 | 198 ns | 2.4 µs | 5.03 µs | 8.09 µs | 12.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,186.1 124.4,185.0 160.5,173.7 196.7,164.7 232.9,160.9 269.1,152.6 305.3,145.2 341.5,132.0 377.6,135.8 413.8,124.8 450.0,118.1 450.0,45.6 413.8,53.3 377.6,66.4 341.5,73.1 305.3,84.1 269.1,92.9 232.9,108.1 196.7,114.0 160.5,129.5 124.4,135.8 88.2,154.2 52.0,181.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,186.1 124.4,185.0 160.5,173.7 196.7,164.7 232.9,160.9 269.1,152.6 305.3,145.2 341.5,132.0 377.6,135.8 413.8,124.8 450.0,118.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.5 88.2,168.9 124.4,157.2 160.5,146.8 196.7,142.7 232.9,139.7 269.1,126.1 305.3,120.9 341.5,104.6 377.6,97.7 413.8,81.8 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,150.6 124.4,147.9 160.5,144.9 196.7,131.9 232.9,126.7 269.1,118.6 305.3,105.2 341.5,85.3 377.6,79.4 413.8,71.2 450.0,61.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,153.2 124.4,134.3 160.5,135.1 196.7,118.5 232.9,113.5 269.1,99.7 305.3,92.7 341.5,79.1 377.6,72.8 413.8,62.3 450.0,53.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.7 88.2,154.2 124.4,135.8 160.5,129.5 196.7,114.0 232.9,108.1 269.1,92.9 305.3,84.1 341.5,73.1 377.6,66.4 413.8,53.3 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.703 ns | 0.703 ns | 0.703 ns | 0.622 ns |
| D38 | 1.45 ns | 1.12 ns | 1.45 ns | 1.12 ns | 1.16 ns |
| D57 | 1.74 ns | 1.87 ns | 1.74 ns | 1.74 ns | 1.87 ns |
| D76 | 2.09 ns | 2.17 ns | 2.09 ns | 1.79 ns | 2.5 ns |
| D115 | 3.17 ns | 2.86 ns | 3.17 ns | 3.25 ns | 3.55 ns |
| D153 | 4.22 ns | 4.22 ns | 4.61 ns | 4.61 ns | 4.29 ns |
| D230 | 5.86 ns | 6.66 ns | 3.65 ns | 7.24 ns | 5.61 ns |
| D307 | 9.51 ns | 11.1 ns | 7.72 ns | 9.7 ns | 11.1 ns |
| D462 | 15 ns | 14.9 ns | 16.7 ns | 16.7 ns | 17 ns |
| D616 | 13 ns | 20.3 ns | 21.8 ns | 20.2 ns | 20.2 ns |
| D924 | 54.9 ns | 84.6 ns | 76.4 ns | 77.7 ns | 84.6 ns |
| D1232 | 54.5 ns | 69.8 ns | 69.9 ns | 61.4 ns | 61.6 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,132.7 124.4,127.3 160.5,122.0 196.7,110.0 232.9,101.6 269.1,92.2 305.3,78.1 341.5,64.9 377.6,69.0 413.8,27.4 450.0,27.6 450.0,24.0 413.8,14.8 377.6,56.3 341.5,61.3 305.3,73.7 269.1,93.4 232.9,101.1 196.7,106.6 160.5,116.8 124.4,125.2 88.2,139.1 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,132.7 124.4,127.3 160.5,122.0 196.7,110.0 232.9,101.6 269.1,92.2 305.3,78.1 341.5,64.9 377.6,69.0 413.8,27.4 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,140.0 124.4,125.2 160.5,121.0 196.7,112.9 232.9,101.7 269.1,88.5 305.3,73.7 341.5,65.1 377.6,56.2 413.8,14.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.7 124.4,127.3 160.5,122.0 196.7,110.0 232.9,99.1 269.1,105.8 305.3,84.2 341.5,61.9 377.6,54.1 413.8,17.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,140.0 124.4,127.3 160.5,126.4 196.7,109.2 232.9,99.1 269.1,86.0 305.3,77.6 341.5,61.9 377.6,56.3 413.8,17.3 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,139.1 124.4,125.2 160.5,116.8 196.7,106.6 232.9,101.1 269.1,93.4 305.3,73.7 341.5,61.3 377.6,56.3 413.8,14.8 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 5.28 µs | 7.48 µs | 8.01 µs | 7.93 µs |
| D38 | 6.49 ns | 5.78 µs | 8.95 µs | 7.93 µs | 6.75 µs |
| D57 | 64.6 ns | 3.96 µs | 4.66 µs | 4.96 µs | 4.92 µs |
| D76 | 66.4 ns | 4.35 µs | 4.49 µs | 4.51 µs | 5.35 µs |
| D115 | 149 ns | 8.02 µs | 9.68 µs | 10 µs | 11.5 µs |
| D153 | 191 ns | 8.99 µs | 10.1 µs | 11.8 µs | 12 µs |
| D230 | 284 ns | 13.2 µs | 8.8 µs | 19.2 µs | 17.7 µs |
| D307 | 342 ns | 18.8 µs | 18.9 µs | 23.4 µs | 31.9 µs |
| D462 | 635 ns | 69.4 µs | 147 µs | 221 µs | 281 µs |
| D616 | 625 ns | 178 µs | 352 µs | 354 µs | 526 µs |
| D924 | 930 ns | 450 µs | 454 µs | 783 µs | 1.74 ms |
| D1232 | 1.47 µs | 827 µs | 857 µs | 2.11 ms | 2.7 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,186.8 124.4,158.3 160.5,157.9 196.7,147.9 232.9,144.8 269.1,139.9 305.3,137.6 341.5,129.9 377.6,130.1 413.8,125.2 450.0,119.5 450.0,26.2 413.8,31.7 377.6,46.5 341.5,54.3 305.3,81.3 269.1,88.6 232.9,93.5 196.7,93.9 160.5,103.5 124.4,104.5 88.2,100.6 52.0,98.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,186.8 124.4,158.3 160.5,157.9 196.7,147.9 232.9,144.8 269.1,139.9 305.3,137.6 341.5,129.9 377.6,130.1 413.8,125.2 450.0,119.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,102.5 124.4,107.2 160.5,106.0 196.7,98.4 232.9,97.0 269.1,92.3 305.3,87.9 341.5,71.7 377.6,60.0 413.8,48.5 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.3 88.2,97.1 124.4,105.2 160.5,105.6 196.7,96.1 232.9,95.5 269.1,97.3 305.3,87.8 341.5,62.4 377.6,51.5 413.8,48.4 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,98.6 124.4,104.4 160.5,105.6 196.7,95.7 232.9,93.7 269.1,87.6 305.3,85.2 341.5,57.3 377.6,51.5 413.8,41.6 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,100.6 124.4,104.5 160.5,103.5 196.7,93.9 232.9,93.5 269.1,88.6 305.3,81.3 341.5,54.3 377.6,46.5 413.8,31.7 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 2.11 ns | 2.11 ns | 2.46 ns | 2.18 ns |
| D38 | 8.12 ns | 6.18 ns | 14.9 ns | 15 ns | 10.6 ns |
| D57 | 8.09 ns | 7.17 ns | 8.09 ns | 8.09 ns | 7.16 ns |
| D76 | 8.54 ns | 9.52 ns | 8.71 ns | 6.76 ns | 8.72 ns |
| D115 | 14.4 ns | 12.4 ns | 14.1 ns | 12.4 ns | 14.1 ns |
| D153 | 20.7 ns | 20 ns | 20.1 ns | 20.1 ns | 16.5 ns |
| D230 | 32.3 ns | 36.3 ns | 18.5 ns | 36.8 ns | 27.9 ns |
| D307 | 37.6 ns | 40.1 ns | 30 ns | 37.9 ns | 43.1 ns |
| D462 | 86.5 ns | 82.4 ns | 93 ns | 84.5 ns | 82.8 ns |
| D616 | 67.1 ns | 81.9 ns | 95.5 ns | 77.8 ns | 77.5 ns |
| D924 | 114 ns | 123 ns | 99.8 ns | 97 ns | 109 ns |
| D1232 | 143 ns | 145 ns | 130 ns | 125 ns | 121 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,149.3 124.4,149.5 160.5,147.9 196.7,132.8 232.9,122.2 269.1,109.4 305.3,105.0 341.5,80.9 377.6,88.2 413.8,73.0 450.0,66.4 450.0,71.1 413.8,74.2 377.6,84.0 341.5,82.1 305.3,101.1 269.1,113.6 232.9,128.8 196.7,133.5 160.5,147.3 124.4,153.0 88.2,141.5 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,149.3 124.4,149.5 160.5,147.9 196.7,132.8 232.9,122.2 269.1,109.4 305.3,105.0 341.5,80.9 377.6,88.2 413.8,73.0 450.0,66.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,157.3 124.4,153.0 160.5,144.8 196.7,137.0 232.9,123.2 269.1,106.0 305.3,103.1 341.5,82.3 377.6,82.4 413.8,70.7 450.0,66.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,131.9 124.4,149.5 160.5,147.3 196.7,133.5 232.9,123.2 269.1,125.5 305.3,111.5 341.5,78.8 377.6,78.0 413.8,76.7 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,131.7 124.4,149.5 160.5,154.7 196.7,137.0 232.9,123.2 269.1,105.6 305.3,104.8 341.5,81.5 377.6,83.9 413.8,77.6 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,141.5 124.4,153.0 160.5,147.3 196.7,133.5 232.9,128.8 269.1,113.6 305.3,101.1 341.5,82.1 377.6,84.0 413.8,74.2 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 4.12 µs | 5.76 µs | 6.33 µs | 6.37 µs |
| D38 | 4.92 ns | 4.47 µs | 7.11 µs | 6.5 µs | 5.19 µs |
| D57 | 2.79 ns | 3.11 µs | 4.56 µs | 5.6 µs | 8.73 µs |
| D76 | 3.74 ns | 3.7 µs | 4.92 µs | 6.28 µs | 8.43 µs |
| D115 | 17.3 ns | 4.1 µs | 10.9 µs | 13.5 µs | 18.7 µs |
| D153 | 22.5 ns | 5.3 µs | 9.87 µs | 18.5 µs | 29.2 µs |
| D230 | 48.8 ns | 10.6 µs | 11.3 µs | 39.1 µs | 56.7 µs |
| D307 | 68.6 ns | 11.2 µs | 22.2 µs | 57.1 µs | 108 µs |
| D462 | 121 ns | 14.9 µs | 64.2 µs | 140 µs | 227 µs |
| D616 | 114 ns | 29 µs | 126 µs | 253 µs | 426 µs |
| D924 | 163 ns | 72.5 µs | 251 µs | 567 µs | 1.13 ms |
| D1232 | 396 ns | 125 µs | 452 µs | 1.03 ms | 2.26 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,190.2 124.4,197.3 160.5,193.6 196.7,174.6 232.9,171.4 269.1,161.8 305.3,157.5 341.5,150.5 377.6,151.2 413.8,146.8 450.0,135.8 450.0,28.4 413.8,37.1 377.6,49.2 341.5,57.0 305.3,66.2 269.1,74.2 232.9,82.4 196.7,88.0 160.5,97.8 124.4,97.4 88.2,103.8 52.0,101.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,190.2 124.4,197.3 160.5,193.6 196.7,174.6 232.9,171.4 269.1,161.8 305.3,157.5 341.5,150.5 377.6,151.2 413.8,146.8 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.7 88.2,105.7 124.4,110.2 160.5,108.0 196.7,106.8 232.9,103.6 269.1,95.0 305.3,94.3 341.5,90.7 377.6,82.5 413.8,71.1 450.0,64.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.6 88.2,99.9 124.4,105.5 160.5,104.5 196.7,94.6 232.9,95.9 269.1,94.2 305.3,85.8 341.5,72.6 377.6,64.3 413.8,55.7 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,101.1 124.4,102.9 160.5,101.5 196.7,92.0 232.9,88.1 269.1,78.8 305.3,74.1 341.5,62.9 377.6,55.6 413.8,45.6 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.3 88.2,103.8 124.4,97.4 160.5,97.8 196.7,88.0 232.9,82.4 269.1,74.2 305.3,66.2 341.5,57.0 377.6,49.2 413.8,37.1 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.92 µs | 10.2 µs | 11 µs | 11.1 µs |
| D38 | 4.57 ns | 7.91 µs | 12.5 µs | 11.3 µs | 8.58 µs |
| D57 | 12.2 ns | 5.28 µs | 7.47 µs | 8.41 µs | 10.2 µs |
| D76 | 10.5 ns | 6.09 µs | 7.44 µs | 9.17 µs | 11.8 µs |
| D115 | 11.3 ns | 11.9 µs | 12 µs | 21.1 µs | 27.8 µs |
| D153 | 20.7 ns | 8.12 µs | 15.9 µs | 24.2 µs | 35.1 µs |
| D230 | 49.7 ns | 14.8 µs | 13.1 µs | 47 µs | 67 µs |
| D307 | 67.8 ns | 16.2 µs | 41.8 µs | 68.1 µs | 117 µs |
| D462 | 125 ns | 23 µs | 85.5 µs | 166 µs | 247 µs |
| D616 | 125 ns | 36 µs | 142 µs | 269 µs | 413 µs |
| D924 | 167 ns | 87 µs | 268 µs | 553 µs | 1 ms |
| D1232 | 382 ns | 142 µs | 447 µs | 911 µs | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.1 124.4,179.0 160.5,180.8 196.7,180.0 232.9,172.4 269.1,161.5 305.3,157.7 341.5,150.1 377.6,150.0 413.8,146.5 450.0,136.2 450.0,26.3 413.8,38.6 377.6,49.6 341.5,55.9 305.3,65.2 269.1,72.1 232.9,80.1 196.7,83.0 160.5,93.7 124.4,95.5 88.2,97.6 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.1 124.4,179.0 160.5,180.8 196.7,180.0 232.9,172.4 269.1,161.5 305.3,157.7 341.5,150.1 377.6,150.0 413.8,146.5 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,98.6 124.4,103.6 160.5,101.9 196.7,93.6 232.9,98.3 269.1,90.8 305.3,89.8 341.5,85.4 377.6,79.8 413.8,68.9 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,93.0 124.4,99.3 160.5,99.4 196.7,93.5 232.9,89.9 269.1,92.4 305.3,78.0 341.5,69.1 377.6,62.8 413.8,54.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,94.2 124.4,97.9 160.5,96.8 196.7,86.5 232.9,84.8 269.1,76.5 305.3,71.9 341.5,60.9 377.6,54.9 413.8,45.9 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,97.6 124.4,95.5 160.5,93.7 196.7,83.0 232.9,80.1 269.1,72.1 305.3,65.2 341.5,55.9 377.6,49.6 413.8,38.6 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.92 ns | 14.4 ns | 20 ns | 30.5 ns | 28.8 ns |
| D38 | 9.03 ns | 28.3 ns | 34.8 ns | 1.27 µs | 2.04 µs |
| D57 | 176 ns | 206 ns | 432 ns | 675 ns | 764 ns |
| D76 | 192 ns | 277 ns | 762 ns | 626 ns | 1.21 µs |
| D115 | 134 ns | 660 ns | 933 ns | 1.53 µs | 1.65 µs |
| D153 | 126 ns | 1.03 µs | 1.53 µs | 1.84 µs | 2.67 µs |
| D230 | 145 ns | 1.73 µs | 1.35 µs | 3.51 µs | 3.37 µs |
| D307 | 146 ns | 2.38 µs | 3.16 µs | 3.9 µs | 6.9 µs |
| D462 | 178 ns | 3.69 µs | 6.07 µs | 9.96 µs | 11.6 µs |
| D616 | 186 ns | 5.75 µs | 11.2 µs | 14.3 µs | 20.6 µs |
| D924 | 222 ns | 11.4 µs | 17 µs | 26.1 µs | 35.3 µs |
| D1232 | 288 ns | 16.3 µs | 28.2 µs | 39.3 µs | 60.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.4 88.2,171.8 124.4,120.2 160.5,118.6 196.7,124.9 232.9,125.9 269.1,123.6 305.3,123.4 341.5,120.0 377.6,119.2 413.8,116.2 450.0,111.6 450.0,18.6 413.8,28.1 377.6,37.5 341.5,47.4 305.3,56.4 269.1,68.9 232.9,72.9 196.7,81.3 160.5,86.7 124.4,94.7 88.2,77.6 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,176.4 88.2,171.8 124.4,120.2 160.5,118.6 196.7,124.9 232.9,125.9 269.1,123.6 305.3,123.4 341.5,120.0 377.6,119.2 413.8,116.2 450.0,111.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.6 88.2,151.9 124.4,117.4 160.5,112.3 196.7,97.2 232.9,89.6 269.1,80.4 305.3,75.0 341.5,67.3 377.6,59.6 413.8,47.8 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.0 88.2,148.3 124.4,104.6 160.5,94.7 196.7,91.2 232.9,82.6 269.1,84.8 305.3,70.0 341.5,58.7 377.6,48.0 413.8,40.8 450.0,32.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.7 88.2,85.8 124.4,96.8 160.5,98.1 196.7,82.6 232.9,79.4 269.1,68.2 305.3,66.4 341.5,50.1 377.6,43.8 413.8,33.3 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,77.6 124.4,94.7 160.5,86.7 196.7,81.3 232.9,72.9 269.1,68.9 305.3,56.4 341.5,47.4 377.6,37.5 413.8,28.1 450.0,18.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.05 ns | 1.05 ns | 1.05 ns | 0.936 ns |
| D38 | 1.83 ns | 1.42 ns | 1.8 ns | 1.41 ns | 1.44 ns |
| D57 | 2.5 ns | 2.26 ns | 2.5 ns | 2.5 ns | 2.25 ns |
| D76 | 3.09 ns | 3.46 ns | 3.1 ns | 2.45 ns | 3.09 ns |
| D115 | 5.55 ns | 4.82 ns | 5.56 ns | 4.83 ns | 5.54 ns |
| D153 | 8.44 ns | 8.45 ns | 8.46 ns | 8.46 ns | 7.66 ns |
| D230 | 16.2 ns | 17.7 ns | 9.36 ns | 17.6 ns | 13.7 ns |
| D307 | 19.5 ns | 23.6 ns | 18 ns | 19.5 ns | 23.5 ns |
| D462 | 37.1 ns | 37.1 ns | 42.3 ns | 40.5 ns | 43.2 ns |
| D616 | 35.5 ns | 45.8 ns | 75 ns | 45.9 ns | 45.6 ns |
| D924 | 80 ns | 85.1 ns | 75 ns | 75.6 ns | 84.9 ns |
| D1232 | 106 ns | 106 ns | 106 ns | 95.6 ns | 99.2 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,146.9 124.4,140.1 160.5,135.5 196.7,122.8 232.9,113.7 269.1,99.6 305.3,95.5 341.5,81.6 377.6,82.5 413.8,64.9 450.0,58.7 450.0,60.2 413.8,63.6 377.6,77.1 341.5,78.2 305.3,91.4 269.1,103.2 232.9,115.8 196.7,122.8 160.5,135.5 124.4,142.3 88.2,152.1 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,146.9 124.4,140.1 160.5,135.5 196.7,122.8 232.9,113.7 269.1,99.6 305.3,95.5 341.5,81.6 377.6,82.5 413.8,64.9 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,152.4 124.4,142.3 160.5,133.0 196.7,125.8 232.9,113.6 269.1,97.7 305.3,91.3 341.5,81.5 377.6,77.0 413.8,63.5 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,147.2 124.4,140.1 160.5,135.5 196.7,122.8 232.9,113.6 269.1,111.4 305.3,97.2 341.5,78.7 377.6,66.2 413.8,66.3 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,152.5 124.4,140.1 160.5,140.6 196.7,125.8 232.9,113.6 269.1,97.7 305.3,95.5 341.5,79.7 377.6,76.9 413.8,66.1 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,152.1 124.4,142.3 160.5,135.5 196.7,122.8 232.9,115.8 269.1,103.2 305.3,91.4 341.5,78.2 377.6,77.1 413.8,63.6 450.0,60.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 8.22 µs | 10.7 µs | 11.7 µs | 11.7 µs |
| D38 | 4.92 ns | 8.34 µs | 13.2 µs | 12.1 µs | 9.68 µs |
| D57 | 3.17 ns | 4.14 µs | 6.04 µs | 7.43 µs | 8.61 µs |
| D76 | 3.74 ns | 4.87 µs | 6.48 µs | 7.98 µs | 10.5 µs |
| D115 | 17.3 ns | 5.53 µs | 12.6 µs | 16.1 µs | 23.7 µs |
| D153 | 22.2 ns | 7.11 µs | 12.4 µs | 22.5 µs | 34.2 µs |
| D230 | 47.3 ns | 13 µs | 13.4 µs | 45 µs | 64.9 µs |
| D307 | 66.6 ns | 14.8 µs | 26.5 µs | 64.3 µs | 119 µs |
| D462 | 125 ns | 18 µs | 74.2 µs | 154 µs | 250 µs |
| D616 | 118 ns | 33.8 µs | 140 µs | 278 µs | 460 µs |
| D924 | 146 ns | 81.7 µs | 276 µs | 607 µs | 1.2 ms |
| D1232 | 400 ns | 139 µs | 487 µs | 1.1 ms | 2.38 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,190.2 124.4,195.7 160.5,193.6 196.7,174.6 232.9,171.5 269.1,162.1 305.3,157.9 341.5,150.1 377.6,150.8 413.8,148.2 450.0,135.6 450.0,27.8 413.8,36.3 377.6,48.2 341.5,55.8 305.3,65.0 269.1,72.5 232.9,80.4 196.7,85.0 160.5,95.1 124.4,97.6 88.2,96.1 52.0,93.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,190.2 124.4,195.7 160.5,193.6 196.7,174.6 232.9,171.5 269.1,162.1 305.3,157.9 341.5,150.1 377.6,150.8 413.8,148.2 450.0,135.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.1 88.2,98.0 124.4,106.7 160.5,104.6 196.7,103.1 232.9,99.9 269.1,92.4 305.3,90.9 341.5,88.4 377.6,80.6 413.8,69.7 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,92.2 124.4,102.0 160.5,101.1 196.7,92.8 232.9,93.0 269.1,92.1 305.3,83.6 341.5,70.8 377.6,62.9 413.8,54.6 450.0,47.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,93.4 124.4,99.4 160.5,98.5 196.7,89.8 232.9,85.6 269.1,77.0 305.3,72.6 341.5,61.8 377.6,54.4 413.8,44.8 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,96.1 124.4,97.6 160.5,95.1 196.7,85.0 232.9,80.4 269.1,72.5 305.3,65.0 341.5,55.8 377.6,48.2 413.8,36.3 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.92 µs | 10.3 µs | 11.2 µs | 11.2 µs |
| D38 | 4.22 ns | 8.01 µs | 12.5 µs | 11.3 µs | 8.57 µs |
| D57 | 2.83 µs | 5.38 µs | 7.71 µs | 8.68 µs | 10.4 µs |
| D76 | 2.63 µs | 6.18 µs | 7.75 µs | 9.42 µs | 12.2 µs |
| D115 | 5.97 µs | 12.2 µs | 12.7 µs | 21.6 µs | 28.8 µs |
| D153 | 3.08 µs | 8.41 µs | 16.7 µs | 25 µs | 36.2 µs |
| D230 | 2.93 µs | 15 µs | 13.9 µs | 48.1 µs | 67.5 µs |
| D307 | 2.73 µs | 16.5 µs | 44.3 µs | 69.8 µs | 118 µs |
| D462 | 3.38 µs | 23.6 µs | 87 µs | 167 µs | 249 µs |
| D616 | 3.2 µs | 37.6 µs | 142 µs | 273 µs | 415 µs |
| D924 | 3.32 µs | 88.6 µs | 270 µs | 563 µs | 1 ms |
| D1232 | 4.74 µs | 143 µs | 449 µs | 919 µs | 2.69 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,111.4 160.5,112.3 196.7,102.1 232.9,110.3 269.1,111.0 305.3,111.8 341.5,109.2 377.6,109.9 413.8,109.4 450.0,105.0 450.0,26.3 413.8,38.5 377.6,49.5 341.5,55.8 305.3,65.1 269.1,72.0 232.9,79.8 196.7,82.6 160.5,93.3 124.4,95.2 88.2,97.6 52.0,94.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,111.4 160.5,112.3 196.7,102.1 232.9,110.3 269.1,111.0 305.3,111.8 341.5,109.2 377.6,109.9 413.8,109.4 450.0,105.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,98.5 124.4,103.4 160.5,101.7 196.7,93.2 232.9,97.9 269.1,90.7 305.3,89.5 341.5,85.1 377.6,79.3 413.8,68.6 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,93.0 124.4,98.9 160.5,98.9 196.7,92.8 232.9,89.4 269.1,91.6 305.3,77.3 341.5,68.9 377.6,62.8 413.8,54.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,94.2 124.4,97.5 160.5,96.5 196.7,86.1 232.9,84.4 269.1,76.2 305.3,71.6 341.5,60.8 377.6,54.7 413.8,45.7 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,97.6 124.4,95.2 160.5,93.3 196.7,82.6 232.9,79.8 269.1,72.0 305.3,65.1 341.5,55.8 377.6,49.5 413.8,38.5 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.98 ns | 3.21 µs | 3.46 µs | 3.8 µs | 3.85 µs |
| D38 | 4.57 ns | 2.69 µs | 4.3 µs | 3.94 µs | 2.45 µs |
| D57 | 197 ns | 292 ns | 330 ns | 343 ns | 388 ns |
| D76 | 177 ns | 327 ns | 308 ns | 365 ns | 420 ns |
| D115 | 380 ns | 473 ns | 649 ns | 697 ns | 843 ns |
| D153 | 386 ns | 557 ns | 705 ns | 858 ns | 902 ns |
| D230 | 509 ns | 864 ns | 545 ns | 1.37 µs | 1.53 µs |
| D307 | 706 ns | 1.14 µs | 1.35 µs | 1.7 µs | 2.56 µs |
| D462 | 882 ns | 1.3 µs | 2.3 µs | 3.2 µs | 4.19 µs |
| D616 | 887 ns | 1.75 µs | 2.95 µs | 4.07 µs | 5.56 µs |
| D924 | 1.3 µs | 2.85 µs | 4.32 µs | 6.76 µs | 11.1 µs |
| D1232 | 2.25 µs | 4.07 µs | 7.24 µs | 10.7 µs | 28.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.0 88.2,183.6 124.4,118.2 160.5,120.1 196.7,106.8 232.9,106.6 269.1,101.7 305.3,96.0 341.5,92.2 377.6,92.1 413.8,85.4 450.0,75.9 450.0,31.7 413.8,48.1 377.6,60.2 341.5,65.1 305.3,73.6 269.1,82.6 232.9,91.8 196.7,93.0 160.5,105.1 124.4,106.5 88.2,74.5 52.0,66.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.0 88.2,183.6 124.4,118.2 160.5,120.1 196.7,106.8 232.9,106.6 269.1,101.7 305.3,96.0 341.5,92.2 377.6,92.1 413.8,85.4 450.0,75.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,69.7 88.2,72.8 124.4,111.4 160.5,109.4 196.7,103.0 232.9,100.2 269.1,92.5 305.3,87.7 341.5,85.4 377.6,80.3 413.8,71.8 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.4 88.2,64.6 124.4,109.3 160.5,110.4 196.7,97.5 232.9,96.1 269.1,100.5 305.3,84.8 341.5,75.5 377.6,71.2 413.8,64.6 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.8 88.2,66.2 124.4,108.6 160.5,107.5 196.7,96.3 232.9,92.7 269.1,84.6 305.3,80.8 341.5,69.8 377.6,65.6 413.8,56.8 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.6 88.2,74.5 124.4,106.5 160.5,105.1 196.7,93.0 232.9,91.8 269.1,82.6 305.3,73.6 341.5,65.1 377.6,60.2 413.8,48.1 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 158 ns | 201 ns | 205 ns | 184 ns |
| D38 | 4.92 ns | 157 ns | 201 ns | 157 ns | 172 ns |
| D57 | 311 ns | 403 ns | 427 ns | 449 ns | 502 ns |
| D76 | 272 ns | 417 ns | 418 ns | 479 ns | 550 ns |
| D115 | 614 ns | 680 ns | 863 ns | 921 ns | 1.04 µs |
| D153 | 630 ns | 757 ns | 905 ns | 1.05 µs | 1.15 µs |
| D230 | 883 ns | 1.25 µs | 714 ns | 1.81 µs | 1.83 µs |
| D307 | 1.15 µs | 1.64 µs | 1.81 µs | 2.16 µs | 3.12 µs |
| D462 | 1.45 µs | 1.8 µs | 2.87 µs | 3.83 µs | 4.84 µs |
| D616 | 1.43 µs | 2.31 µs | 3.76 µs | 4.77 µs | 6.33 µs |
| D924 | 2.05 µs | 3.79 µs | 5.29 µs | 7.79 µs | 12.2 µs |
| D1232 | 3.48 µs | 5.25 µs | 8.54 µs | 12.3 µs | 30.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,110.3 160.5,112.6 196.7,98.5 232.9,98.0 269.1,92.2 305.3,87.6 341.5,83.6 377.6,83.8 413.8,77.5 450.0,68.4 450.0,30.8 413.8,46.5 377.6,58.0 341.5,62.6 305.3,70.2 269.1,79.5 232.9,87.5 196.7,89.4 160.5,100.4 124.4,102.0 88.2,120.6 52.0,119.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,110.3 160.5,112.6 196.7,98.5 232.9,98.0 269.1,92.2 305.3,87.6 341.5,83.6 377.6,83.8 413.8,77.5 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,122.2 124.4,105.8 160.5,105.2 196.7,96.7 232.9,94.8 269.1,86.2 305.3,81.4 341.5,79.8 377.6,75.5 413.8,66.9 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.8 88.2,117.8 124.4,104.8 160.5,105.2 196.7,92.6 232.9,91.7 269.1,95.8 305.3,79.7 341.5,71.7 377.6,67.0 413.8,61.1 450.0,52.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.6 88.2,122.1 124.4,103.9 160.5,102.8 196.7,91.4 232.9,89.2 269.1,79.7 305.3,76.6 341.5,66.7 377.6,62.9 413.8,54.3 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,120.6 124.4,102.0 160.5,100.4 196.7,89.4 232.9,87.5 269.1,79.5 305.3,70.2 341.5,62.6 377.6,58.0 413.8,46.5 450.0,30.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body -->
