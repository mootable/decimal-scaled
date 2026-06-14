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
| D18 | 7.27 ns | 23.3 µs | 29.7 µs | 25.1 µs | 36.8 µs |
| D38 | 4.28 µs | 6.92 µs | 7.36 µs | 11.6 µs | 15.1 µs |
| D57 | 5.03 µs | 8.49 µs | 9.58 µs | 13.4 µs | 17.5 µs |
| D76 | 5.17 µs | 8.52 µs | 14.3 µs | 17.7 µs | 22.1 µs |
| D115 | 5.16 µs | 12.1 µs | 23.8 µs | 33.1 µs | 43.9 µs |
| D153 | 5.19 µs | 16.1 µs | 28.6 µs | 43.4 µs | 64.5 µs |
| D230 | 5.27 µs | 24.3 µs | 40.8 µs | 67.9 µs | 121 µs |
| D307 | 4.75 µs | 29.2 µs | 56.9 µs | 121 µs | 190 µs |
| D462 | 4.29 µs | 41.2 µs | 123 µs | 252 µs | 396 µs |
| D616 | 5.4 µs | 64.6 µs | 215 µs | 443 µs | 657 µs |
| D924 | 4.53 µs | 132 µs | 353 µs | 821 µs | 1.52 ms |
| D1232 | 4.53 µs | 212 µs | 705 µs | 1.5 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.4 88.2,106.2 124.4,104.2 160.5,103.9 196.7,103.9 232.9,103.8 269.1,103.7 305.3,104.9 341.5,106.2 377.6,103.4 413.8,105.5 450.0,105.5 450.0,23.1 413.8,33.4 377.6,43.8 341.5,50.1 305.3,59.2 269.1,64.8 232.9,72.6 196.7,77.4 160.5,85.9 124.4,88.8 88.2,90.6 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.4 88.2,106.2 124.4,104.2 160.5,103.9 196.7,103.9 232.9,103.8 269.1,103.7 305.3,104.9 341.5,106.2 377.6,103.4 413.8,105.5 450.0,105.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,100.3 124.4,97.7 160.5,97.7 196.7,93.4 232.9,89.8 269.1,84.7 305.3,82.4 341.5,78.1 377.6,72.6 413.8,63.7 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,99.5 124.4,96.2 160.5,91.2 196.7,84.9 232.9,82.7 269.1,78.3 305.3,74.1 341.5,64.6 377.6,57.6 413.8,51.5 450.0,42.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,84.3 88.2,93.9 124.4,92.1 160.5,88.6 196.7,80.9 232.9,77.5 269.1,71.9 305.3,64.8 341.5,55.7 377.6,48.7 413.8,41.0 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,90.6 124.4,88.8 160.5,85.9 196.7,77.4 232.9,72.6 269.1,64.8 305.3,59.2 341.5,50.1 377.6,43.8 413.8,33.4 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 27.4 µs | 31 µs | 35.5 µs | 30.2 µs | 35.9 µs |
| D38 | 25.7 µs | 33.2 µs | 41.3 µs | 53.3 µs | 61.5 µs |
| D57 | 3.7 µs | 4.28 µs | 4.23 µs | 5.59 µs | 7.48 µs |
| D76 | 3.71 µs | 4.53 µs | 5.71 µs | 7.84 µs | 10.1 µs |
| D115 | 6.61 µs | 9.33 µs | 12.2 µs | 18.1 µs | 23.3 µs |
| D153 | 6.7 µs | 10.7 µs | 15.8 µs | 23.5 µs | 33.4 µs |
| D230 | 8.93 µs | 16 µs | 27 µs | 43.1 µs | 69.8 µs |
| D307 | 12.6 µs | 27.6 µs | 42.2 µs | 83 µs | 137 µs |
| D462 | 10.4 µs | 37 µs | 85.2 µs | 167 µs | 277 µs |
| D616 | 22.3 µs | 76.1 µs | 179 µs | 333 µs | 522 µs |
| D924 | 26.5 µs | 161 µs | 317 µs | 693 µs | 1.34 ms |
| D1232 | 34.3 µs | 274 µs | 711 µs | 1.43 ms | 3.06 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,138.1 88.2,139.5 124.4,181.6 160.5,181.5 196.7,169.0 232.9,168.7 269.1,162.5 305.3,155.0 341.5,159.2 377.6,142.6 413.8,138.8 450.0,133.3 450.0,35.7 413.8,53.7 377.6,74.1 341.5,87.9 305.3,103.1 269.1,117.8 232.9,133.8 196.7,141.6 160.5,159.9 124.4,166.3 88.2,120.6 52.0,132.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,138.1 88.2,139.5 124.4,181.6 160.5,181.5 196.7,169.0 232.9,168.7 269.1,162.5 305.3,155.0 341.5,159.2 377.6,142.6 413.8,138.8 450.0,133.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,135.4 88.2,134.0 124.4,178.4 160.5,177.2 196.7,161.5 232.9,158.6 269.1,149.8 305.3,137.9 341.5,131.6 377.6,115.9 413.8,99.7 450.0,88.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.5 88.2,129.2 124.4,178.7 160.5,172.2 196.7,155.7 232.9,150.1 269.1,138.4 305.3,128.7 341.5,113.5 377.6,97.3 413.8,85.0 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.0 88.2,123.7 124.4,172.6 160.5,165.3 196.7,147.2 232.9,141.5 269.1,128.3 305.3,114.1 341.5,98.9 377.6,83.9 413.8,67.9 450.0,52.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.3 88.2,120.6 124.4,166.3 160.5,159.9 196.7,141.6 232.9,133.8 269.1,117.8 305.3,103.1 341.5,87.9 377.6,74.1 413.8,53.7 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.06 ns | 1.05 ns | 0.818 ns | 0.938 ns |
| D38 | 1.61 ns | 1.62 ns | 1.62 ns | 1.62 ns | 1.62 ns |
| D57 | 2.5 ns | 2.25 ns | 1.94 ns | 2.88 ns | 2.26 ns |
| D76 | 3.48 ns | 3.08 ns | 3.09 ns | 3.09 ns | 3.48 ns |
| D115 | 5.02 ns | 4.99 ns | 4.4 ns | 4.98 ns | 5 ns |
| D153 | 6.65 ns | 6.62 ns | 6.64 ns | 6.64 ns | 6.63 ns |
| D230 | 15.3 ns | 13.9 ns | 13.8 ns | 13.8 ns | 13.9 ns |
| D307 | 18.5 ns | 19.6 ns | 14.6 ns | 18.6 ns | 19.6 ns |
| D462 | 26.1 ns | 28.9 ns | 29.1 ns | 49 ns | 32.7 ns |
| D616 | 72.3 ns | 51 ns | 52.5 ns | 56.4 ns | 45.6 ns |
| D924 | 55.9 ns | 84.8 ns | 78.4 ns | 56 ns | 74.4 ns |
| D1232 | 93.7 ns | 107 ns | 108 ns | 95.5 ns | 108 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,149.7 124.4,140.1 160.5,132.9 196.7,125.0 232.9,118.9 269.1,100.7 305.3,96.6 341.5,89.2 377.6,67.0 413.8,72.6 450.0,61.4 450.0,58.4 413.8,66.4 377.6,77.1 341.5,84.3 305.3,95.4 269.1,102.9 232.9,118.9 196.7,125.1 160.5,132.9 124.4,142.3 88.2,149.6 52.0,161.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,149.7 124.4,140.1 160.5,132.9 196.7,125.0 232.9,118.9 269.1,100.7 305.3,96.6 341.5,89.2 377.6,67.0 413.8,72.6 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.6 124.4,142.4 160.5,135.6 196.7,125.1 232.9,119.0 269.1,102.9 305.3,95.4 341.5,86.9 377.6,74.6 413.8,63.6 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,145.6 160.5,135.5 196.7,127.8 232.9,118.9 269.1,103.0 305.3,101.9 341.5,86.8 377.6,74.0 413.8,65.3 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.4 88.2,149.5 124.4,137.0 160.5,135.5 196.7,125.1 232.9,118.9 269.1,103.0 305.3,96.6 341.5,75.5 377.6,72.4 413.8,72.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.4 88.2,149.6 124.4,142.3 160.5,132.9 196.7,125.1 232.9,118.9 269.1,102.9 305.3,95.4 341.5,84.3 377.6,77.1 413.8,66.4 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.58 ns | 23.2 µs | 29.6 µs | 25.2 µs | 36.8 µs |
| D38 | 4.23 µs | 6.94 µs | 7.32 µs | 11.5 µs | 15.1 µs |
| D57 | 4.99 µs | 8.34 µs | 9.52 µs | 13.3 µs | 17.5 µs |
| D76 | 5.14 µs | 8.57 µs | 14.3 µs | 17.7 µs | 22 µs |
| D115 | 5.11 µs | 12 µs | 23.8 µs | 33.3 µs | 43.4 µs |
| D153 | 5.13 µs | 16.2 µs | 29.6 µs | 46.8 µs | 64.1 µs |
| D230 | 5.22 µs | 23.8 µs | 41.2 µs | 68.3 µs | 121 µs |
| D307 | 4.7 µs | 28.8 µs | 56.9 µs | 121 µs | 191 µs |
| D462 | 4.33 µs | 40.7 µs | 123 µs | 252 µs | 395 µs |
| D616 | 5.33 µs | 64.5 µs | 215 µs | 442 µs | 659 µs |
| D924 | 4.39 µs | 131 µs | 353 µs | 824 µs | 1.53 ms |
| D1232 | 4.49 µs | 212 µs | 702 µs | 1.5 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,106.4 124.4,104.3 160.5,104.0 196.7,104.0 232.9,104.0 269.1,103.8 305.3,105.1 341.5,106.1 377.6,103.5 413.8,105.9 450.0,105.6 450.0,23.1 413.8,33.3 377.6,43.8 341.5,50.1 305.3,59.1 269.1,64.7 232.9,72.7 196.7,77.5 160.5,85.9 124.4,88.8 88.2,90.6 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,106.4 124.4,104.3 160.5,104.0 196.7,104.0 232.9,104.0 269.1,103.8 305.3,105.1 341.5,106.1 377.6,103.5 413.8,105.9 450.0,105.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,100.3 124.4,98.0 160.5,97.6 196.7,93.5 232.9,89.8 269.1,84.9 305.3,82.6 341.5,78.3 377.6,72.6 413.8,63.8 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,99.6 124.4,96.3 160.5,91.3 196.7,85.0 232.9,82.2 269.1,78.1 305.3,74.1 341.5,64.6 377.6,57.6 413.8,51.5 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,84.3 88.2,94.0 124.4,92.1 160.5,88.7 196.7,80.8 232.9,76.6 269.1,71.9 305.3,64.8 341.5,55.7 377.6,48.7 413.8,41.0 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,90.6 124.4,88.8 160.5,85.9 196.7,77.5 232.9,72.7 269.1,64.7 305.3,59.1 341.5,50.1 377.6,43.8 413.8,33.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 23 µs | 29 µs | 24.7 µs | 36.1 µs |
| D38 | 3.74 ns | 27.1 µs | 33.7 µs | 39.9 µs | 51.3 µs |
| D57 | 2.11 ns | 5.31 µs | 6.24 µs | 8.16 µs | 11.6 µs |
| D76 | 2.23 ns | 6.21 µs | 8.73 µs | 11.7 µs | 15.2 µs |
| D115 | 12.4 ns | 13.6 µs | 18.5 µs | 27.2 µs | 35.7 µs |
| D153 | 15.9 ns | 16 µs | 24.4 µs | 37.2 µs | 47.4 µs |
| D230 | 32.2 ns | 24.7 µs | 41.6 µs | 61.7 µs | 95.9 µs |
| D307 | 44.8 ns | 40 µs | 58.7 µs | 112 µs | 174 µs |
| D462 | 61.3 ns | 54.6 µs | 119 µs | 207 µs | 337 µs |
| D616 | 90.2 ns | 114 µs | 246 µs | 434 µs | 618 µs |
| D924 | 75.8 ns | 255 µs | 427 µs | 792 µs | 1.47 ms |
| D1232 | 130 ns | 384 µs | 920 µs | 1.61 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,200.7 160.5,200.0 196.7,178.7 232.9,175.7 269.1,166.9 305.3,162.8 341.5,158.9 377.6,154.1 413.8,156.3 450.0,149.6 450.0,24.5 413.8,33.8 377.6,44.5 341.5,52.1 305.3,60.3 269.1,67.7 232.9,76.4 196.7,79.9 160.5,90.5 124.4,93.8 88.2,75.4 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,200.7 160.5,200.0 196.7,178.7 232.9,175.7 269.1,166.9 305.3,162.8 341.5,158.9 377.6,154.1 413.8,156.3 450.0,149.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.4 88.2,83.3 124.4,103.6 160.5,101.6 196.7,91.9 232.9,89.9 269.1,84.5 305.3,78.5 341.5,74.7 377.6,65.5 413.8,55.5 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.5 88.2,80.6 124.4,101.6 160.5,97.4 196.7,88.1 232.9,84.6 269.1,78.0 305.3,73.8 341.5,65.0 377.6,56.0 413.8,49.1 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,84.5 88.2,78.5 124.4,98.2 160.5,93.7 196.7,83.3 232.9,79.4 269.1,73.1 305.3,65.7 341.5,58.1 377.6,48.9 413.8,41.5 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,75.4 124.4,93.8 160.5,90.5 196.7,79.9 232.9,76.4 269.1,67.7 305.3,60.3 341.5,52.1 377.6,44.5 413.8,33.8 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.63 µs | 2.68 µs | 2.27 µs | 3.07 µs |
| D38 | 5.4 µs | 8.08 µs | 5.96 µs | 9.88 µs | 13 µs |
| D57 | 4.16 µs | 7.03 µs | 8.25 µs | 11.8 µs | 5.01 µs |
| D76 | 4.27 µs | 7.4 µs | 12.6 µs | 15.6 µs | 19.5 µs |
| D115 | 4.26 µs | 10.3 µs | 21 µs | 29.8 µs | 39.7 µs |
| D153 | 4.27 µs | 14.1 µs | 22.3 µs | 39.3 µs | 60.2 µs |
| D230 | 4.37 µs | 21.6 µs | 37.3 µs | 62.3 µs | 113 µs |
| D307 | 3.9 µs | 25.6 µs | 46.5 µs | 112 µs | 176 µs |
| D462 | 2.56 µs | 32.8 µs | 103 µs | 218 µs | 338 µs |
| D616 | 4.43 µs | 58.9 µs | 201 µs | 417 µs | 622 µs |
| D924 | 3.64 µs | 122 µs | 334 µs | 782 µs | 1.43 ms |
| D1232 | 3.76 µs | 197 µs | 662 µs | 1.45 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,103.4 124.4,106.6 160.5,106.3 196.7,106.3 232.9,106.3 269.1,106.0 305.3,107.4 341.5,112.6 377.6,105.8 413.8,108.3 450.0,107.8 450.0,23.6 413.8,34.1 377.6,44.5 341.5,52.0 305.3,60.1 269.1,65.6 232.9,73.4 196.7,78.6 160.5,87.4 124.4,104.3 88.2,92.4 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,103.4 124.4,106.6 160.5,106.3 196.7,106.3 232.9,106.3 269.1,106.0 305.3,107.4 341.5,112.6 377.6,105.8 413.8,108.3 450.0,107.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,98.4 124.4,100.1 160.5,99.4 196.7,95.3 232.9,91.5 269.1,86.2 305.3,84.1 341.5,81.0 377.6,73.7 413.8,64.7 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,102.1 124.4,98.1 160.5,92.8 196.7,86.5 232.9,85.8 269.1,79.4 305.3,76.7 341.5,66.8 377.6,58.5 413.8,52.2 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.1 88.2,95.9 124.4,93.7 160.5,90.2 196.7,82.2 232.9,78.7 269.1,73.0 305.3,65.7 341.5,57.5 377.6,49.4 413.8,41.6 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,92.4 124.4,104.3 160.5,87.4 196.7,78.6 232.9,73.4 269.1,65.6 305.3,60.1 341.5,52.0 377.6,44.5 413.8,34.1 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.81 µs | 9.4 µs | 8.02 µs | 10.6 µs |
| D38 | 3.74 ns | 8.82 µs | 11 µs | 13 µs | 15 µs |
| D57 | 609 ns | 5.42 µs | 5.7 µs | 7.92 µs | 11 µs |
| D76 | 606 ns | 5.97 µs | 7.96 µs | 11.4 µs | 14.9 µs |
| D115 | 1.22 µs | 12.8 µs | 17.7 µs | 27.4 µs | 36.2 µs |
| D153 | 1.29 µs | 14.7 µs | 24.3 µs | 36.7 µs | 53.5 µs |
| D230 | 1.46 µs | 23.2 µs | 41.2 µs | 71.5 µs | 118 µs |
| D307 | 2.23 µs | 41.9 µs | 69.2 µs | 143 µs | 242 µs |
| D462 | 1.91 µs | 57.4 µs | 146 µs | 297 µs | 499 µs |
| D616 | 3.99 µs | 122 µs | 312 µs | 598 µs | 957 µs |
| D924 | 4.39 µs | 267 µs | 568 µs | 1.28 ms | 2.5 ms |
| D1232 | 6.13 µs | 464 µs | 1.28 ms | 2.66 ms | 5.76 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,130.4 160.5,130.5 196.7,121.8 232.9,121.1 269.1,119.6 305.3,114.3 341.5,116.3 377.6,107.1 413.8,105.9 450.0,101.8 450.0,16.8 413.8,27.2 377.6,39.1 341.5,47.2 305.3,56.2 269.1,65.1 232.9,74.9 196.7,79.8 160.5,90.8 124.4,94.5 88.2,90.7 52.0,95.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,130.4 160.5,130.5 196.7,121.8 232.9,121.1 269.1,119.6 305.3,114.3 341.5,116.3 377.6,107.1 413.8,105.9 450.0,101.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.8 88.2,97.3 124.4,103.3 160.5,102.1 196.7,92.6 232.9,90.9 269.1,85.3 305.3,77.9 341.5,74.0 377.6,64.7 413.8,55.0 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,94.5 124.4,102.7 160.5,98.5 196.7,88.7 232.9,84.7 269.1,78.1 305.3,71.7 341.5,62.4 377.6,53.0 413.8,45.6 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,92.5 124.4,98.6 160.5,94.1 196.7,83.2 232.9,79.6 269.1,71.3 305.3,62.7 341.5,53.6 377.6,44.9 413.8,35.6 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.0 88.2,90.7 124.4,94.5 160.5,90.8 196.7,79.8 232.9,74.9 269.1,65.1 305.3,56.2 341.5,47.2 377.6,39.1 413.8,27.2 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.73 µs | 5.78 µs | 5.79 µs | 6.74 µs | 7.71 µs |
| D38 | 5.07 µs | 5.14 µs | 7.71 µs | 9.67 µs | 9.64 µs |
| D57 | 346 ns | 594 ns | 505 ns | 900 ns | 1.14 µs |
| D76 | 533 ns | 791 ns | 1.19 µs | 1.38 µs | 1.9 µs |
| D115 | 334 ns | 2.09 µs | 2.43 µs | 3.57 µs | 5.31 µs |
| D153 | 355 ns | 2.6 µs | 3.64 µs | 5.96 µs | 7.03 µs |
| D230 | 409 ns | 4.45 µs | 8.11 µs | 10.2 µs | 14.2 µs |
| D307 | 563 ns | 5.59 µs | 9.52 µs | 16.5 µs | 22.5 µs |
| D462 | 449 ns | 9.58 µs | 24.2 µs | 35.7 µs | 51.7 µs |
| D616 | 543 ns | 15.8 µs | 40.5 µs | 63.7 µs | 77.6 µs |
| D924 | 395 ns | 31.2 µs | 70.5 µs | 109 µs | 180 µs |
| D1232 | 674 ns | 48.7 µs | 150 µs | 227 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,122.1 88.2,124.7 124.4,183.0 160.5,173.7 196.7,183.8 232.9,182.5 269.1,179.4 305.3,172.5 341.5,177.4 377.6,173.3 413.8,180.2 450.0,168.6 450.0,33.1 413.8,47.2 377.6,65.5 341.5,74.3 305.3,92.4 269.1,102.4 232.9,117.6 196.7,123.8 160.5,146.1 124.4,157.1 88.2,110.8 52.0,115.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,122.1 88.2,124.7 124.4,183.0 160.5,173.7 196.7,183.8 232.9,182.5 269.1,179.4 305.3,172.5 341.5,177.4 377.6,173.3 413.8,180.2 450.0,168.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,124.4 124.4,171.3 160.5,165.1 196.7,144.0 232.9,139.3 269.1,127.6 305.3,122.6 341.5,110.9 377.6,100.0 413.8,85.3 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,115.7 124.4,174.8 160.5,156.2 196.7,140.8 232.9,131.9 269.1,114.6 305.3,111.1 341.5,90.8 377.6,79.6 413.8,67.6 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.6 88.2,110.7 124.4,162.3 160.5,153.0 196.7,132.4 232.9,121.2 269.1,109.5 305.3,99.2 341.5,82.3 377.6,69.8 413.8,58.2 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.6 88.2,110.8 124.4,157.1 160.5,146.1 196.7,123.8 232.9,117.6 269.1,102.4 305.3,92.4 341.5,74.3 377.6,65.5 413.8,47.2 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 4.26 µs | 5.95 µs | 4.97 µs | 6.44 µs |
| D38 | 4.98 ns | 5.49 µs | 6.75 µs | 7.91 µs | 8.94 µs |
| D57 | 2.81 ns | 3.33 µs | 3.7 µs | 4.87 µs | 8.68 µs |
| D76 | 3.14 ns | 3.62 µs | 5.2 µs | 6.97 µs | 9.35 µs |
| D115 | 16.8 ns | 4.69 µs | 9.65 µs | 14.2 µs | 19.7 µs |
| D153 | 22.4 ns | 5.74 µs | 9.92 µs | 19.1 µs | 32 µs |
| D230 | 52.8 ns | 9.65 µs | 18.1 µs | 35.9 µs | 68.4 µs |
| D307 | 81.7 ns | 12.4 µs | 23.2 µs | 67.1 µs | 115 µs |
| D462 | 153 ns | 14.9 µs | 60.3 µs | 139 µs | 230 µs |
| D616 | 176 ns | 32.3 µs | 129 µs | 269 µs | 428 µs |
| D924 | 119 ns | 73 µs | 217 µs | 549 µs | 1.04 ms |
| D1232 | 319 ns | 126 µs | 453 µs | 1.04 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,190.1 124.4,197.2 160.5,195.8 196.7,175.0 232.9,171.4 269.1,160.8 305.3,155.4 341.5,147.6 377.6,145.9 413.8,150.7 450.0,138.5 450.0,27.5 413.8,38.1 377.6,49.1 341.5,56.8 305.3,65.4 269.1,71.8 232.9,81.3 196.7,87.3 160.5,96.6 124.4,97.5 88.2,97.1 52.0,101.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,190.1 124.4,197.2 160.5,195.8 196.7,175.0 232.9,171.4 269.1,160.8 305.3,155.4 341.5,147.6 377.6,145.9 413.8,150.7 450.0,138.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.3 88.2,103.1 124.4,109.3 160.5,108.3 196.7,105.1 232.9,102.6 269.1,96.2 305.3,93.1 341.5,90.8 377.6,81.1 413.8,71.0 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.1 88.2,100.6 124.4,108.1 160.5,103.8 196.7,96.2 232.9,95.8 269.1,88.3 305.3,85.3 341.5,73.4 377.6,64.0 413.8,57.5 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.4 88.2,98.6 124.4,104.6 160.5,100.2 196.7,91.4 232.9,87.7 269.1,79.8 305.3,72.1 341.5,63.0 377.6,54.9 413.8,46.0 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,97.1 124.4,97.5 160.5,96.6 196.7,87.3 232.9,81.3 269.1,71.8 305.3,65.4 341.5,56.8 377.6,49.1 413.8,38.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.92 µs | 10.2 µs | 8.56 µs | 11.1 µs |
| D38 | 4.05 ns | 9.46 µs | 11.5 µs | 13.4 µs | 15.3 µs |
| D57 | 2.48 ns | 5.15 µs | 5.77 µs | 7.16 µs | 10.1 µs |
| D76 | 3.17 ns | 5.51 µs | 7.38 µs | 9.99 µs | 12.9 µs |
| D115 | 10.9 ns | 12.5 µs | 11.4 µs | 23 µs | 28 µs |
| D153 | 21.6 ns | 8.07 µs | 15.6 µs | 24.4 µs | 38.4 µs |
| D230 | 52.5 ns | 13.4 µs | 22.3 µs | 42.3 µs | 78.6 µs |
| D307 | 82.3 ns | 17 µs | 42.6 µs | 78.4 µs | 123 µs |
| D462 | 153 ns | 22.8 µs | 80.8 µs | 167 µs | 247 µs |
| D616 | 177 ns | 39.9 µs | 142 µs | 288 µs | 415 µs |
| D924 | 124 ns | 86.9 µs | 228 µs | 543 µs | 911 µs |
| D1232 | 319 ns | 140 µs | 446 µs | 911 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.6 124.4,198.7 160.5,195.7 196.7,180.4 232.9,171.9 269.1,160.8 305.3,155.3 341.5,147.6 377.6,145.7 413.8,150.2 450.0,138.5 450.0,25.6 413.8,39.7 377.6,49.5 341.5,55.9 305.3,64.6 269.1,70.1 232.9,79.0 196.7,83.0 160.5,92.6 124.4,95.5 88.2,90.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.6 124.4,198.7 160.5,195.7 196.7,180.4 232.9,171.9 269.1,160.8 305.3,155.3 341.5,147.6 377.6,145.7 413.8,150.2 450.0,138.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.4 124.4,104.0 160.5,103.1 196.7,92.9 232.9,98.4 269.1,92.1 305.3,89.1 341.5,85.5 377.6,78.6 413.8,68.9 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,94.0 124.4,102.5 160.5,99.5 196.7,94.1 232.9,90.2 269.1,85.8 305.3,77.7 341.5,69.8 377.6,62.8 413.8,56.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,92.1 124.4,99.9 160.5,95.7 196.7,85.4 232.9,84.6 269.1,77.8 305.3,70.2 341.5,60.8 377.6,54.0 413.8,46.1 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,90.5 124.4,95.5 160.5,92.6 196.7,83.0 232.9,79.0 269.1,70.1 305.3,64.6 341.5,55.9 377.6,49.5 413.8,39.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.5 ns | 6.31 ns | 6.34 ns | 7.15 ns | 8.63 ns |
| D38 | 9.55 ns | 11 ns | 15.2 ns | 726 ns | 947 ns |
| D57 | 35.3 ns | 49.2 ns | 57.6 ns | 93.4 ns | 103 ns |
| D76 | 41.2 ns | 60.6 ns | 76 ns | 104 ns | 133 ns |
| D115 | 57.4 ns | 84.5 ns | 105 ns | 186 ns | 230 ns |
| D153 | 68.1 ns | 114 ns | 152 ns | 239 ns | 318 ns |
| D230 | 111 ns | 154 ns | 227 ns | 364 ns | 534 ns |
| D307 | 130 ns | 240 ns | 332 ns | 558 ns | 929 ns |
| D462 | 198 ns | 415 ns | 656 ns | 1.11 µs | 1.42 µs |
| D616 | 298 ns | 644 ns | 1.05 µs | 1.91 µs | 2.2 µs |
| D924 | 273 ns | 1.16 µs | 1.75 µs | 2.7 µs | 4.43 µs |
| D1232 | 432 ns | 1.9 µs | 3.77 µs | 4.15 µs | 7.83 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.4 88.2,161.0 124.4,132.6 160.5,129.3 196.7,122.1 232.9,118.4 269.1,107.8 305.3,104.2 341.5,95.2 377.6,86.3 413.8,88.2 450.0,78.2 450.0,15.3 413.8,27.7 377.6,42.9 341.5,52.3 305.3,61.6 269.1,73.6 232.9,84.8 196.7,91.9 160.5,103.9 124.4,109.4 88.2,61.2 52.0,163.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.4 88.2,161.0 124.4,132.6 160.5,129.3 196.7,122.1 232.9,118.4 269.1,107.8 305.3,104.2 341.5,95.2 377.6,86.3 413.8,88.2 450.0,78.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,157.9 124.4,125.4 160.5,120.9 196.7,113.7 232.9,107.2 269.1,100.7 305.3,91.0 341.5,79.1 377.6,69.5 413.8,56.8 450.0,46.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.9 88.2,150.9 124.4,122.0 160.5,116.0 196.7,108.9 232.9,100.9 269.1,92.2 305.3,83.9 341.5,69.1 377.6,58.9 413.8,47.8 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.3 88.2,67.0 124.4,111.5 160.5,109.1 196.7,96.5 232.9,91.1 269.1,81.9 305.3,72.7 341.5,57.7 377.6,46.0 413.8,38.4 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.2 88.2,61.2 124.4,109.4 160.5,103.9 196.7,91.9 232.9,84.8 269.1,73.6 305.3,61.6 341.5,52.3 377.6,42.9 413.8,27.7 450.0,15.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.17 ns | 4.81 µs | 6.78 µs | 5.68 µs | 7.26 µs |
| D38 | 1.87 ns | 6.26 µs | 7.57 µs | 8.73 µs | 9.86 µs |
| D57 | 2.81 ns | 3.61 µs | 3.46 µs | 5.01 µs | 8.67 µs |
| D76 | 3.2 ns | 5.53 µs | 6.71 µs | 8.98 µs | 11.8 µs |
| D115 | 17 ns | 6.82 µs | 12.7 µs | 18.6 µs | 23.3 µs |
| D153 | 21.7 ns | 7.37 µs | 15.7 µs | 23 µs | 37.1 µs |
| D230 | 57.6 ns | 12.6 µs | 21.5 µs | 41.1 µs | 77.1 µs |
| D307 | 84.3 ns | 15.8 µs | 31.8 µs | 77 µs | 120 µs |
| D462 | 136 ns | 21.7 µs | 78.9 µs | 162 µs | 245 µs |
| D616 | 189 ns | 37.8 µs | 139 µs | 284 µs | 409 µs |
| D924 | 128 ns | 84.6 µs | 224 µs | 537 µs | 902 µs |
| D1232 | 316 ns | 137 µs | 440 µs | 901 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,202.2 124.4,197.2 160.5,195.6 196.7,174.9 232.9,171.8 269.1,159.7 305.3,155.0 341.5,149.0 377.6,145.0 413.8,149.8 450.0,138.6 450.0,25.6 413.8,39.8 377.6,49.7 341.5,56.0 305.3,64.9 269.1,70.4 232.9,79.4 196.7,85.2 160.5,93.7 124.4,97.5 88.2,95.9 52.0,99.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,202.2 124.4,197.2 160.5,195.6 196.7,174.9 232.9,171.8 269.1,159.7 305.3,155.0 341.5,149.0 377.6,145.0 413.8,149.8 450.0,138.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.8 88.2,101.5 124.4,108.4 160.5,103.1 196.7,100.5 232.9,99.5 269.1,92.9 305.3,90.0 341.5,86.1 377.6,79.2 413.8,69.2 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.5 88.2,99.2 124.4,108.9 160.5,100.7 196.7,92.7 232.9,90.1 269.1,86.2 305.3,81.3 341.5,70.1 377.6,63.1 413.8,57.1 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.7 88.2,97.4 124.4,104.3 160.5,97.1 196.7,88.0 232.9,85.4 269.1,78.2 305.3,70.4 341.5,61.1 377.6,54.2 413.8,46.3 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.7 88.2,95.9 124.4,97.5 160.5,93.7 196.7,85.2 232.9,79.4 269.1,70.4 305.3,64.9 341.5,56.0 377.6,49.7 413.8,39.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.2 ns | 39.7 ns | 42.9 ns | 31.3 ns | 39.5 ns |
| D38 | 15.9 ns | 41.7 ns | 66.9 ns | 67.1 ns | 94.7 ns |
| D57 | 16.5 ns | 40 ns | 56.6 ns | 489 ns | 704 ns |
| D76 | 17.3 ns | 67.4 ns | 699 ns | 707 ns | 900 ns |
| D115 | 20.5 ns | 79.6 ns | 707 ns | 1.08 µs | 1.26 µs |
| D153 | 23.1 ns | 608 ns | 955 ns | 1.28 µs | 2 µs |
| D230 | 28.2 ns | 728 ns | 1.47 µs | 2.35 µs | 3.37 µs |
| D307 | 42.2 ns | 963 ns | 1.83 µs | 3.28 µs | 5.6 µs |
| D462 | 59.3 ns | 1.52 µs | 3.63 µs | 6.41 µs | 9.03 µs |
| D616 | 83 ns | 2.36 µs | 6.17 µs | 11.3 µs | 16 µs |
| D924 | 84.5 ns | 3.68 µs | 8.94 µs | 21.6 µs | 28.5 µs |
| D1232 | 96.5 ns | 6.24 µs | 20.8 µs | 27.6 µs | 50.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,199.9 124.4,199.1 160.5,198.0 196.7,194.4 232.9,191.9 269.1,187.5 305.3,178.8 341.5,171.3 377.6,164.0 413.8,163.7 450.0,160.8 450.0,24.7 413.8,37.3 377.6,49.8 341.5,62.2 305.3,72.6 269.1,83.6 232.9,95.0 196.7,104.9 160.5,112.3 124.4,117.6 88.2,161.2 52.0,180.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,199.9 124.4,199.1 160.5,198.0 196.7,194.4 232.9,191.9 269.1,187.5 305.3,178.8 341.5,171.3 377.6,164.0 413.8,163.7 450.0,160.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,179.0 124.4,179.9 160.5,168.6 196.7,165.0 232.9,120.8 269.1,116.9 305.3,110.8 341.5,100.9 377.6,91.3 413.8,81.7 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,168.7 124.4,172.4 160.5,117.8 196.7,117.5 232.9,111.0 269.1,101.7 305.3,96.9 341.5,82.0 377.6,70.5 413.8,62.4 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.2 88.2,168.7 124.4,125.5 160.5,117.5 196.7,108.3 232.9,104.7 269.1,91.5 305.3,84.2 341.5,69.7 377.6,57.4 413.8,43.3 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.1 88.2,161.2 124.4,117.6 160.5,112.3 196.7,104.9 232.9,95.0 269.1,83.6 305.3,72.6 341.5,62.2 377.6,49.8 413.8,37.3 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 301 ns | 387 ns | 307 ns | 376 ns |
| D38 | 2.18 ns | 359 ns | 369 ns | 369 ns | 374 ns |
| D57 | 269 ns | 444 ns | 359 ns | 409 ns | 555 ns |
| D76 | 276 ns | 445 ns | 433 ns | 564 ns | 657 ns |
| D115 | 311 ns | 477 ns | 565 ns | 1.03 µs | 1.05 µs |
| D153 | 337 ns | 514 ns | 686 ns | 1.08 µs | 1.37 µs |
| D230 | 550 ns | 610 ns | 988 ns | 1.21 µs | 1.63 µs |
| D307 | 647 ns | 763 ns | 892 ns | 1.22 µs | 10.1 µs |
| D462 | 1.15 µs | 2.76 µs | 3.29 µs | 4.07 µs | 5.13 µs |
| D616 | 1.5 µs | 1.59 µs | 1.8 µs | 2.82 µs | 3.38 µs |
| D924 | 1.49 µs | 1.96 µs | 2.26 µs | 2.77 µs | 4.28 µs |
| D1232 | 2.4 µs | 3.07 µs | 4.06 µs | 4.58 µs | 6.51 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,196.5 124.4,112.8 160.5,112.4 196.7,110.3 232.9,108.9 269.1,100.4 305.3,97.6 341.5,87.6 377.6,83.0 413.8,83.1 450.0,74.8 450.0,57.5 413.8,64.8 377.6,68.8 341.5,61.6 305.3,49.9 269.1,81.5 232.9,84.5 196.7,89.1 160.5,97.3 124.4,100.2 88.2,107.1 52.0,107.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,196.5 124.4,112.8 160.5,112.4 196.7,110.3 232.9,108.9 269.1,100.4 305.3,97.6 341.5,87.6 377.6,83.0 413.8,83.1 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,110.9 88.2,107.8 124.4,104.1 160.5,104.1 196.7,102.9 232.9,101.5 269.1,98.6 305.3,94.7 341.5,72.4 377.6,82.0 413.8,78.3 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,107.3 124.4,107.8 160.5,104.5 196.7,99.9 232.9,96.6 269.1,90.2 305.3,92.0 341.5,69.3 377.6,79.8 413.8,75.8 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.5 88.2,107.3 124.4,105.5 160.5,100.0 196.7,89.4 232.9,88.6 269.1,86.7 305.3,86.5 341.5,65.6 377.6,72.0 413.8,72.3 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,107.1 124.4,100.2 160.5,97.3 196.7,89.1 232.9,84.5 269.1,81.5 305.3,49.9 341.5,61.6 377.6,68.8 413.8,64.8 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.17 µs | 9.24 µs | 12.4 µs | 10.5 µs | 13.9 µs |
| D38 | 7.71 µs | 11.6 µs | 14.5 µs | 17.1 µs | 19.7 µs |
| D57 | 4.47 µs | 4.03 µs | 3.81 µs | 3.9 µs | 4.64 µs |
| D76 | 4.47 µs | 4.27 µs | 4.36 µs | 4.6 µs | 5.31 µs |
| D115 | 8.32 µs | 8.93 µs | 8.45 µs | 10.2 µs | 10.5 µs |
| D153 | 8.36 µs | 9.18 µs | 9.67 µs | 10.7 µs | 11.5 µs |
| D230 | 10.9 µs | 11.9 µs | 13.9 µs | 14.8 µs | 16.7 µs |
| D307 | 15.9 µs | 19.3 µs | 18.7 µs | 23.3 µs | 28 µs |
| D462 | 13.1 µs | 20.2 µs | 23.5 µs | 29.7 µs | 33.8 µs |
| D616 | 28.7 µs | 43.3 µs | 55.8 µs | 66.8 µs | 71.8 µs |
| D924 | 34.8 µs | 72.4 µs | 81.9 µs | 110 µs | 149 µs |
| D1232 | 44.7 µs | 113 µs | 166 µs | 201 µs | 270 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.2 88.2,150.8 124.4,166.6 160.5,166.6 196.7,148.7 232.9,148.5 269.1,140.9 305.3,129.9 341.5,135.5 377.6,112.9 413.8,107.3 450.0,99.9 450.0,47.9 413.8,65.2 377.6,86.3 341.5,108.1 305.3,113.5 269.1,128.5 232.9,139.4 196.7,142.0 160.5,161.7 124.4,165.6 88.2,123.7 52.0,133.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.2 88.2,150.8 124.4,166.6 160.5,166.6 196.7,148.7 232.9,148.5 269.1,140.9 305.3,129.9 341.5,135.5 377.6,112.9 413.8,107.3 450.0,99.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.6 88.2,139.0 124.4,169.7 160.5,168.0 196.7,146.6 232.9,145.8 269.1,138.3 305.3,124.4 341.5,122.9 377.6,100.9 413.8,86.0 450.0,73.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.1 88.2,132.5 124.4,171.3 160.5,167.4 196.7,148.2 232.9,144.3 269.1,133.9 305.3,125.2 341.5,118.6 377.6,93.6 413.8,82.5 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.8 88.2,127.8 124.4,170.6 160.5,165.8 196.7,142.7 232.9,141.5 269.1,132.0 305.3,118.8 341.5,111.8 377.6,88.4 413.8,73.9 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.8 88.2,123.7 124.4,165.6 160.5,161.7 196.7,142.0 232.9,139.4 269.1,128.5 305.3,113.5 341.5,108.1 377.6,86.3 413.8,65.2 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 3.06 ns | 3.2 ns | 3.89 ns | 5.09 ns |
| D38 | 3.48 ns | 13.6 ns | 26.3 ns | 25 ns | 28.5 ns |
| D57 | 4.22 ns | 20.8 ns | 27.8 ns | 72.4 ns | 71.7 ns |
| D76 | 5.64 ns | 34.2 ns | 42.1 ns | 79.8 ns | 107 ns |
| D115 | 13.5 ns | 56.6 ns | 98.3 ns | 214 ns | 252 ns |
| D153 | 16.9 ns | 57.5 ns | 121 ns | 258 ns | 396 ns |
| D230 | 27.8 ns | 122 ns | 338 ns | 518 ns | 969 ns |
| D307 | 44.4 ns | 188 ns | 417 ns | 1.06 µs | 1.46 µs |
| D462 | 73 ns | 415 ns | 1.25 µs | 1.87 µs | 2.61 µs |
| D616 | 104 ns | 734 ns | 1.85 µs | 2.92 µs | 3.88 µs |
| D924 | 113 ns | 1.59 µs | 2.52 µs | 4.93 µs | 7.61 µs |
| D1232 | 162 ns | 2.4 µs | 5.04 µs | 8.14 µs | 14.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,188.3 124.4,185.0 160.5,179.9 196.7,164.7 232.9,160.9 269.1,152.2 305.3,144.1 341.5,135.5 377.6,129.3 413.8,127.8 450.0,121.7 450.0,44.0 413.8,54.7 377.6,66.5 341.5,73.3 305.3,83.4 269.1,90.5 232.9,106.1 196.7,113.9 160.5,128.8 124.4,135.8 88.2,151.8 52.0,181.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,188.3 124.4,185.0 160.5,179.9 196.7,164.7 232.9,160.9 269.1,152.2 305.3,144.1 341.5,135.5 377.6,129.3 413.8,127.8 450.0,121.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.5 88.2,164.6 124.4,157.3 160.5,148.7 196.7,139.9 232.9,139.6 269.1,126.6 305.3,119.1 341.5,105.3 377.6,95.4 413.8,81.9 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,153.2 124.4,152.2 160.5,145.0 196.7,130.3 232.9,126.7 269.1,108.9 305.3,105.2 341.5,86.1 377.6,79.3 413.8,74.0 450.0,61.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.4 88.2,154.1 124.4,135.6 160.5,133.9 196.7,116.8 232.9,113.5 269.1,101.4 305.3,89.0 341.5,79.1 377.6,71.4 413.8,62.3 450.0,53.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.7 88.2,151.8 124.4,135.8 160.5,128.8 196.7,113.9 232.9,106.1 269.1,90.5 305.3,83.4 341.5,73.3 377.6,66.5 413.8,54.7 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.703 ns | 0.703 ns | 0.545 ns | 0.622 ns |
| D38 | 1.32 ns | 1.33 ns | 1.32 ns | 1.33 ns | 1.32 ns |
| D57 | 1.74 ns | 1.87 ns | 1.35 ns | 1.56 ns | 1.87 ns |
| D76 | 2.17 ns | 2.1 ns | 2.09 ns | 2.1 ns | 2.62 ns |
| D115 | 3.17 ns | 3.17 ns | 2.86 ns | 3.55 ns | 3.56 ns |
| D153 | 4.22 ns | 4.22 ns | 4.6 ns | 4.6 ns | 4.6 ns |
| D230 | 6.66 ns | 5.86 ns | 7.16 ns | 7.16 ns | 7.16 ns |
| D307 | 10.9 ns | 12.5 ns | 7.73 ns | 11.1 ns | 12.5 ns |
| D462 | 14.1 ns | 14.9 ns | 15.3 ns | 17 ns | 16.7 ns |
| D616 | 28.3 ns | 21.9 ns | 21.9 ns | 22 ns | 20.2 ns |
| D924 | 38.9 ns | 84.9 ns | 77.1 ns | 59.5 ns | 75.8 ns |
| D1232 | 58.7 ns | 69.8 ns | 69.7 ns | 61.4 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,135.2 124.4,127.3 160.5,121.0 196.7,110.0 232.9,101.6 269.1,88.4 305.3,74.2 341.5,66.7 377.6,46.5 413.8,37.4 450.0,25.4 450.0,20.4 413.8,18.0 377.6,56.3 341.5,61.9 305.3,70.2 269.1,86.4 232.9,99.1 196.7,106.6 160.5,115.4 124.4,125.2 88.2,135.2 52.0,157.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,135.2 124.4,127.3 160.5,121.0 196.7,110.0 232.9,101.6 269.1,88.4 305.3,74.2 341.5,66.7 377.6,46.5 413.8,37.4 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,135.1 124.4,125.2 160.5,121.9 196.7,109.9 232.9,101.6 269.1,92.2 305.3,70.2 341.5,65.1 377.6,53.9 413.8,14.7 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,134.7 160.5,121.9 196.7,112.9 232.9,99.1 269.1,86.4 305.3,84.1 341.5,64.3 377.6,53.9 413.8,17.5 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.9 88.2,135.1 124.4,130.4 160.5,121.9 196.7,106.6 232.9,99.1 269.1,86.3 305.3,73.7 341.5,61.3 377.6,53.9 413.8,25.0 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.2 124.4,125.2 160.5,115.4 196.7,106.6 232.9,99.1 269.1,86.4 305.3,70.2 341.5,61.9 377.6,56.3 413.8,18.0 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 5.29 µs | 7.47 µs | 6.22 µs | 7.94 µs |
| D38 | 6.24 ns | 6.88 µs | 8.22 µs | 9.37 µs | 10.6 µs |
| D57 | 64.9 ns | 3.99 µs | 3.66 µs | 4.13 µs | 4.92 µs |
| D76 | 78.5 ns | 3.84 µs | 4.5 µs | 4.99 µs | 5.86 µs |
| D115 | 149 ns | 8.62 µs | 8.66 µs | 10.9 µs | 11.4 µs |
| D153 | 191 ns | 8.99 µs | 10.2 µs | 11.8 µs | 12.9 µs |
| D230 | 343 ns | 12.3 µs | 14.9 µs | 17.8 µs | 20.7 µs |
| D307 | 367 ns | 20.2 µs | 19 µs | 27.6 µs | 34.4 µs |
| D462 | 603 ns | 69.9 µs | 136 µs | 221 µs | 280 µs |
| D616 | 797 ns | 192 µs | 351 µs | 381 µs | 527 µs |
| D924 | 754 ns | 451 µs | 381 µs | 755 µs | 1.62 ms |
| D1232 | 1.2 µs | 827 µs | 858 µs | 2.11 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,187.3 124.4,158.2 160.5,155.9 196.7,147.9 232.9,144.8 269.1,137.6 305.3,136.7 341.5,130.6 377.6,127.1 413.8,127.8 450.0,122.0 450.0,25.0 413.8,32.6 377.6,46.5 341.5,54.4 305.3,80.4 269.1,86.7 232.9,92.5 196.7,94.0 160.5,102.3 124.4,104.5 88.2,95.0 52.0,98.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,187.3 124.4,158.2 160.5,155.9 196.7,147.9 232.9,144.8 269.1,137.6 305.3,136.7 341.5,130.6 377.6,127.1 413.8,127.8 450.0,122.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,100.4 124.4,107.1 160.5,107.6 196.7,97.6 232.9,97.0 269.1,93.1 305.3,87.0 341.5,71.6 377.6,59.0 413.8,48.4 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.3 88.2,98.1 124.4,108.2 160.5,105.6 196.7,97.5 232.9,95.4 269.1,90.8 305.3,87.8 341.5,63.4 377.6,51.6 413.8,50.5 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.6 88.2,96.5 124.4,106.7 160.5,104.3 196.7,94.6 232.9,93.7 269.1,88.5 305.3,83.1 341.5,57.3 377.6,50.6 413.8,42.1 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,95.0 124.4,104.5 160.5,102.3 196.7,94.0 232.9,92.5 269.1,86.7 305.3,80.4 341.5,54.4 377.6,46.5 413.8,32.6 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 2.11 ns | 2.11 ns | 1.91 ns | 2.18 ns |
| D38 | 7.18 ns | 7.19 ns | 12.4 ns | 12.6 ns | 13.1 ns |
| D57 | 8.09 ns | 7.17 ns | 6.27 ns | 6.54 ns | 7.17 ns |
| D76 | 9.83 ns | 8.44 ns | 8.71 ns | 8.71 ns | 9.84 ns |
| D115 | 14.4 ns | 14.1 ns | 12.5 ns | 14.1 ns | 14.1 ns |
| D153 | 20.7 ns | 20 ns | 20.1 ns | 20.1 ns | 20.1 ns |
| D230 | 38.3 ns | 32.1 ns | 32.1 ns | 31.9 ns | 32.1 ns |
| D307 | 41.3 ns | 48.5 ns | 30.2 ns | 43.9 ns | 48.2 ns |
| D462 | 66.1 ns | 73.5 ns | 74.1 ns | 87 ns | 82.7 ns |
| D616 | 123 ns | 98.2 ns | 95.4 ns | 97.3 ns | 77.4 ns |
| D924 | 74.5 ns | 117 ns | 108 ns | 61.1 ns | 94.4 ns |
| D1232 | 128 ns | 137 ns | 133 ns | 133 ns | 122 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,152.9 124.4,149.5 160.5,143.8 196.7,132.8 232.9,122.2 269.1,104.4 305.3,102.3 341.5,88.6 377.6,70.7 413.8,85.2 450.0,69.6 450.0,70.9 413.8,78.3 377.6,84.1 341.5,82.2 305.3,97.8 269.1,109.6 232.9,123.2 196.7,133.5 160.5,143.8 124.4,153.0 88.2,135.5 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,152.9 124.4,149.5 160.5,143.8 196.7,132.8 232.9,122.2 269.1,104.4 305.3,102.3 341.5,88.6 377.6,70.7 413.8,85.2 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,152.9 124.4,153.0 160.5,148.3 196.7,133.5 232.9,123.2 269.1,109.5 305.3,97.6 341.5,85.6 377.6,77.2 413.8,72.1 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,137.0 124.4,156.8 160.5,147.3 196.7,137.0 232.9,123.2 269.1,109.6 305.3,111.3 341.5,85.3 377.6,78.0 413.8,74.5 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.3 88.2,136.6 124.4,155.7 160.5,147.3 196.7,133.5 232.9,123.2 269.1,109.8 305.3,100.5 341.5,80.7 377.6,77.5 413.8,90.9 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,135.5 124.4,153.0 160.5,143.8 196.7,133.5 232.9,123.2 269.1,109.6 305.3,97.8 341.5,82.2 377.6,84.1 413.8,78.3 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 4.12 µs | 5.77 µs | 4.91 µs | 6.36 µs |
| D38 | 4.36 ns | 5.31 µs | 6.57 µs | 7.74 µs | 8.89 µs |
| D57 | 2.78 ns | 3.13 µs | 3.53 µs | 4.91 µs | 8.67 µs |
| D76 | 3.87 ns | 3.36 µs | 4.85 µs | 6.86 µs | 8.95 µs |
| D115 | 16.8 ns | 4.49 µs | 9.42 µs | 13.8 µs | 18.6 µs |
| D153 | 22.5 ns | 5.31 µs | 10.1 µs | 18.6 µs | 31.3 µs |
| D230 | 52.6 ns | 9.46 µs | 18.1 µs | 35.5 µs | 66.9 µs |
| D307 | 77.2 ns | 12 µs | 22.4 µs | 66.4 µs | 115 µs |
| D462 | 142 ns | 14.6 µs | 58.9 µs | 139 µs | 230 µs |
| D616 | 169 ns | 32 µs | 127 µs | 266 µs | 425 µs |
| D924 | 107 ns | 73.4 µs | 213 µs | 542 µs | 1.03 ms |
| D1232 | 315 ns | 125 µs | 451 µs | 1.03 ms | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,191.7 124.4,197.3 160.5,193.2 196.7,175.0 232.9,171.4 269.1,160.8 305.3,156.1 341.5,148.5 377.6,146.4 413.8,152.0 450.0,138.6 450.0,27.5 413.8,38.2 377.6,49.2 341.5,56.8 305.3,65.5 269.1,72.1 232.9,81.6 196.7,88.0 160.5,97.1 124.4,97.5 88.2,97.2 52.0,101.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,191.7 124.4,197.3 160.5,193.2 196.7,175.0 232.9,171.4 269.1,160.8 305.3,156.1 341.5,148.5 377.6,146.4 413.8,152.0 450.0,138.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.7 88.2,103.6 124.4,110.1 160.5,109.2 196.7,105.6 232.9,103.6 269.1,96.4 305.3,93.5 341.5,91.0 377.6,81.3 413.8,71.0 450.0,64.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.5 88.2,100.9 124.4,108.6 160.5,104.7 196.7,96.5 232.9,95.6 269.1,88.3 305.3,85.7 341.5,73.7 377.6,64.1 413.8,57.8 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.5 88.2,98.9 124.4,104.6 160.5,100.4 196.7,91.7 232.9,88.0 269.1,80.0 305.3,72.2 341.5,63.0 377.6,55.0 413.8,46.2 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.3 88.2,97.2 124.4,97.5 160.5,97.1 196.7,88.0 232.9,81.6 269.1,72.1 305.3,65.5 341.5,56.8 377.6,49.2 413.8,38.2 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.94 µs | 10.2 µs | 8.56 µs | 11.1 µs |
| D38 | 4.05 ns | 9.44 µs | 11.5 µs | 13.4 µs | 15.2 µs |
| D57 | 12.3 ns | 5.21 µs | 5.8 µs | 7.17 µs | 10.1 µs |
| D76 | 12.1 ns | 5.56 µs | 7.35 µs | 10 µs | 12.9 µs |
| D115 | 11.2 ns | 12.5 µs | 11.1 µs | 22.8 µs | 28.1 µs |
| D153 | 20.8 ns | 8.17 µs | 15.7 µs | 24.4 µs | 38.4 µs |
| D230 | 53.2 ns | 13.4 µs | 22.4 µs | 42.8 µs | 78.8 µs |
| D307 | 76.7 ns | 17.2 µs | 41.8 µs | 79.4 µs | 123 µs |
| D462 | 143 ns | 23 µs | 80.9 µs | 167 µs | 247 µs |
| D616 | 166 ns | 39.7 µs | 141 µs | 288 µs | 414 µs |
| D924 | 124 ns | 87.2 µs | 227 µs | 543 µs | 910 µs |
| D1232 | 306 ns | 141 µs | 445 µs | 910 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.7 124.4,178.9 160.5,179.0 196.7,180.0 232.9,172.4 269.1,160.7 305.3,156.2 341.5,148.4 377.6,146.6 413.8,150.2 450.0,139.0 450.0,25.6 413.8,39.7 377.6,49.5 341.5,55.9 305.3,64.6 269.1,70.1 232.9,79.0 196.7,82.9 160.5,92.6 124.4,95.6 88.2,90.5 52.0,94.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.7 124.4,178.9 160.5,179.0 196.7,180.0 232.9,172.4 269.1,160.7 305.3,156.2 341.5,148.4 377.6,146.6 413.8,150.2 450.0,139.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.4 124.4,103.8 160.5,103.0 196.7,92.9 232.9,98.2 269.1,92.0 305.3,89.0 341.5,85.4 377.6,78.6 413.8,68.8 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,94.0 124.4,102.5 160.5,99.5 196.7,94.4 232.9,90.1 269.1,85.7 305.3,78.0 341.5,69.8 377.6,62.9 413.8,57.0 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,92.1 124.4,99.8 160.5,95.7 196.7,85.5 232.9,84.7 269.1,77.7 305.3,70.0 341.5,60.8 377.6,54.0 413.8,46.2 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.4 88.2,90.5 124.4,95.6 160.5,92.6 196.7,82.9 232.9,79.0 269.1,70.1 305.3,64.6 341.5,55.9 377.6,49.5 413.8,39.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.92 ns | 14.4 ns | 19.9 ns | 23.6 ns | 28.9 ns |
| D38 | 7.51 ns | 26.7 ns | 31.1 ns | 1.47 µs | 2.84 µs |
| D57 | 175 ns | 206 ns | 349 ns | 573 ns | 766 ns |
| D76 | 208 ns | 270 ns | 769 ns | 839 ns | 1.05 µs |
| D115 | 126 ns | 601 ns | 1.01 µs | 1.52 µs | 1.65 µs |
| D153 | 137 ns | 1.03 µs | 1.53 µs | 1.85 µs | 2.6 µs |
| D230 | 158 ns | 1.55 µs | 2.4 µs | 3.54 µs | 4.22 µs |
| D307 | 144 ns | 2.3 µs | 3.12 µs | 4.61 µs | 7.18 µs |
| D462 | 209 ns | 3.73 µs | 5.89 µs | 9.87 µs | 11.7 µs |
| D616 | 266 ns | 6.43 µs | 11.1 µs | 14.3 µs | 20.5 µs |
| D924 | 173 ns | 11.4 µs | 13.6 µs | 23.1 µs | 34.8 µs |
| D1232 | 273 ns | 16.4 µs | 28.3 µs | 39.3 µs | 62 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.4 88.2,175.0 124.4,120.3 160.5,117.3 196.7,126.0 232.9,124.5 269.1,122.0 305.3,123.7 341.5,117.2 377.6,113.0 413.8,120.4 450.0,112.6 450.0,18.3 413.8,28.3 377.6,37.5 341.5,47.2 305.3,55.8 269.1,65.0 232.9,73.4 196.7,81.3 160.5,89.2 124.4,94.6 88.2,71.8 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,176.4 88.2,175.0 124.4,120.3 160.5,117.3 196.7,126.0 232.9,124.5 269.1,122.0 305.3,123.7 341.5,117.2 377.6,113.0 413.8,120.4 450.0,112.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.6 88.2,152.9 124.4,117.4 160.5,112.7 196.7,98.8 232.9,89.5 269.1,82.4 305.3,75.6 341.5,67.2 377.6,57.7 413.8,47.8 450.0,41.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.0 88.2,150.3 124.4,108.3 160.5,94.6 196.7,89.8 232.9,82.6 269.1,74.8 305.3,70.3 341.5,59.2 377.6,48.2 413.8,44.6 450.0,32.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.1 88.2,83.3 124.4,99.7 160.5,93.1 196.7,82.7 232.9,79.3 269.1,68.1 305.3,63.5 341.5,50.2 377.6,43.8 413.8,35.5 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,71.8 124.4,94.6 160.5,89.2 196.7,81.3 232.9,73.4 269.1,65.0 305.3,55.8 341.5,47.2 377.6,37.5 413.8,28.3 450.0,18.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.06 ns | 1.06 ns | 0.819 ns | 0.935 ns |
| D38 | 1.6 ns | 1.61 ns | 1.62 ns | 1.62 ns | 1.6 ns |
| D57 | 2.5 ns | 2.25 ns | 1.94 ns | 2.93 ns | 2.25 ns |
| D76 | 3.45 ns | 3.09 ns | 3.08 ns | 3.09 ns | 3.45 ns |
| D115 | 5.54 ns | 5.54 ns | 4.84 ns | 5.55 ns | 5.57 ns |
| D153 | 8.48 ns | 8.46 ns | 8.46 ns | 8.47 ns | 8.47 ns |
| D230 | 17.7 ns | 16.1 ns | 16.1 ns | 16.1 ns | 16.1 ns |
| D307 | 23.7 ns | 25.2 ns | 18.1 ns | 23.5 ns | 25.1 ns |
| D462 | 32.4 ns | 38.6 ns | 37.3 ns | 56.6 ns | 41.1 ns |
| D616 | 77.9 ns | 49 ns | 48.8 ns | 51.4 ns | 45.9 ns |
| D924 | 60.9 ns | 84.9 ns | 78.7 ns | 61 ns | 74.8 ns |
| D1232 | 93.8 ns | 106 ns | 109 ns | 96.6 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.8 88.2,149.8 124.4,140.1 160.5,133.1 196.7,122.8 232.9,113.6 269.1,97.7 305.3,91.3 341.5,84.5 377.6,65.4 413.8,70.8 450.0,61.4 450.0,58.6 413.8,66.3 377.6,76.9 341.5,79.3 305.3,90.0 269.1,99.6 232.9,113.6 196.7,122.7 160.5,133.1 124.4,142.3 88.2,149.8 52.0,161.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,158.8 88.2,149.8 124.4,140.1 160.5,133.1 196.7,122.8 232.9,113.6 269.1,97.7 305.3,91.3 341.5,84.5 377.6,65.4 413.8,70.8 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.8 88.2,149.6 124.4,142.4 160.5,135.5 196.7,122.8 232.9,113.6 269.1,99.7 305.3,89.9 341.5,80.7 377.6,75.5 413.8,63.6 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,149.6 124.4,145.6 160.5,135.5 196.7,125.8 232.9,113.6 269.1,99.6 305.3,97.2 341.5,81.4 377.6,75.6 413.8,65.2 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.3 88.2,149.5 124.4,136.7 160.5,135.5 196.7,122.8 232.9,113.6 269.1,99.7 305.3,91.4 341.5,72.4 377.6,74.4 413.8,70.7 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.5 88.2,149.8 124.4,142.3 160.5,133.1 196.7,122.7 232.9,113.6 269.1,99.6 305.3,90.0 341.5,79.3 377.6,76.9 413.8,66.3 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 8.22 µs | 10.7 µs | 9.03 µs | 11.7 µs |
| D38 | 4.36 ns | 9.92 µs | 12.2 µs | 14.4 µs | 16.4 µs |
| D57 | 3.16 ns | 4.15 µs | 4.68 µs | 6.31 µs | 8.6 µs |
| D76 | 4.22 ns | 4.4 µs | 6.51 µs | 8.78 µs | 11.3 µs |
| D115 | 16.8 ns | 5.94 µs | 12.1 µs | 17.1 µs | 22.2 µs |
| D153 | 22.2 ns | 7.11 µs | 13.6 µs | 24.2 µs | 36.7 µs |
| D230 | 59.2 ns | 12 µs | 21.8 µs | 40.5 µs | 76.1 µs |
| D307 | 74.6 ns | 15.2 µs | 26.2 µs | 74.7 µs | 127 µs |
| D462 | 136 ns | 17.9 µs | 67.7 µs | 154 µs | 249 µs |
| D616 | 169 ns | 37.2 µs | 141 µs | 293 µs | 462 µs |
| D924 | 134 ns | 81.5 µs | 237 µs | 583 µs | 1.1 ms |
| D1232 | 313 ns | 139 µs | 486 µs | 1.1 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,191.7 124.4,195.7 160.5,192.1 196.7,175.0 232.9,171.5 269.1,159.4 305.3,156.5 341.5,149.1 377.6,146.4 413.8,149.2 450.0,138.7 450.0,26.8 413.8,37.4 377.6,48.2 341.5,55.8 305.3,64.2 269.1,70.5 232.9,79.6 196.7,85.8 160.5,94.2 124.4,97.6 88.2,89.6 52.0,93.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,191.7 124.4,195.7 160.5,192.1 196.7,175.0 232.9,171.5 269.1,159.4 305.3,156.5 341.5,149.1 377.6,146.4 413.8,149.2 450.0,138.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.2 88.2,95.8 124.4,106.6 160.5,105.9 196.7,102.2 232.9,100.0 269.1,93.4 305.3,90.5 341.5,88.5 377.6,79.4 413.8,69.7 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,93.2 124.4,105.1 160.5,101.0 196.7,93.4 232.9,91.9 269.1,86.0 305.3,83.8 341.5,72.0 377.6,62.9 413.8,56.4 450.0,47.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.0 88.2,91.2 124.4,101.4 160.5,97.3 196.7,89.0 232.9,84.8 269.1,78.4 305.3,70.8 341.5,61.8 377.6,53.8 413.8,45.3 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.7 88.2,89.6 124.4,97.6 160.5,94.2 196.7,85.8 232.9,79.6 269.1,70.5 305.3,64.2 341.5,55.8 377.6,48.2 413.8,37.4 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.92 µs | 10.3 µs | 8.69 µs | 11.2 µs |
| D38 | 3.74 ns | 9.57 µs | 11.5 µs | 13.4 µs | 15.3 µs |
| D57 | 2.83 µs | 5.31 µs | 5.99 µs | 7.41 µs | 10.5 µs |
| D76 | 2.86 µs | 5.7 µs | 7.71 µs | 10.4 µs | 13.2 µs |
| D115 | 5.92 µs | 13 µs | 11.9 µs | 24.2 µs | 28.5 µs |
| D153 | 3.07 µs | 8.45 µs | 16.5 µs | 24.6 µs | 39.6 µs |
| D230 | 3.17 µs | 14 µs | 23 µs | 43.6 µs | 80.1 µs |
| D307 | 3.16 µs | 17.6 µs | 43.4 µs | 79.9 µs | 126 µs |
| D462 | 2.91 µs | 23.6 µs | 82.2 µs | 168 µs | 249 µs |
| D616 | 3.8 µs | 40.3 µs | 143 µs | 292 µs | 418 µs |
| D924 | 3.33 µs | 88 µs | 230 µs | 548 µs | 912 µs |
| D1232 | 3.82 µs | 142 µs | 448 µs | 916 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,111.4 160.5,111.2 196.7,102.2 232.9,110.4 269.1,110.0 305.3,110.0 341.5,111.0 377.6,107.7 413.8,109.3 450.0,107.7 450.0,25.5 413.8,39.7 377.6,49.4 341.5,55.8 305.3,64.3 269.1,69.9 232.9,78.6 196.7,82.7 160.5,92.3 124.4,95.1 88.2,90.5 52.0,94.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,111.4 160.5,111.2 196.7,102.2 232.9,110.4 269.1,110.0 305.3,110.0 341.5,111.0 377.6,107.7 413.8,109.3 450.0,107.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.3 124.4,103.6 160.5,102.7 196.7,92.4 232.9,97.8 269.1,91.6 305.3,88.7 341.5,85.0 377.6,78.4 413.8,68.7 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,93.9 124.4,102.1 160.5,98.9 196.7,93.6 232.9,89.5 269.1,85.4 305.3,77.5 341.5,69.6 377.6,62.7 413.8,56.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.4 88.2,92.1 124.4,99.4 160.5,95.2 196.7,84.7 232.9,84.6 269.1,77.5 305.3,69.9 341.5,60.7 377.6,53.9 413.8,46.0 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,90.5 124.4,95.1 160.5,92.3 196.7,82.7 232.9,78.6 269.1,69.9 305.3,64.3 341.5,55.8 377.6,49.4 413.8,39.7 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4 ns | 3.21 µs | 3.46 µs | 2.94 µs | 3.85 µs |
| D38 | 4.05 ns | 3.22 µs | 4.01 µs | 4.71 µs | 5.43 µs |
| D57 | 197 ns | 292 ns | 257 ns | 278 ns | 388 ns |
| D76 | 200 ns | 281 ns | 308 ns | 401 ns | 464 ns |
| D115 | 392 ns | 523 ns | 589 ns | 759 ns | 845 ns |
| D153 | 385 ns | 561 ns | 707 ns | 852 ns | 1.02 µs |
| D230 | 556 ns | 784 ns | 944 ns | 1.23 µs | 1.74 µs |
| D307 | 835 ns | 1.24 µs | 1.35 µs | 2.03 µs | 2.73 µs |
| D462 | 758 ns | 1.31 µs | 2.09 µs | 3.2 µs | 4.18 µs |
| D616 | 1.17 µs | 1.91 µs | 3.01 µs | 4.34 µs | 5.58 µs |
| D924 | 1.19 µs | 2.85 µs | 3.74 µs | 6.64 µs | 10.2 µs |
| D1232 | 1.83 µs | 4.08 µs | 7.22 µs | 10.8 µs | 30.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.9 88.2,185.7 124.4,118.2 160.5,118.0 196.7,106.3 232.9,106.6 269.1,100.2 305.3,93.1 341.5,94.8 377.6,87.2 413.8,87.0 450.0,79.5 450.0,30.7 413.8,49.7 377.6,60.1 341.5,65.2 305.3,72.5 269.1,80.4 232.9,89.7 196.7,92.9 160.5,103.3 124.4,106.4 88.2,60.6 52.0,66.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.9 88.2,185.7 124.4,118.2 160.5,118.0 196.7,106.3 232.9,106.6 269.1,100.2 305.3,93.1 341.5,94.8 377.6,87.2 413.8,87.0 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,69.7 88.2,69.7 124.4,111.4 160.5,112.1 196.7,101.2 232.9,100.1 269.1,94.2 305.3,86.2 341.5,85.4 377.6,78.8 413.8,71.8 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.4 88.2,65.9 124.4,113.6 160.5,110.5 196.7,99.2 232.9,96.0 269.1,91.0 305.3,84.8 341.5,77.2 377.6,70.8 413.8,67.1 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,71.2 88.2,63.1 124.4,112.3 160.5,105.9 196.7,94.8 232.9,92.8 269.1,86.4 305.3,77.7 341.5,69.8 377.6,64.5 413.8,57.1 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.6 88.2,60.6 124.4,106.4 160.5,103.3 196.7,92.9 232.9,89.7 269.1,80.4 305.3,72.5 341.5,65.2 377.6,60.1 413.8,49.7 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 158 ns | 201 ns | 159 ns | 185 ns |
| D38 | 4.36 ns | 183 ns | 186 ns | 186 ns | 187 ns |
| D57 | 310 ns | 403 ns | 337 ns | 396 ns | 499 ns |
| D76 | 306 ns | 376 ns | 418 ns | 518 ns | 602 ns |
| D115 | 613 ns | 741 ns | 802 ns | 1.01 µs | 1.04 µs |
| D153 | 648 ns | 764 ns | 898 ns | 1.05 µs | 1.25 µs |
| D230 | 944 ns | 1.15 µs | 1.3 µs | 1.63 µs | 2.13 µs |
| D307 | 1.39 µs | 1.78 µs | 1.8 µs | 2.56 µs | 3.36 µs |
| D462 | 1.24 µs | 1.8 µs | 2.69 µs | 3.81 µs | 4.81 µs |
| D616 | 1.9 µs | 2.5 µs | 3.78 µs | 5.1 µs | 6.34 µs |
| D924 | 1.91 µs | 3.81 µs | 4.5 µs | 7.48 µs | 11.3 µs |
| D1232 | 2.81 µs | 5.29 µs | 8.48 µs | 12.2 µs | 32 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,184.4 124.4,110.3 160.5,110.6 196.7,98.5 232.9,97.5 269.1,91.0 305.3,84.2 341.5,86.3 377.6,78.9 413.8,78.7 450.0,72.1 450.0,29.8 413.8,47.9 377.6,57.9 341.5,62.7 305.3,68.9 269.1,76.9 232.9,86.1 196.7,89.4 160.5,98.8 124.4,102.1 88.2,119.1 52.0,119.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,184.4 124.4,110.3 160.5,110.6 196.7,98.5 232.9,97.5 269.1,91.0 305.3,84.2 341.5,86.3 377.6,78.9 413.8,78.7 450.0,72.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,119.5 124.4,105.8 160.5,107.0 196.7,95.2 232.9,94.7 269.1,87.6 305.3,80.0 341.5,79.8 377.6,74.1 413.8,66.8 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.8 88.2,119.2 124.4,108.9 160.5,105.2 196.7,93.8 232.9,91.9 269.1,85.4 305.3,79.8 341.5,72.8 377.6,66.9 413.8,63.9 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.0 88.2,119.2 124.4,106.1 160.5,101.4 196.7,89.7 232.9,89.2 269.1,81.5 305.3,73.7 341.5,66.7 377.6,61.7 413.8,55.0 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.3 88.2,119.1 124.4,102.1 160.5,98.8 196.7,89.4 232.9,86.1 269.1,76.9 305.3,68.9 341.5,62.7 377.6,57.9 413.8,47.9 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body -->
