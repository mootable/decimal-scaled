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
| D18 | 7.27 ns | 21.8 µs | 27.7 µs | 30.3 µs | 39.5 µs |
| D38 | 4.62 µs | 7.65 µs | 6.69 µs | 11.6 µs | 15.1 µs |
| D57 | 5.03 µs | 8.43 µs | 11 µs | 14.7 µs | 17.5 µs |
| D76 | 5.19 µs | 5.65 µs | 16 µs | 17.7 µs | 22.1 µs |
| D115 | 5.22 µs | 12.1 µs | 23.9 µs | 33.5 µs | 36.8 µs |
| D153 | 4.37 µs | 13.2 µs | 26.9 µs | 41.5 µs | 64.6 µs |
| D230 | 5.27 µs | 23.9 µs | 40.7 µs | 73.6 µs | 129 µs |
| D307 | 5.18 µs | 29.4 µs | 59.7 µs | 121 µs | 189 µs |
| D462 | 5.12 µs | 41.2 µs | 122 µs | 250 µs | 372 µs |
| D616 | 4.52 µs | 65.4 µs | 217 µs | 422 µs | 615 µs |
| D924 | 4.93 µs | 131 µs | 443 µs | 934 µs | 1.52 ms |
| D1232 | 5.3 µs | 202 µs | 649 µs | 1.29 ms | 3.22 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.4 88.2,105.3 124.4,104.2 160.5,103.9 196.7,103.8 232.9,106.0 269.1,103.7 305.3,103.9 341.5,104.0 377.6,105.6 413.8,104.5 450.0,103.6 450.0,24.1 413.8,33.4 377.6,44.6 341.5,50.8 305.3,59.3 269.1,64.0 232.9,72.6 196.7,79.6 160.5,85.9 124.4,88.8 88.2,90.6 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.4 88.2,105.3 124.4,104.2 160.5,103.9 196.7,103.8 232.9,106.0 269.1,103.7 305.3,103.9 341.5,104.0 377.6,105.6 413.8,104.5 450.0,103.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,99.0 124.4,97.8 160.5,102.8 196.7,93.4 232.9,92.2 269.1,84.9 305.3,82.3 341.5,78.1 377.6,72.4 413.8,63.8 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,100.7 124.4,94.5 160.5,89.9 196.7,84.9 232.9,83.4 269.1,78.3 305.3,73.5 341.5,64.6 377.6,57.5 413.8,48.7 450.0,43.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.8 124.4,90.9 160.5,88.6 196.7,80.7 232.9,78.1 269.1,71.0 305.3,64.8 341.5,55.8 377.6,49.3 413.8,39.4 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,90.6 124.4,88.8 160.5,85.9 196.7,79.6 232.9,72.6 269.1,64.0 305.3,59.3 341.5,50.8 377.6,44.6 413.8,33.4 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 27.4 µs | 29.2 µs | 33.2 µs | 36.4 µs | 38.4 µs |
| D38 | 27.4 µs | 35.5 µs | 21.3 µs | 53.7 µs | 61.4 µs |
| D57 | 3.74 µs | 4.33 µs | 5 µs | 6.12 µs | 7.43 µs |
| D76 | 3.69 µs | 2.48 µs | 6.33 µs | 7.72 µs | 10.1 µs |
| D115 | 6.71 µs | 9.33 µs | 12.4 µs | 18.2 µs | 19 µs |
| D153 | 5.55 µs | 8.98 µs | 14.5 µs | 21.3 µs | 33 µs |
| D230 | 8.76 µs | 16 µs | 26.6 µs | 46.3 µs | 73.5 µs |
| D307 | 12.8 µs | 27.9 µs | 47.3 µs | 82.9 µs | 139 µs |
| D462 | 12.5 µs | 37.4 µs | 85.4 µs | 167 µs | 260 µs |
| D616 | 18.7 µs | 75.9 µs | 181 µs | 313 µs | 485 µs |
| D924 | 31.7 µs | 164 µs | 401 µs | 799 µs | 1.34 ms |
| D1232 | 41.5 µs | 258 µs | 669 µs | 1.2 ms | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,138.1 88.2,138.1 124.4,181.4 160.5,181.6 196.7,168.7 232.9,172.8 269.1,162.9 305.3,154.6 341.5,155.2 377.6,146.5 413.8,134.9 450.0,129.1 450.0,37.3 413.8,53.6 377.6,75.7 341.5,89.2 305.3,102.9 269.1,116.7 232.9,134.0 196.7,146.1 160.5,159.9 124.4,166.4 88.2,120.6 52.0,130.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,138.1 88.2,138.1 124.4,181.4 160.5,181.6 196.7,168.7 232.9,172.8 269.1,162.9 305.3,154.6 341.5,155.2 377.6,146.5 413.8,134.9 450.0,129.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,136.8 88.2,132.5 124.4,178.2 160.5,190.3 196.7,161.5 232.9,162.3 269.1,149.8 305.3,137.7 341.5,131.3 377.6,116.0 413.8,99.3 450.0,89.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.9 88.2,143.6 124.4,175.1 160.5,169.9 196.7,155.3 232.9,151.9 269.1,138.7 305.3,126.3 341.5,113.4 377.6,97.1 413.8,79.8 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,123.5 124.4,170.7 160.5,165.6 196.7,147.0 232.9,143.5 269.1,126.7 305.3,114.1 341.5,98.9 377.6,85.2 413.8,64.9 450.0,56.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.8 88.2,120.6 124.4,166.4 160.5,159.9 196.7,146.1 232.9,134.0 269.1,116.7 305.3,102.9 341.5,89.2 377.6,75.7 413.8,53.6 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 0.937 ns | 0.936 ns | 0.935 ns | 1.05 ns |
| D38 | 1.82 ns | 1.82 ns | 1.44 ns | 1.62 ns | 1.62 ns |
| D57 | 2.5 ns | 2.25 ns | 2.25 ns | 2.25 ns | 2.25 ns |
| D76 | 3.49 ns | 1.49 ns | 3.48 ns | 3.1 ns | 3.48 ns |
| D115 | 5 ns | 4.99 ns | 4.4 ns | 4.99 ns | 3.88 ns |
| D153 | 4.48 ns | 4.47 ns | 5.94 ns | 5.95 ns | 6.62 ns |
| D230 | 15.3 ns | 13.9 ns | 13.9 ns | 15.4 ns | 15.4 ns |
| D307 | 19.6 ns | 19.6 ns | 18.5 ns | 18.5 ns | 19.6 ns |
| D462 | 29.6 ns | 33 ns | 41.3 ns | 32.7 ns | 29.2 ns |
| D616 | 50.8 ns | 71.5 ns | 60 ns | 45.3 ns | 33.4 ns |
| D924 | 74.3 ns | 89.6 ns | 84.7 ns | 84.8 ns | 74.3 ns |
| D1232 | 95.1 ns | 95.5 ns | 95.6 ns | 87.9 ns | 100 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,147.0 124.4,140.1 160.5,132.9 196.7,125.0 232.9,127.4 269.1,100.7 305.3,95.4 341.5,86.4 377.6,74.7 413.8,66.4 450.0,61.1 450.0,60.0 413.8,66.4 377.6,83.8 341.5,86.7 305.3,95.4 269.1,100.6 232.9,118.9 196.7,130.6 160.5,132.9 124.4,142.4 88.2,149.5 52.0,158.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,147.0 124.4,140.1 160.5,132.9 196.7,125.0 232.9,127.4 269.1,100.7 305.3,95.4 341.5,86.4 377.6,74.7 413.8,66.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.4 88.2,147.0 124.4,142.4 160.5,151.4 196.7,125.1 232.9,127.5 269.1,102.9 305.3,95.4 341.5,84.1 377.6,67.3 413.8,62.4 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,152.1 124.4,142.4 160.5,132.9 196.7,127.8 232.9,121.3 269.1,102.9 305.3,96.6 341.5,79.2 377.6,71.1 413.8,63.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.6 124.4,142.4 160.5,135.4 196.7,125.1 232.9,121.3 269.1,100.6 305.3,96.6 341.5,84.3 377.6,77.2 413.8,63.6 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.5 124.4,142.4 160.5,132.9 196.7,130.6 232.9,118.9 269.1,100.6 305.3,95.4 341.5,86.7 377.6,83.8 413.8,66.4 450.0,60.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 21.8 µs | 27.7 µs | 30.3 µs | 39.5 µs |
| D38 | 4.57 µs | 7.67 µs | 6.66 µs | 11.6 µs | 15 µs |
| D57 | 5.04 µs | 8.35 µs | 11 µs | 14.7 µs | 17.5 µs |
| D76 | 5.19 µs | 5.66 µs | 16 µs | 17.6 µs | 22 µs |
| D115 | 5.23 µs | 12 µs | 23.9 µs | 33.3 µs | 36.5 µs |
| D153 | 4.24 µs | 13.2 µs | 26.6 µs | 40.2 µs | 63.9 µs |
| D230 | 5.22 µs | 23.8 µs | 40.9 µs | 73.5 µs | 130 µs |
| D307 | 5.16 µs | 29.4 µs | 59.5 µs | 121 µs | 190 µs |
| D462 | 4.81 µs | 41 µs | 122 µs | 250 µs | 373 µs |
| D616 | 4.46 µs | 65 µs | 214 µs | 422 µs | 614 µs |
| D924 | 4.87 µs | 132 µs | 448 µs | 935 µs | 1.52 ms |
| D1232 | 5.25 µs | 202 µs | 649 µs | 1.28 ms | 3.22 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,105.4 124.4,104.2 160.5,103.8 196.7,103.8 232.9,106.4 269.1,103.8 305.3,103.9 341.5,104.8 377.6,105.7 413.8,104.6 450.0,103.7 450.0,24.1 413.8,33.3 377.6,44.6 341.5,50.8 305.3,59.2 269.1,63.9 232.9,72.7 196.7,79.6 160.5,85.9 124.4,88.8 88.2,90.7 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,105.4 124.4,104.2 160.5,103.8 196.7,103.8 232.9,106.4 269.1,103.8 305.3,103.9 341.5,104.8 377.6,105.7 413.8,104.6 450.0,103.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,99.0 124.4,98.0 160.5,102.8 196.7,93.5 232.9,92.3 269.1,85.0 305.3,82.3 341.5,78.2 377.6,72.5 413.8,63.7 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,100.8 124.4,94.6 160.5,89.9 196.7,84.9 232.9,83.6 269.1,78.2 305.3,73.6 341.5,64.7 377.6,57.7 413.8,48.5 450.0,43.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.8 124.4,90.9 160.5,88.7 196.7,80.8 232.9,78.4 269.1,71.0 305.3,64.8 341.5,55.8 377.6,49.3 413.8,39.4 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,90.7 124.4,88.8 160.5,85.9 196.7,79.6 232.9,72.7 269.1,63.9 305.3,59.2 341.5,50.8 377.6,44.6 413.8,33.3 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 21.5 µs | 27.1 µs | 29.7 µs | 38.7 µs |
| D38 | 4.22 ns | 29 µs | 16.2 µs | 40 µs | 51.2 µs |
| D57 | 2.11 ns | 5.31 µs | 7.3 µs | 8.84 µs | 11.6 µs |
| D76 | 2.22 ns | 3.49 µs | 9.48 µs | 11.8 µs | 15.2 µs |
| D115 | 12.4 ns | 13.9 µs | 18.8 µs | 27.5 µs | 29.7 µs |
| D153 | 14.6 ns | 13.7 µs | 22.1 µs | 33.2 µs | 47.9 µs |
| D230 | 32.2 ns | 24.4 µs | 41.5 µs | 67.5 µs | 102 µs |
| D307 | 52.6 ns | 39.1 µs | 66.3 µs | 113 µs | 172 µs |
| D462 | 69.6 ns | 54.3 µs | 118 µs | 208 µs | 315 µs |
| D616 | 74.2 ns | 115 µs | 242 µs | 404 µs | 572 µs |
| D924 | 102 ns | 253 µs | 533 µs | 953 µs | 1.47 ms |
| D1232 | 142 ns | 368 µs | 862 µs | 1.39 ms | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,200.7 160.5,200.1 196.7,178.7 232.9,176.8 269.1,166.9 305.3,160.8 341.5,157.3 377.6,156.6 413.8,152.6 450.0,148.5 450.0,25.6 413.8,33.8 377.6,45.5 341.5,52.9 305.3,60.4 269.1,66.9 232.9,76.3 196.7,82.2 160.5,90.6 124.4,93.8 88.2,75.4 52.0,78.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,200.7 160.5,200.1 196.7,178.7 232.9,176.8 269.1,166.9 305.3,160.8 341.5,157.3 377.6,156.6 413.8,152.6 450.0,148.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.2 88.2,82.5 124.4,103.6 160.5,108.8 196.7,91.6 232.9,91.8 269.1,84.7 305.3,78.8 341.5,74.7 377.6,65.4 413.8,55.6 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.3 88.2,89.8 124.4,99.6 160.5,96.4 196.7,87.9 232.9,85.9 269.1,78.1 305.3,72.3 341.5,65.1 377.6,56.2 413.8,46.4 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,78.5 124.4,97.2 160.5,93.6 196.7,83.2 232.9,80.8 269.1,72.0 305.3,65.7 341.5,58.1 377.6,49.8 413.8,39.2 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.9 88.2,75.4 124.4,93.8 160.5,90.6 196.7,82.2 232.9,76.3 269.1,66.9 305.3,60.4 341.5,52.9 377.6,45.5 413.8,33.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.59 µs | 2.57 µs | 2.82 µs | 3.2 µs |
| D38 | 6.23 µs | 9.07 µs | 5.52 µs | 10 µs | 13.1 µs |
| D57 | 4.16 µs | 7.13 µs | 9.47 µs | 13.1 µs | 5.06 µs |
| D76 | 4.26 µs | 4.85 µs | 14 µs | 15.6 µs | 19.5 µs |
| D115 | 4.23 µs | 10.3 µs | 21 µs | 30 µs | 33.1 µs |
| D153 | 3.5 µs | 11.6 µs | 21.2 µs | 37.6 µs | 60.1 µs |
| D230 | 4.36 µs | 21.2 µs | 37 µs | 67.7 µs | 121 µs |
| D307 | 4.44 µs | 25.7 µs | 48.4 µs | 112 µs | 177 µs |
| D462 | 3.08 µs | 32.9 µs | 103 µs | 219 µs | 318 µs |
| D616 | 3.69 µs | 58.9 µs | 200 µs | 394 µs | 582 µs |
| D924 | 4.07 µs | 122 µs | 420 µs | 888 µs | 1.44 ms |
| D1232 | 4.37 µs | 187 µs | 615 µs | 1.23 ms | 3.09 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,101.6 124.4,106.6 160.5,106.3 196.7,106.4 232.9,108.7 269.1,106.0 305.3,105.8 341.5,110.3 377.6,108.1 413.8,106.9 450.0,106.0 450.0,24.6 413.8,34.0 377.6,45.3 341.5,52.8 305.3,60.1 269.1,64.8 232.9,73.5 196.7,80.9 160.5,87.4 124.4,104.2 88.2,92.4 52.0,109.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,101.6 124.4,106.6 160.5,106.3 196.7,106.4 232.9,108.7 269.1,106.0 305.3,105.8 341.5,110.3 377.6,108.1 413.8,106.9 450.0,106.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.5 88.2,96.9 124.4,99.9 160.5,104.7 196.7,95.3 232.9,93.9 269.1,86.4 305.3,84.0 341.5,80.9 377.6,73.7 413.8,64.7 450.0,59.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,103.1 124.4,96.4 160.5,91.5 196.7,86.5 232.9,86.4 269.1,79.5 305.3,76.1 341.5,66.8 377.6,58.6 413.8,49.3 450.0,44.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,95.7 124.4,92.3 160.5,90.2 196.7,82.1 232.9,79.3 269.1,72.0 305.3,65.7 341.5,57.4 377.6,50.1 413.8,40.1 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,92.4 124.4,104.2 160.5,87.4 196.7,80.9 232.9,73.5 269.1,64.8 305.3,60.1 341.5,52.8 377.6,45.3 413.8,34.0 450.0,24.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.36 µs | 8.83 µs | 9.69 µs | 11.3 µs |
| D38 | 4.22 ns | 9.41 µs | 6.46 µs | 13.1 µs | 15 µs |
| D57 | 615 ns | 5.47 µs | 6.96 µs | 8.54 µs | 10.9 µs |
| D76 | 609 ns | 3.26 µs | 8.6 µs | 11.1 µs | 14.9 µs |
| D115 | 1.23 µs | 13 µs | 17.8 µs | 27.3 µs | 29.3 µs |
| D153 | 884 ns | 12.3 µs | 21.8 µs | 33.1 µs | 53.6 µs |
| D230 | 1.45 µs | 23.4 µs | 42 µs | 76.5 µs | 126 µs |
| D307 | 2.33 µs | 41.7 µs | 78 µs | 143 µs | 243 µs |
| D462 | 2.21 µs | 57.5 µs | 147 µs | 298 µs | 472 µs |
| D616 | 3.25 µs | 122 µs | 317 µs | 557 µs | 888 µs |
| D924 | 5.45 µs | 272 µs | 707 µs | 1.46 ms | 2.5 ms |
| D1232 | 7.5 µs | 441 µs | 1.21 ms | 2.23 ms | 5.37 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,130.3 160.5,130.4 196.7,121.7 232.9,125.8 269.1,119.6 305.3,113.8 341.5,114.4 377.6,109.7 413.8,103.2 450.0,99.3 450.0,17.7 413.8,27.2 377.6,40.0 341.5,47.9 305.3,56.1 269.1,64.3 232.9,74.9 196.7,82.4 160.5,90.8 124.4,94.6 88.2,90.7 52.0,94.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,130.3 160.5,130.4 196.7,121.7 232.9,125.8 269.1,119.6 305.3,113.8 341.5,114.4 377.6,109.7 413.8,103.2 450.0,99.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,96.5 124.4,103.2 160.5,109.6 196.7,92.5 232.9,93.2 269.1,85.2 305.3,78.0 341.5,74.0 377.6,64.7 413.8,54.7 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.3 88.2,101.1 124.4,100.2 160.5,97.6 196.7,88.5 232.9,86.0 269.1,77.9 305.3,70.2 341.5,62.4 377.6,52.8 413.8,42.9 450.0,36.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.1 88.2,92.3 124.4,97.7 160.5,94.4 196.7,83.3 232.9,80.9 269.1,70.5 305.3,62.7 341.5,53.6 377.6,45.8 413.8,33.9 450.0,28.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.2 88.2,90.7 124.4,94.6 160.5,90.8 196.7,82.4 232.9,74.9 269.1,64.3 305.3,56.1 341.5,47.9 377.6,40.0 413.8,27.2 450.0,17.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.73 µs | 5.14 µs | 5.15 µs | 7.7 µs | 8.7 µs |
| D38 | 5.73 µs | 5.8 µs | 3.63 µs | 9.67 µs | 9.64 µs |
| D57 | 347 ns | 595 ns | 717 ns | 1.14 µs | 1.14 µs |
| D76 | 534 ns | 395 ns | 1.12 µs | 1.36 µs | 1.91 µs |
| D115 | 345 ns | 2.09 µs | 2.5 µs | 3.55 µs | 4.28 µs |
| D153 | 294 ns | 2.15 µs | 3.68 µs | 5.78 µs | 7.03 µs |
| D230 | 416 ns | 4.32 µs | 8.26 µs | 10.7 µs | 14.7 µs |
| D307 | 450 ns | 5.57 µs | 11.2 µs | 16.6 µs | 22.7 µs |
| D462 | 630 ns | 9.76 µs | 24.3 µs | 35.9 µs | 47.4 µs |
| D616 | 479 ns | 15.7 µs | 40.6 µs | 58.1 µs | 69.2 µs |
| D924 | 700 ns | 31.1 µs | 89.9 µs | 133 µs | 180 µs |
| D1232 | 1.22 µs | 61 µs | 137 µs | 194 µs | 312 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,122.1 88.2,122.1 124.4,183.0 160.5,173.6 196.7,183.1 232.9,186.6 269.1,179.0 305.3,177.3 341.5,170.0 377.6,176.0 413.8,167.7 450.0,155.7 450.0,35.3 413.8,47.2 377.6,68.0 341.5,76.2 305.3,92.2 269.1,101.6 232.9,117.7 196.7,128.4 160.5,145.9 124.4,157.2 88.2,110.8 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,122.1 88.2,122.1 124.4,183.0 160.5,173.6 196.7,183.1 232.9,186.6 269.1,179.0 305.3,177.3 341.5,170.0 377.6,176.0 413.8,167.7 450.0,155.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.5 88.2,121.8 124.4,171.3 160.5,180.2 196.7,144.0 232.9,143.4 269.1,128.2 305.3,122.7 341.5,110.5 377.6,100.2 413.8,85.3 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,132.0 124.4,167.2 160.5,157.6 196.7,140.1 232.9,131.7 269.1,114.1 305.3,107.6 341.5,90.7 377.6,79.6 413.8,62.3 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,110.7 124.4,157.2 160.5,153.4 196.7,132.5 232.9,121.9 269.1,108.5 305.3,99.0 341.5,82.2 377.6,71.8 413.8,53.8 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,110.8 124.4,157.2 160.5,145.9 196.7,128.4 232.9,117.7 269.1,101.6 305.3,92.2 341.5,76.2 377.6,68.0 413.8,47.2 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 3.96 µs | 5.5 µs | 5.9 µs | 6.98 µs |
| D38 | 5.62 ns | 5.96 µs | 4.05 µs | 7.99 µs | 8.94 µs |
| D57 | 2.81 ns | 3.31 µs | 4.32 µs | 5.37 µs | 8.72 µs |
| D76 | 3.14 ns | 2.49 µs | 5.5 µs | 6.93 µs | 9.35 µs |
| D115 | 16.7 ns | 4.66 µs | 9.73 µs | 14.3 µs | 16.7 µs |
| D153 | 20 ns | 4.81 µs | 9.46 µs | 18.2 µs | 31.9 µs |
| D230 | 52.8 ns | 9.46 µs | 18 µs | 39 µs | 73.5 µs |
| D307 | 96 ns | 12.3 µs | 25.1 µs | 67.6 µs | 114 µs |
| D462 | 179 ns | 14.9 µs | 60.1 µs | 140 µs | 220 µs |
| D616 | 170 ns | 32.4 µs | 130 µs | 257 µs | 410 µs |
| D924 | 185 ns | 73.2 µs | 271 µs | 611 µs | 1.04 ms |
| D1232 | 425 ns | 120 µs | 422 µs | 887 µs | 2.27 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,188.6 124.4,197.2 160.5,195.8 196.7,175.0 232.9,172.8 269.1,160.8 305.3,153.4 341.5,145.7 377.6,146.3 413.8,145.2 450.0,134.9 450.0,28.4 413.8,38.1 377.6,49.6 341.5,57.4 305.3,65.6 269.1,71.0 232.9,81.3 196.7,89.4 160.5,96.5 124.4,97.4 88.2,97.1 52.0,100.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,188.6 124.4,197.2 160.5,195.8 196.7,175.0 232.9,172.8 269.1,160.8 305.3,153.4 341.5,145.7 377.6,146.3 413.8,145.2 450.0,134.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.2 88.2,102.1 124.4,109.4 160.5,113.0 196.7,105.2 232.9,104.8 269.1,96.4 305.3,93.1 341.5,90.7 377.6,81.1 413.8,71.0 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.1 88.2,106.9 124.4,106.1 160.5,103.1 196.7,96.1 232.9,96.4 269.1,88.4 305.3,84.3 341.5,73.5 377.6,63.8 413.8,54.8 450.0,49.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.3 88.2,98.5 124.4,103.4 160.5,100.3 196.7,91.3 232.9,88.3 269.1,78.8 305.3,72.0 341.5,62.9 377.6,55.4 413.8,44.7 450.0,40.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.2 88.2,97.1 124.4,97.4 160.5,96.5 196.7,89.4 232.9,81.3 269.1,71.0 305.3,65.6 341.5,57.4 377.6,49.6 413.8,38.1 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 7.34 µs | 9.43 µs | 10.2 µs | 11.9 µs |
| D38 | 4.57 ns | 10.2 µs | 6.62 µs | 13.5 µs | 15.2 µs |
| D57 | 2.48 ns | 5.2 µs | 7.02 µs | 7.96 µs | 10.2 µs |
| D76 | 3.17 ns | 3.51 µs | 8 µs | 10.1 µs | 12.8 µs |
| D115 | 10.9 ns | 12.5 µs | 11.4 µs | 22.7 µs | 22.5 µs |
| D153 | 20 ns | 6.93 µs | 15 µs | 22.3 µs | 38.5 µs |
| D230 | 52.6 ns | 13.7 µs | 23 µs | 47.1 µs | 84.3 µs |
| D307 | 96.9 ns | 17 µs | 49.2 µs | 78.2 µs | 124 µs |
| D462 | 180 ns | 22.9 µs | 81.4 µs | 164 µs | 230 µs |
| D616 | 172 ns | 39.3 µs | 141 µs | 269 µs | 399 µs |
| D924 | 176 ns | 86.5 µs | 288 µs | 607 µs | 909 µs |
| D1232 | 410 ns | 132 µs | 409 µs | 779 µs | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.2 124.4,198.7 160.5,195.7 196.7,180.4 232.9,172.9 269.1,160.8 305.3,153.3 341.5,145.6 377.6,146.2 413.8,145.8 450.0,135.3 450.0,26.3 413.8,39.8 377.6,50.0 341.5,56.8 305.3,64.5 269.1,69.3 232.9,79.0 196.7,85.6 160.5,92.7 124.4,95.5 88.2,90.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.2 124.4,198.7 160.5,195.7 196.7,180.4 232.9,172.9 269.1,160.8 305.3,153.3 341.5,145.6 377.6,146.2 413.8,145.8 450.0,135.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.6 88.2,95.5 124.4,103.8 160.5,108.7 196.7,93.0 232.9,100.3 269.1,91.8 305.3,89.1 341.5,85.4 377.6,78.7 413.8,68.9 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,100.8 124.4,100.1 160.5,98.5 196.7,94.1 232.9,90.7 269.1,85.4 305.3,76.0 341.5,69.7 377.6,62.8 413.8,54.0 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,92.0 124.4,98.5 160.5,95.6 196.7,85.5 232.9,85.8 269.1,76.5 305.3,70.2 341.5,61.0 377.6,54.9 413.8,44.8 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,90.5 124.4,95.5 160.5,92.7 196.7,85.6 232.9,79.0 269.1,69.3 305.3,64.5 341.5,56.8 377.6,50.0 413.8,39.8 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.51 ns | 5.92 ns | 5.7 ns | 8.54 ns | 9.33 ns |
| D38 | 11.2 ns | 12.4 ns | 15.1 ns | 726 ns | 946 ns |
| D57 | 35.3 ns | 49.2 ns | 67.8 ns | 100 ns | 102 ns |
| D76 | 40.9 ns | 30.5 ns | 85.6 ns | 104 ns | 132 ns |
| D115 | 57.1 ns | 84.4 ns | 104 ns | 187 ns | 178 ns |
| D153 | 58.6 ns | 96.6 ns | 143 ns | 222 ns | 319 ns |
| D230 | 109 ns | 153 ns | 226 ns | 403 ns | 586 ns |
| D307 | 144 ns | 240 ns | 358 ns | 558 ns | 936 ns |
| D462 | 215 ns | 419 ns | 657 ns | 1.11 µs | 1.34 µs |
| D616 | 255 ns | 657 ns | 1.05 µs | 1.78 µs | 2.24 µs |
| D924 | 379 ns | 1.2 µs | 2.25 µs | 2.77 µs | 4.48 µs |
| D1232 | 487 ns | 1.72 µs | 3.43 µs | 3.61 µs | 7.02 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.3 88.2,157.6 124.4,132.6 160.5,129.4 196.7,122.2 232.9,121.6 269.1,108.2 305.3,102.0 341.5,93.3 377.6,89.7 413.8,81.1 450.0,75.6 450.0,17.7 413.8,27.4 377.6,42.5 341.5,53.7 305.3,61.4 269.1,71.6 232.9,84.8 196.7,97.5 160.5,103.9 124.4,109.5 88.2,61.2 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.3 88.2,157.6 124.4,132.6 160.5,129.4 196.7,122.2 232.9,121.6 269.1,108.2 305.3,102.0 341.5,93.3 377.6,89.7 413.8,81.1 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,155.4 124.4,125.4 160.5,135.8 196.7,113.7 232.9,110.7 269.1,100.8 305.3,91.0 341.5,78.9 377.6,69.1 413.8,56.0 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.2 88.2,151.0 124.4,118.4 160.5,113.4 196.7,109.1 232.9,102.2 269.1,92.3 305.3,82.3 341.5,69.1 377.6,58.9 413.8,42.4 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.4 88.2,67.0 124.4,109.9 160.5,109.1 196.7,96.4 232.9,92.7 269.1,79.7 305.3,72.7 341.5,57.7 377.6,47.5 413.8,37.8 450.0,32.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,61.2 124.4,109.5 160.5,103.9 196.7,97.5 232.9,84.8 269.1,71.6 305.3,61.4 341.5,53.7 377.6,42.5 413.8,27.4 450.0,17.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.17 ns | 4.42 µs | 6.25 µs | 6.73 µs | 7.87 µs |
| D38 | 2.11 ns | 6.78 µs | 4.84 µs | 8.82 µs | 9.85 µs |
| D57 | 2.81 ns | 3.54 µs | 4.07 µs | 5.35 µs | 8.61 µs |
| D76 | 3.19 ns | 3.05 µs | 7.32 µs | 9.05 µs | 11.8 µs |
| D115 | 17 ns | 6.81 µs | 12.7 µs | 18.6 µs | 19.4 µs |
| D153 | 20.3 ns | 6.35 µs | 14.9 µs | 21.4 µs | 37.2 µs |
| D230 | 57.6 ns | 12.6 µs | 21.4 µs | 46.1 µs | 82.6 µs |
| D307 | 104 ns | 15.7 µs | 33.7 µs | 77.6 µs | 120 µs |
| D462 | 179 ns | 21.8 µs | 79.6 µs | 161 µs | 227 µs |
| D616 | 163 ns | 38.8 µs | 139 µs | 265 µs | 396 µs |
| D924 | 194 ns | 85 µs | 284 µs | 599 µs | 902 µs |
| D1232 | 394 ns | 129 µs | 405 µs | 769 µs | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,200.7 124.4,197.2 160.5,195.6 196.7,174.9 232.9,172.7 269.1,159.7 305.3,152.4 341.5,145.6 377.6,146.8 413.8,144.7 450.0,135.9 450.0,26.4 413.8,39.8 377.6,50.1 341.5,57.0 305.3,64.9 269.1,69.5 232.9,79.4 196.7,87.5 160.5,93.6 124.4,97.6 88.2,95.9 52.0,98.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,200.7 124.4,197.2 160.5,195.6 196.7,174.9 232.9,172.7 269.1,159.7 305.3,152.4 341.5,145.6 377.6,146.8 413.8,144.7 450.0,135.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.9 88.2,100.5 124.4,108.6 160.5,110.4 196.7,100.5 232.9,101.4 269.1,92.8 305.3,90.1 341.5,86.1 377.6,78.9 413.8,69.2 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.5 88.2,104.7 124.4,106.9 160.5,99.6 196.7,92.7 232.9,90.7 269.1,86.3 305.3,80.6 341.5,70.0 377.6,63.1 413.8,54.2 450.0,49.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.6 88.2,97.3 124.4,103.5 160.5,97.0 196.7,88.0 232.9,86.3 269.1,76.8 305.3,70.3 341.5,61.2 377.6,55.1 413.8,44.9 450.0,41.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.7 88.2,95.9 124.4,97.6 160.5,93.6 196.7,87.5 232.9,79.4 269.1,69.5 305.3,64.9 341.5,57.0 377.6,50.1 413.8,39.8 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.2 ns | 40.2 ns | 42.4 ns | 40.2 ns | 40.1 ns |
| D38 | 15.4 ns | 42.8 ns | 70.1 ns | 67.1 ns | 94.7 ns |
| D57 | 16.6 ns | 40 ns | 67.3 ns | 688 ns | 705 ns |
| D76 | 17.3 ns | 43.7 ns | 593 ns | 703 ns | 900 ns |
| D115 | 20.5 ns | 79.2 ns | 703 ns | 1.11 µs | 976 ns |
| D153 | 19.7 ns | 496 ns | 1.06 µs | 1.4 µs | 2 µs |
| D230 | 28.2 ns | 728 ns | 1.47 µs | 2.26 µs | 3.2 µs |
| D307 | 43.5 ns | 983 ns | 2.14 µs | 3.33 µs | 5.64 µs |
| D462 | 62.5 ns | 1.54 µs | 3.64 µs | 6.38 µs | 9.59 µs |
| D616 | 76.7 ns | 2.31 µs | 6.17 µs | 10.8 µs | 14.8 µs |
| D924 | 95.2 ns | 3.71 µs | 11.3 µs | 24.8 µs | 28.5 µs |
| D1232 | 100 ns | 6.18 µs | 19.6 µs | 21.2 µs | 50.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,200.6 124.4,199.0 160.5,198.0 196.7,194.4 232.9,195.2 269.1,187.5 305.3,178.1 341.5,170.2 377.6,165.8 413.8,161.1 450.0,159.9 450.0,24.8 413.8,37.3 377.6,51.4 341.5,60.9 305.3,72.4 269.1,84.7 232.9,95.0 196.7,110.5 160.5,112.3 124.4,117.6 88.2,161.2 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,200.6 124.4,199.0 160.5,198.0 196.7,194.4 232.9,195.2 269.1,187.5 305.3,178.1 341.5,170.2 377.6,165.8 413.8,161.1 450.0,159.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.8 88.2,178.4 124.4,179.9 160.5,178.0 196.7,165.1 232.9,125.2 269.1,116.9 305.3,110.4 341.5,100.7 377.6,91.8 413.8,81.5 450.0,70.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.6 88.2,167.7 124.4,168.6 160.5,121.3 196.7,117.7 232.9,108.8 269.1,101.7 305.3,93.5 341.5,82.0 377.6,70.5 413.8,57.4 450.0,45.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,168.7 124.4,118.1 160.5,117.6 196.7,107.8 232.9,102.7 269.1,92.3 305.3,83.9 341.5,69.8 377.6,58.4 413.8,40.3 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,161.2 124.4,117.6 160.5,112.3 196.7,110.5 232.9,95.0 269.1,84.7 305.3,72.4 341.5,60.9 377.6,51.4 413.8,37.3 450.0,24.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 277 ns | 374 ns | 369 ns | 400 ns |
| D38 | 2.11 ns | 390 ns | 356 ns | 377 ns | 372 ns |
| D57 | 268 ns | 438 ns | 439 ns | 439 ns | 558 ns |
| D76 | 305 ns | 272 ns | 480 ns | 564 ns | 645 ns |
| D115 | 302 ns | 477 ns | 560 ns | 1.02 µs | 975 ns |
| D153 | 289 ns | 431 ns | 598 ns | 934 ns | 1.38 µs |
| D230 | 554 ns | 607 ns | 961 ns | 1.34 µs | 1.72 µs |
| D307 | 728 ns | 759 ns | 1.01 µs | 1.22 µs | 10.1 µs |
| D462 | 1.17 µs | 2.79 µs | 3.28 µs | 4.09 µs | 4.73 µs |
| D616 | 1.35 µs | 1.48 µs | 1.85 µs | 2.6 µs | 2.84 µs |
| D924 | 1.81 µs | 2.01 µs | 2.87 µs | 3.57 µs | 4.26 µs |
| D1232 | 2.98 µs | 2.96 µs | 3.91 µs | 4.04 µs | 6.21 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,197.0 124.4,112.8 160.5,110.6 196.7,110.8 232.9,111.5 269.1,100.3 305.3,95.5 341.5,87.2 377.6,84.8 413.8,79.7 450.0,71.0 450.0,58.3 413.8,64.8 377.6,71.9 341.5,63.0 305.3,49.8 269.1,80.5 232.9,84.4 196.7,90.4 160.5,97.6 124.4,100.1 88.2,107.2 52.0,105.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,197.0 124.4,112.8 160.5,110.6 196.7,110.8 232.9,111.5 269.1,100.3 305.3,95.5 341.5,87.2 377.6,84.8 413.8,79.7 450.0,71.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.3 88.2,106.3 124.4,104.4 160.5,112.6 196.7,102.9 232.9,104.6 269.1,98.7 305.3,94.8 341.5,72.2 377.6,83.1 413.8,77.8 450.0,71.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,107.9 124.4,104.3 160.5,102.7 196.7,100.1 232.9,98.9 269.1,90.7 305.3,89.8 341.5,69.4 377.6,79.3 413.8,71.7 450.0,66.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,107.0 124.4,104.3 160.5,100.0 196.7,89.7 232.9,91.2 269.1,84.9 305.3,86.6 341.5,65.5 377.6,73.4 413.8,67.9 450.0,65.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,107.2 124.4,100.1 160.5,97.6 196.7,90.4 232.9,84.4 269.1,80.5 305.3,49.8 341.5,63.0 377.6,71.9 413.8,64.8 450.0,58.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.16 µs | 8.74 µs | 11.6 µs | 12.7 µs | 14.9 µs |
| D38 | 8.14 µs | 12.4 µs | 9.23 µs | 17.1 µs | 19.7 µs |
| D57 | 4.46 µs | 4.06 µs | 4.45 µs | 4.39 µs | 4.59 µs |
| D76 | 4.46 µs | 2.32 µs | 4.96 µs | 4.6 µs | 5.3 µs |
| D115 | 8.17 µs | 8.93 µs | 8.58 µs | 10.2 µs | 8.9 µs |
| D153 | 6.94 µs | 7.66 µs | 8.83 µs | 9.72 µs | 11.5 µs |
| D230 | 10.8 µs | 12 µs | 13.7 µs | 15.7 µs | 17.8 µs |
| D307 | 16.4 µs | 19.3 µs | 21.1 µs | 23.6 µs | 28.1 µs |
| D462 | 15.9 µs | 20.2 µs | 23.7 µs | 29.8 µs | 31 µs |
| D616 | 24 µs | 42.8 µs | 55.9 µs | 60.9 µs | 64.8 µs |
| D924 | 40.4 µs | 72.7 µs | 103 µs | 133 µs | 149 µs |
| D1232 | 54.4 µs | 105 µs | 153 µs | 172 µs | 248 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.2 88.2,149.3 124.4,166.7 160.5,166.7 196.7,149.2 232.9,153.9 269.1,141.1 305.3,128.9 341.5,129.8 377.6,118.0 413.8,102.9 450.0,94.3 450.0,50.4 413.8,65.1 377.6,89.2 341.5,110.6 305.3,113.4 269.1,126.7 232.9,139.3 196.7,146.7 160.5,161.7 124.4,165.9 88.2,123.7 52.0,131.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.2 88.2,149.3 124.4,166.7 160.5,166.7 196.7,149.2 232.9,153.9 269.1,141.1 305.3,128.9 341.5,129.8 377.6,118.0 413.8,102.9 450.0,94.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,147.2 88.2,137.1 124.4,169.4 160.5,185.6 196.7,146.6 232.9,151.1 269.1,138.1 305.3,124.3 341.5,123.0 377.6,101.3 413.8,85.9 450.0,75.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,139.0 88.2,145.7 124.4,166.8 160.5,163.6 196.7,147.8 232.9,146.9 269.1,134.1 305.3,121.7 341.5,118.4 377.6,93.5 413.8,75.7 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.4 88.2,127.7 124.4,167.1 160.5,165.8 196.7,142.9 232.9,144.2 269.1,130.3 305.3,118.5 341.5,111.7 377.6,91.0 413.8,68.5 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,123.7 124.4,165.9 160.5,161.7 196.7,146.7 232.9,139.3 269.1,126.7 305.3,113.4 341.5,110.6 377.6,89.2 413.8,65.1 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 3 ns | 3.1 ns | 4.95 ns | 4.94 ns |
| D38 | 3.94 ns | 13.7 ns | 26.1 ns | 25 ns | 28.5 ns |
| D57 | 4.22 ns | 20.8 ns | 32.9 ns | 70.9 ns | 71.8 ns |
| D76 | 5.64 ns | 19.2 ns | 43.6 ns | 80.2 ns | 107 ns |
| D115 | 13.6 ns | 57.4 ns | 93.5 ns | 214 ns | 196 ns |
| D153 | 14.8 ns | 46.4 ns | 112 ns | 233 ns | 396 ns |
| D230 | 27.9 ns | 122 ns | 337 ns | 573 ns | 1.04 µs |
| D307 | 55 ns | 188 ns | 459 ns | 1.06 µs | 1.45 µs |
| D462 | 73 ns | 427 ns | 1.26 µs | 1.86 µs | 2.44 µs |
| D616 | 89.7 ns | 726 ns | 1.85 µs | 2.7 µs | 3.8 µs |
| D924 | 146 ns | 1.59 µs | 3.19 µs | 5.43 µs | 7.62 µs |
| D1232 | 182 ns | 2.2 µs | 4.59 µs | 6.97 µs | 12.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,186.2 124.4,185.0 160.5,179.9 196.7,164.7 232.9,163.2 269.1,152.2 305.3,140.4 341.5,135.5 377.6,131.9 413.8,123.4 450.0,119.6 450.0,45.7 413.8,54.7 377.6,66.8 341.5,74.5 305.3,83.5 269.1,89.4 232.9,106.1 196.7,118.3 160.5,128.8 124.4,135.8 88.2,151.8 52.0,182.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,186.2 124.4,185.0 160.5,179.9 196.7,164.7 232.9,163.2 269.1,152.2 305.3,140.4 341.5,135.5 377.6,131.9 413.8,123.4 450.0,119.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.9 88.2,164.5 124.4,157.3 160.5,158.7 196.7,139.6 232.9,143.4 269.1,126.6 305.3,119.1 341.5,104.8 377.6,95.6 413.8,81.9 450.0,76.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.3 88.2,153.3 124.4,149.3 160.5,144.4 196.7,131.2 232.9,128.0 269.1,108.9 305.3,103.5 341.5,86.0 377.6,79.3 413.8,69.9 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.2 88.2,154.1 124.4,136.0 160.5,133.8 196.7,116.8 232.9,115.3 269.1,99.7 305.3,89.0 341.5,79.2 377.6,72.7 413.8,60.6 450.0,56.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.2 88.2,151.8 124.4,135.8 160.5,128.8 196.7,118.3 232.9,106.1 269.1,89.4 305.3,83.5 341.5,74.5 377.6,66.8 413.8,54.7 450.0,45.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.622 ns | 0.622 ns | 0.622 ns | 0.703 ns |
| D38 | 1.45 ns | 1.45 ns | 1.15 ns | 1.33 ns | 1.32 ns |
| D57 | 1.74 ns | 1.87 ns | 1.87 ns | 1.87 ns | 1.87 ns |
| D76 | 2.17 ns | 1.09 ns | 2.16 ns | 2.09 ns | 2.63 ns |
| D115 | 3.17 ns | 3.17 ns | 2.86 ns | 3.55 ns | 2.75 ns |
| D153 | 2.9 ns | 2.9 ns | 4.3 ns | 4.29 ns | 4.6 ns |
| D230 | 6.65 ns | 5.86 ns | 7.16 ns | 7.24 ns | 7.24 ns |
| D307 | 12.3 ns | 12.5 ns | 11.1 ns | 11.1 ns | 12.5 ns |
| D462 | 15.1 ns | 14.9 ns | 15.3 ns | 16.7 ns | 15.3 ns |
| D616 | 18.5 ns | 21.8 ns | 21.7 ns | 20.2 ns | 15 ns |
| D924 | 54.9 ns | 86 ns | 84.7 ns | 84.8 ns | 75.7 ns |
| D1232 | 47.1 ns | 61.4 ns | 61.7 ns | 51.9 ns | 61.5 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,132.7 124.4,127.3 160.5,121.0 196.7,110.0 232.9,112.5 269.1,88.5 305.3,70.8 341.5,64.8 377.6,58.9 413.8,27.3 450.0,31.8 450.0,24.1 413.8,18.1 377.6,64.9 341.5,64.3 305.3,70.2 269.1,86.0 232.9,99.1 196.7,114.0 160.5,115.3 124.4,125.2 88.2,135.2 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,132.7 124.4,127.3 160.5,121.0 196.7,110.0 232.9,112.5 269.1,88.5 305.3,70.8 341.5,64.8 377.6,58.9 413.8,27.3 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,157.1 88.2,132.6 124.4,125.2 160.5,140.7 196.7,110.0 232.9,112.5 269.1,92.2 305.3,70.2 341.5,65.1 377.6,54.1 413.8,14.4 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,139.2 124.4,125.2 160.5,121.0 196.7,112.9 232.9,101.1 269.1,86.4 305.3,73.7 341.5,64.4 377.6,54.3 413.8,14.8 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,125.2 160.5,122.0 196.7,106.6 232.9,101.1 269.1,86.0 305.3,73.7 341.5,61.9 377.6,56.2 413.8,14.8 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,125.2 160.5,115.3 196.7,114.0 232.9,99.1 269.1,86.0 305.3,70.2 341.5,64.3 377.6,64.9 413.8,18.1 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 4.87 µs | 6.88 µs | 7.38 µs | 8.6 µs |
| D38 | 6.48 ns | 7.45 µs | 5.44 µs | 9.49 µs | 10.6 µs |
| D57 | 64 ns | 4.02 µs | 4.38 µs | 4.49 µs | 4.91 µs |
| D76 | 78.3 ns | 2.26 µs | 4.91 µs | 5.01 µs | 5.86 µs |
| D115 | 148 ns | 8.58 µs | 8.75 µs | 10.9 µs | 9.53 µs |
| D153 | 158 ns | 7.8 µs | 9.42 µs | 10.8 µs | 13 µs |
| D230 | 345 ns | 12.3 µs | 14.8 µs | 19.2 µs | 22.2 µs |
| D307 | 437 ns | 20.2 µs | 21 µs | 28 µs | 34.4 µs |
| D462 | 673 ns | 69.4 µs | 136 µs | 220 µs | 262 µs |
| D616 | 727 ns | 192 µs | 350 µs | 353 µs | 508 µs |
| D924 | 901 ns | 450 µs | 485 µs | 847 µs | 1.62 ms |
| D1232 | 1.41 µs | 759 µs | 792 µs | 1.8 ms | 2.7 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,186.8 124.4,158.4 160.5,155.9 196.7,148.0 232.9,147.2 269.1,137.5 305.3,134.6 341.5,129.2 377.6,128.3 413.8,125.6 450.0,120.0 450.0,26.3 413.8,32.6 377.6,47.0 341.5,55.2 305.3,80.4 269.1,85.8 232.9,92.5 196.7,96.3 160.5,102.3 124.4,104.5 88.2,95.0 52.0,97.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,186.8 124.4,158.4 160.5,155.9 196.7,148.0 232.9,147.2 269.1,137.5 305.3,134.6 341.5,129.2 377.6,128.3 413.8,125.6 450.0,120.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.6 88.2,99.4 124.4,107.0 160.5,114.2 196.7,97.6 232.9,98.8 269.1,93.1 305.3,87.0 341.5,71.7 377.6,59.0 413.8,48.5 450.0,42.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,103.3 124.4,105.9 160.5,104.5 196.7,97.4 232.9,96.5 269.1,90.8 305.3,86.5 341.5,63.3 377.6,51.6 413.8,47.5 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.5 88.2,96.4 124.4,105.7 160.5,104.3 196.7,94.6 232.9,94.8 269.1,87.6 305.3,82.9 341.5,57.4 377.6,51.5 413.8,40.6 450.0,31.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,95.0 124.4,104.5 160.5,102.3 196.7,96.3 232.9,92.5 269.1,85.8 305.3,80.4 341.5,55.2 377.6,47.0 413.8,32.6 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 1.87 ns | 1.87 ns | 2.18 ns | 2.46 ns |
| D38 | 8.15 ns | 7.99 ns | 10.7 ns | 12.6 ns | 13.1 ns |
| D57 | 8.09 ns | 7.17 ns | 7.17 ns | 7.16 ns | 7.16 ns |
| D76 | 9.84 ns | 4.04 ns | 9.51 ns | 8.47 ns | 9.5 ns |
| D115 | 14.4 ns | 14.1 ns | 12.4 ns | 14.1 ns | 10.9 ns |
| D153 | 12.8 ns | 11.9 ns | 15.9 ns | 16.2 ns | 20.1 ns |
| D230 | 37.8 ns | 32.2 ns | 32.1 ns | 36.3 ns | 36.3 ns |
| D307 | 50.2 ns | 50 ns | 44.8 ns | 43.1 ns | 47.8 ns |
| D462 | 74.6 ns | 74.1 ns | 73.7 ns | 88.1 ns | 73.9 ns |
| D616 | 80 ns | 103 ns | 95.5 ns | 86.5 ns | 62.5 ns |
| D924 | 109 ns | 127 ns | 108 ns | 104 ns | 94.3 ns |
| D1232 | 141 ns | 125 ns | 130 ns | 111 ns | 121 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,149.2 124.4,149.5 160.5,143.8 196.7,132.7 232.9,136.2 269.1,104.9 305.3,96.6 341.5,85.1 377.6,83.1 413.8,74.2 450.0,66.8 450.0,71.2 413.8,78.4 377.6,90.3 341.5,85.4 305.3,98.0 269.1,106.0 232.9,123.2 196.7,140.8 160.5,144.8 124.4,153.0 88.2,135.5 52.0,183.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,149.2 124.4,149.5 160.5,143.8 196.7,132.7 232.9,136.2 269.1,104.9 305.3,96.6 341.5,85.1 377.6,83.1 413.8,74.2 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,191.9 88.2,149.8 124.4,153.0 160.5,169.6 196.7,133.5 232.9,138.4 269.1,109.5 305.3,96.8 341.5,85.4 377.6,75.7 413.8,69.7 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.9 88.2,141.5 124.4,153.0 160.5,144.8 196.7,137.0 232.9,129.9 269.1,109.6 305.3,99.9 341.5,85.5 377.6,78.0 413.8,74.5 450.0,69.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,136.6 124.4,153.0 160.5,148.1 196.7,133.5 232.9,129.4 269.1,106.0 305.3,101.0 341.5,80.3 377.6,80.9 413.8,75.6 450.0,73.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,135.5 124.4,153.0 160.5,144.8 196.7,140.8 232.9,123.2 269.1,106.0 305.3,98.0 341.5,85.4 377.6,90.3 413.8,78.4 450.0,71.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 3.84 µs | 5.32 µs | 5.85 µs | 6.9 µs |
| D38 | 4.92 ns | 5.76 µs | 3.9 µs | 7.8 µs | 8.88 µs |
| D57 | 2.78 ns | 3.09 µs | 4.1 µs | 5.3 µs | 8.72 µs |
| D76 | 3.87 ns | 2.32 µs | 5.26 µs | 6.89 µs | 8.95 µs |
| D115 | 16.8 ns | 4.43 µs | 9.49 µs | 13.8 µs | 16.2 µs |
| D153 | 20.3 ns | 4.57 µs | 9.3 µs | 17.4 µs | 31.3 µs |
| D230 | 52.5 ns | 9.46 µs | 18 µs | 39 µs | 71.4 µs |
| D307 | 88.5 ns | 12.2 µs | 24.3 µs | 66.1 µs | 113 µs |
| D462 | 123 ns | 14.7 µs | 59 µs | 140 µs | 215 µs |
| D616 | 161 ns | 31.9 µs | 127 µs | 253 µs | 405 µs |
| D924 | 176 ns | 72.3 µs | 269 µs | 608 µs | 1.04 ms |
| D1232 | 415 ns | 119 µs | 417 µs | 877 µs | 2.26 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,190.2 124.4,197.3 160.5,193.2 196.7,175.0 232.9,172.6 269.1,160.8 305.3,154.4 341.5,150.3 377.6,147.0 413.8,145.9 450.0,135.2 450.0,28.4 413.8,38.1 377.6,49.8 341.5,57.6 305.3,65.6 269.1,71.3 232.9,81.6 196.7,89.8 160.5,97.1 124.4,97.4 88.2,97.2 52.0,100.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,190.2 124.4,197.3 160.5,193.2 196.7,175.0 232.9,172.6 269.1,160.8 305.3,154.4 341.5,150.3 377.6,147.0 413.8,145.9 450.0,135.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.6 88.2,102.6 124.4,110.3 160.5,113.8 196.7,105.8 232.9,105.4 269.1,96.4 305.3,93.3 341.5,91.0 377.6,81.3 413.8,71.2 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.5 88.2,107.4 124.4,106.8 160.5,103.7 196.7,96.4 232.9,96.6 269.1,88.5 305.3,84.7 341.5,73.7 377.6,64.2 413.8,54.9 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.4 88.2,98.8 124.4,103.6 160.5,100.3 196.7,91.8 232.9,88.8 269.1,78.8 305.3,72.3 341.5,63.0 377.6,55.6 413.8,44.8 450.0,40.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,97.2 124.4,97.4 160.5,97.1 196.7,89.8 232.9,81.6 269.1,71.3 305.3,65.6 341.5,57.6 377.6,49.8 413.8,38.1 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.34 µs | 9.43 µs | 10.2 µs | 12 µs |
| D38 | 4.57 ns | 10.2 µs | 6.62 µs | 13.5 µs | 15.2 µs |
| D57 | 12.2 ns | 5.25 µs | 7.03 µs | 7.97 µs | 10.2 µs |
| D76 | 12.1 ns | 3.54 µs | 8.04 µs | 10.1 µs | 12.8 µs |
| D115 | 11.3 ns | 12.7 µs | 11.1 µs | 22.9 µs | 22.4 µs |
| D153 | 19.8 ns | 6.98 µs | 14.7 µs | 23 µs | 38.6 µs |
| D230 | 53.1 ns | 13.6 µs | 22.3 µs | 47.8 µs | 84.9 µs |
| D307 | 87.4 ns | 17.3 µs | 48.3 µs | 78.3 µs | 123 µs |
| D462 | 125 ns | 22.9 µs | 81.1 µs | 165 µs | 230 µs |
| D616 | 153 ns | 39.9 µs | 142 µs | 269 µs | 400 µs |
| D924 | 171 ns | 86.4 µs | 289 µs | 606 µs | 910 µs |
| D1232 | 402 ns | 132 µs | 409 µs | 780 µs | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.1 124.4,179.0 160.5,179.1 196.7,180.0 232.9,173.0 269.1,160.7 305.3,154.5 341.5,150.1 377.6,147.6 413.8,146.2 450.0,135.6 450.0,26.3 413.8,39.7 377.6,49.9 341.5,56.8 305.3,64.6 269.1,69.2 232.9,79.0 196.7,85.7 160.5,92.6 124.4,95.4 88.2,90.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.1 124.4,179.0 160.5,179.1 196.7,180.0 232.9,173.0 269.1,160.7 305.3,154.5 341.5,150.1 377.6,147.6 413.8,146.2 450.0,135.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,95.5 124.4,103.7 160.5,108.6 196.7,92.8 232.9,100.2 269.1,91.9 305.3,88.9 341.5,85.4 377.6,78.6 413.8,69.0 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,100.8 124.4,100.1 160.5,98.4 196.7,94.4 232.9,90.9 269.1,85.7 305.3,76.2 341.5,69.7 377.6,62.8 413.8,54.0 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.4 88.2,92.0 124.4,98.5 160.5,95.6 196.7,85.5 232.9,85.4 269.1,76.3 305.3,70.2 341.5,60.9 377.6,54.9 413.8,44.8 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,90.5 124.4,95.4 160.5,92.6 196.7,85.7 232.9,79.0 269.1,69.2 305.3,64.6 341.5,56.8 377.6,49.9 413.8,39.7 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.92 ns | 14.5 ns | 19.3 ns | 29.9 ns | 30.6 ns |
| D38 | 8.97 ns | 36.5 ns | 35.1 ns | 1.47 µs | 2.84 µs |
| D57 | 176 ns | 206 ns | 489 ns | 772 ns | 766 ns |
| D76 | 209 ns | 152 ns | 645 ns | 837 ns | 1.05 µs |
| D115 | 123 ns | 602 ns | 1.01 µs | 1.55 µs | 1.28 µs |
| D153 | 109 ns | 855 ns | 1.61 µs | 1.89 µs | 2.6 µs |
| D230 | 158 ns | 1.54 µs | 2.44 µs | 3.48 µs | 4.29 µs |
| D307 | 167 ns | 2.37 µs | 3.62 µs | 4.65 µs | 7.23 µs |
| D462 | 184 ns | 3.71 µs | 5.93 µs | 9.8 µs | 11.9 µs |
| D616 | 244 ns | 6.38 µs | 11.1 µs | 14.3 µs | 18.2 µs |
| D924 | 213 ns | 11.4 µs | 17.2 µs | 26.6 µs | 34.8 µs |
| D1232 | 271 ns | 15.5 µs | 27.9 µs | 31.9 µs | 60.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.4 88.2,171.9 124.4,120.2 160.5,117.2 196.7,126.4 232.9,128.5 269.1,122.0 305.3,121.1 341.5,119.4 377.6,114.5 413.8,116.9 450.0,112.7 450.0,18.6 413.8,28.3 377.6,39.6 341.5,47.0 305.3,55.6 269.1,64.7 232.9,73.4 196.7,85.8 160.5,89.2 124.4,94.6 88.2,71.8 52.0,150.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,176.4 88.2,171.9 124.4,120.2 160.5,117.2 196.7,126.4 232.9,128.5 269.1,122.0 305.3,121.1 341.5,119.4 377.6,114.5 413.8,116.9 450.0,112.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.5 88.2,147.5 124.4,117.4 160.5,122.7 196.7,98.8 232.9,92.7 269.1,82.5 305.3,75.0 341.5,67.2 377.6,57.8 413.8,47.8 450.0,42.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.6 88.2,148.2 124.4,102.4 160.5,97.6 196.7,89.8 232.9,81.8 269.1,74.5 305.3,67.7 341.5,59.1 377.6,48.2 413.8,40.5 450.0,32.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.0 88.2,83.3 124.4,94.5 160.5,93.1 196.7,82.4 232.9,78.9 269.1,68.3 305.3,63.3 341.5,50.3 377.6,43.8 413.8,33.0 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.5 88.2,71.8 124.4,94.6 160.5,89.2 196.7,85.8 232.9,73.4 269.1,64.7 305.3,55.6 341.5,47.0 377.6,39.6 413.8,28.3 450.0,18.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 0.937 ns | 0.936 ns | 0.934 ns | 1.05 ns |
| D38 | 1.82 ns | 1.82 ns | 1.44 ns | 1.61 ns | 1.61 ns |
| D57 | 2.51 ns | 2.26 ns | 2.25 ns | 2.25 ns | 2.25 ns |
| D76 | 3.45 ns | 1.59 ns | 3.45 ns | 3.09 ns | 3.46 ns |
| D115 | 5.56 ns | 5.55 ns | 4.84 ns | 5.56 ns | 4.31 ns |
| D153 | 6.37 ns | 6.37 ns | 7.66 ns | 7.65 ns | 8.46 ns |
| D230 | 17.6 ns | 16.2 ns | 16.1 ns | 17.7 ns | 17.6 ns |
| D307 | 25.1 ns | 25.2 ns | 23.5 ns | 23.4 ns | 25.2 ns |
| D462 | 37.4 ns | 41.3 ns | 49.4 ns | 42.4 ns | 37.2 ns |
| D616 | 52 ns | 69.5 ns | 61.3 ns | 46 ns | 35.5 ns |
| D924 | 75.2 ns | 86.3 ns | 84.8 ns | 84.7 ns | 74.8 ns |
| D1232 | 95.4 ns | 95.5 ns | 95.6 ns | 85.3 ns | 99.4 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,141.8 88.2,126.0 124.4,116.7 160.5,107.5 196.7,93.7 232.9,89.7 269.1,60.3 305.3,50.0 341.5,38.5 377.6,28.9 413.8,18.3 450.0,11.4 450.0,10.2 413.8,18.4 377.6,40.0 341.5,38.6 305.3,50.0 269.1,60.2 232.9,81.5 196.7,101.0 160.5,107.4 124.4,119.8 88.2,129.5 52.0,141.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,141.8 88.2,126.0 124.4,116.7 160.5,107.5 196.7,93.7 232.9,89.7 269.1,60.3 305.3,50.0 341.5,38.5 377.6,28.9 413.8,18.3 450.0,11.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.2 88.2,126.1 124.4,119.8 160.5,130.0 196.7,93.7 232.9,89.7 269.1,62.7 305.3,49.9 341.5,35.6 377.6,20.5 413.8,14.3 450.0,11.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.2 88.2,132.8 124.4,119.8 160.5,107.5 196.7,97.7 232.9,84.4 269.1,62.8 305.3,51.9 341.5,30.4 377.6,24.2 413.8,14.8 450.0,11.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.3 88.2,129.6 124.4,119.9 160.5,110.7 196.7,93.7 232.9,84.4 269.1,60.2 305.3,52.0 341.5,34.8 377.6,32.5 413.8,14.8 450.0,14.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.8 88.2,129.5 124.4,119.8 160.5,107.4 196.7,101.0 232.9,81.5 269.1,60.2 305.3,50.0 341.5,38.6 377.6,40.0 413.8,18.4 450.0,10.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 7.63 µs | 9.92 µs | 10.8 µs | 12.7 µs |
| D38 | 4.92 ns | 10.7 µs | 7.4 µs | 14.5 µs | 16.4 µs |
| D57 | 3.17 ns | 4.17 µs | 5.51 µs | 6.94 µs | 8.74 µs |
| D76 | 4.22 ns | 2.97 µs | 7.05 µs | 8.82 µs | 11.2 µs |
| D115 | 16.8 ns | 5.94 µs | 12.2 µs | 17.8 µs | 19.3 µs |
| D153 | 19.8 ns | 6.06 µs | 11.9 µs | 21 µs | 36.7 µs |
| D230 | 58.7 ns | 12.6 µs | 22.1 µs | 44.8 µs | 82.8 µs |
| D307 | 85.9 ns | 15.7 µs | 28.7 µs | 74.7 µs | 125 µs |
| D462 | 121 ns | 17.8 µs | 67.3 µs | 156 µs | 233 µs |
| D616 | 151 ns | 36.9 µs | 142 µs | 277 µs | 436 µs |
| D924 | 139 ns | 83.4 µs | 295 µs | 657 µs | 1.1 ms |
| D1232 | 400 ns | 132 µs | 453 µs | 933 µs | 2.38 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,190.2 124.4,195.7 160.5,192.1 196.7,175.0 232.9,173.0 269.1,159.5 305.3,154.7 341.5,150.5 377.6,147.7 413.8,148.8 450.0,135.7 450.0,27.8 413.8,37.4 377.6,48.9 341.5,56.6 305.3,64.4 269.1,69.5 232.9,79.6 196.7,87.5 160.5,94.3 124.4,97.4 88.2,89.6 52.0,92.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,190.2 124.4,195.7 160.5,192.1 196.7,175.0 232.9,173.0 269.1,159.5 305.3,154.7 341.5,150.5 377.6,147.7 413.8,148.8 450.0,135.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.1 88.2,94.8 124.4,106.6 160.5,110.8 196.7,102.2 232.9,101.9 269.1,92.9 305.3,90.1 341.5,88.6 377.6,79.5 413.8,69.4 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.8 88.2,99.5 124.4,103.1 160.5,100.1 196.7,93.3 232.9,93.5 269.1,85.9 305.3,82.6 341.5,72.1 377.6,62.8 413.8,53.7 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,91.1 124.4,100.2 160.5,97.3 196.7,88.6 232.9,86.5 269.1,77.1 305.3,70.8 341.5,61.6 377.6,54.5 413.8,43.8 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.8 88.2,89.6 124.4,97.4 160.5,94.3 196.7,87.5 232.9,79.6 269.1,69.5 305.3,64.4 341.5,56.6 377.6,48.9 413.8,37.4 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.36 µs | 9.57 µs | 10.4 µs | 12.1 µs |
| D38 | 4.22 ns | 10.3 µs | 6.64 µs | 13.5 µs | 15.3 µs |
| D57 | 2.83 µs | 5.33 µs | 7.33 µs | 8.24 µs | 10.5 µs |
| D76 | 2.98 µs | 3.66 µs | 8.27 µs | 10.4 µs | 13.1 µs |
| D115 | 5.88 µs | 13 µs | 11.7 µs | 23.5 µs | 23.2 µs |
| D153 | 2.59 µs | 7.25 µs | 15.4 µs | 23.3 µs | 39.7 µs |
| D230 | 3.17 µs | 14.1 µs | 22.9 µs | 48 µs | 86.1 µs |
| D307 | 3.37 µs | 17.8 µs | 50.5 µs | 79.3 µs | 125 µs |
| D462 | 3.43 µs | 23.7 µs | 82.7 µs | 166 µs | 233 µs |
| D616 | 3.1 µs | 40.3 µs | 144 µs | 271 µs | 403 µs |
| D924 | 3.75 µs | 87.8 µs | 291 µs | 615 µs | 917 µs |
| D1232 | 4.44 µs | 134 µs | 413 µs | 785 µs | 2.7 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,111.4 160.5,110.7 196.7,102.3 232.9,112.5 269.1,110.0 305.3,109.2 341.5,109.0 377.6,110.2 413.8,107.9 450.0,105.8 450.0,26.3 413.8,39.6 377.6,49.8 341.5,56.6 305.3,64.4 269.1,69.0 232.9,78.6 196.7,85.3 160.5,92.3 124.4,95.1 88.2,90.5 52.0,93.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,111.4 160.5,110.7 196.7,102.3 232.9,112.5 269.1,110.0 305.3,109.2 341.5,109.0 377.6,110.2 413.8,107.9 450.0,105.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,95.3 124.4,103.5 160.5,108.2 196.7,92.5 232.9,99.7 269.1,91.5 305.3,88.5 341.5,85.0 377.6,78.4 413.8,68.8 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.3 88.2,100.8 124.4,99.6 160.5,98.1 196.7,93.8 232.9,90.4 269.1,85.4 305.3,75.6 341.5,69.5 377.6,62.6 413.8,53.9 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,92.0 124.4,98.1 160.5,95.2 196.7,85.1 232.9,85.2 269.1,76.2 305.3,70.0 341.5,60.9 377.6,54.8 413.8,44.6 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.3 88.2,90.5 124.4,95.1 160.5,92.3 196.7,85.3 232.9,78.6 269.1,69.0 305.3,64.4 341.5,56.6 377.6,49.8 413.8,39.6 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.95 ns | 3 µs | 3.23 µs | 3.53 µs | 4.14 µs |
| D38 | 4.57 ns | 3.47 µs | 1.81 µs | 4.72 µs | 5.43 µs |
| D57 | 197 ns | 292 ns | 300 ns | 307 ns | 388 ns |
| D76 | 200 ns | 148 ns | 337 ns | 401 ns | 464 ns |
| D115 | 376 ns | 519 ns | 587 ns | 766 ns | 685 ns |
| D153 | 309 ns | 451 ns | 622 ns | 752 ns | 1.02 µs |
| D230 | 559 ns | 790 ns | 955 ns | 1.37 µs | 1.86 µs |
| D307 | 885 ns | 1.24 µs | 1.46 µs | 2.02 µs | 2.72 µs |
| D462 | 863 ns | 1.32 µs | 2.1 µs | 3.18 µs | 3.85 µs |
| D616 | 984 ns | 1.97 µs | 3.04 µs | 4.07 µs | 5.36 µs |
| D924 | 1.45 µs | 2.87 µs | 4.69 µs | 7.48 µs | 10.1 µs |
| D1232 | 2.21 µs | 3.89 µs | 6.72 µs | 9.18 µs | 28.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.1 88.2,183.6 124.4,118.2 160.5,117.9 196.7,107.0 232.9,110.4 269.1,100.1 305.3,92.1 341.5,92.6 377.6,90.3 413.8,83.5 450.0,76.2 450.0,31.7 413.8,49.9 377.6,60.8 341.5,66.6 305.3,72.6 269.1,79.3 232.9,89.6 196.7,96.6 160.5,103.4 124.4,106.4 88.2,60.6 52.0,65.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.1 88.2,183.6 124.4,118.2 160.5,117.9 196.7,107.0 232.9,110.4 269.1,100.1 305.3,92.1 341.5,92.6 377.6,90.3 413.8,83.5 450.0,76.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,70.9 88.2,68.4 124.4,111.4 160.5,123.2 196.7,101.4 232.9,103.8 269.1,94.1 305.3,86.3 341.5,85.2 377.6,78.3 413.8,71.7 450.0,66.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,69.6 88.2,79.7 124.4,110.9 160.5,108.9 196.7,99.3 232.9,98.3 269.1,90.8 305.3,83.4 341.5,77.1 377.6,70.7 413.8,63.1 450.0,56.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.1 88.2,63.1 124.4,110.5 160.5,105.9 196.7,94.6 232.9,95.0 269.1,84.5 305.3,77.8 341.5,69.9 377.6,65.6 413.8,55.0 450.0,51.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,65.3 88.2,60.6 124.4,106.4 160.5,103.4 196.7,96.6 232.9,89.6 269.1,79.3 305.3,72.6 341.5,66.6 377.6,60.8 413.8,49.9 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 142 ns | 183 ns | 185 ns | 205 ns |
| D38 | 4.92 ns | 201 ns | 171 ns | 187 ns | 187 ns |
| D57 | 311 ns | 393 ns | 406 ns | 431 ns | 499 ns |
| D76 | 310 ns | 202 ns | 453 ns | 514 ns | 606 ns |
| D115 | 618 ns | 727 ns | 805 ns | 996 ns | 852 ns |
| D153 | 483 ns | 649 ns | 862 ns | 962 ns | 1.28 µs |
| D230 | 951 ns | 1.14 µs | 1.28 µs | 1.79 µs | 2.25 µs |
| D307 | 1.47 µs | 1.76 µs | 1.98 µs | 2.57 µs | 3.4 µs |
| D462 | 1.45 µs | 1.8 µs | 2.7 µs | 3.84 µs | 4.47 µs |
| D616 | 1.61 µs | 2.5 µs | 3.83 µs | 4.79 µs | 6.07 µs |
| D924 | 2.35 µs | 3.77 µs | 5.75 µs | 8.54 µs | 11.3 µs |
| D1232 | 3.38 µs | 5.03 µs | 7.94 µs | 10.5 µs | 30.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,110.3 160.5,110.4 196.7,98.4 232.9,102.6 269.1,90.9 305.3,83.3 341.5,83.6 377.6,81.7 413.8,75.2 450.0,68.8 450.0,30.8 413.8,47.9 377.6,58.7 341.5,64.0 305.3,68.7 269.1,75.9 232.9,85.7 196.7,92.8 160.5,98.7 124.4,102.1 88.2,119.1 52.0,117.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,110.3 160.5,110.4 196.7,98.4 232.9,102.6 269.1,90.9 305.3,83.3 341.5,83.6 377.6,81.7 413.8,75.2 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.9 88.2,117.8 124.4,106.2 160.5,117.8 196.7,95.5 232.9,97.5 269.1,87.7 305.3,80.2 341.5,79.8 377.6,74.1 413.8,66.9 450.0,61.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.5 88.2,120.7 124.4,105.7 160.5,103.7 196.7,93.8 232.9,92.6 269.1,85.7 305.3,78.1 341.5,72.8 377.6,66.7 413.8,59.6 450.0,54.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.3 88.2,119.1 124.4,104.6 160.5,101.5 196.7,90.1 232.9,90.7 269.1,79.9 305.3,73.6 341.5,66.6 377.6,62.8 413.8,52.7 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.5 88.2,119.1 124.4,102.1 160.5,98.7 196.7,92.8 232.9,85.7 269.1,75.9 305.3,68.7 341.5,64.0 377.6,58.7 413.8,47.9 450.0,30.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body -->
