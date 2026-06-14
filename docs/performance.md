# Performance

How fast each operation is, by storage width and scale.

The numbers on this page are **generated from CI**: the `bench-branch-compare`
run measures every `(operation, width, scale)` on a GitHub-hosted runner,
compares it against the previous release, and commits the medians to
`results/timing/`, which this page renders. They are refreshed on each release
PR.

> Absolute timings are machine-dependent — the *ratios* between operations and
> widths, measured in the same run, are what to read. Operands are `black_box`-ed
> so the optimiser can't fold the work away.

Times are the unit shown in each cell; the legend below maps each unit to its
size in nanoseconds. Each function's graph plots median time (log scale) against
storage width: the two solid lines are scale `0` and the maximum scale, the
dashed lines the band-edge scales in between, and the shaded band is the spread
between scale `0` and the maximum.

<!-- BEGIN GENERATED:performance:body -->
| Unit | In nanoseconds |
| :-- | --: |
| ns | 10⁰ ns |
| µs | 10³ ns |
| ms | 10⁶ ns |

### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.27 ns | 23.3 µs | 29.7 µs | 32.4 µs | 17.8 µs |
| D38 | 4.29 µs | 7.64 µs | 7.44 µs | 12.4 µs | 16.1 µs |
| D57 | 5.05 µs | 8.42 µs | 10.9 µs | 14.8 µs | 19.2 µs |
| D76 | 5.18 µs | 9.39 µs | 14.2 µs | 19.4 µs | 22 µs |
| D115 | 4.78 µs | 11 µs | 25.2 µs | 33.8 µs | 38.5 µs |
| D153 | 5.18 µs | 14.3 µs | 26.7 µs | 43.7 µs | 64.4 µs |
| D230 | 4.73 µs | 24.9 µs | 40.7 µs | 74.4 µs | 130 µs |
| D307 | 4.95 µs | 27.3 µs | 59.3 µs | 129 µs | 188 µs |
| D462 | 4.87 µs | 40.8 µs | 76.9 µs | 251 µs | 395 µs |
| D616 | 5.37 µs | 59.2 µs | 218 µs | 422 µs | 655 µs |
| D924 | 5.47 µs | 105 µs | 445 µs | 931 µs | 1.53 ms |
| D1232 | 5.3 µs | 168 µs | 701 µs | 1.5 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.4 88.2,106.2 124.4,104.2 160.5,103.9 196.7,104.9 232.9,103.9 269.1,105.0 305.3,104.4 341.5,104.7 377.6,103.4 413.8,103.2 450.0,103.6 450.0,23.1 413.8,33.3 377.6,43.8 341.5,50.1 305.3,59.3 269.1,63.8 232.9,72.6 196.7,79.0 160.5,85.9 124.4,87.6 88.2,89.8 52.0,88.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.4 88.2,106.2 124.4,104.2 160.5,103.9 196.7,104.9 232.9,103.9 269.1,105.0 305.3,104.4 341.5,104.7 377.6,103.4 413.8,103.2 450.0,103.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,99.1 124.4,97.8 160.5,96.5 196.7,94.6 232.9,91.2 269.1,84.4 305.3,83.2 341.5,78.3 377.6,73.6 413.8,66.5 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,99.4 124.4,94.6 160.5,91.4 196.7,84.2 232.9,83.5 269.1,78.3 305.3,73.6 341.5,70.4 377.6,57.5 413.8,48.6 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.0 124.4,90.8 160.5,87.5 196.7,80.6 232.9,77.4 269.1,70.8 305.3,63.9 341.5,55.7 377.6,49.3 413.8,39.5 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,88.5 88.2,89.8 124.4,87.6 160.5,85.9 196.7,79.0 232.9,72.6 269.1,63.8 305.3,59.3 341.5,50.1 377.6,43.8 413.8,33.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 27.4 µs | 31.1 µs | 35.5 µs | 39 µs | 18.7 µs |
| D38 | 25.7 µs | 35.5 µs | 41.3 µs | 57.4 µs | 66.1 µs |
| D57 | 3.7 µs | 4.31 µs | 4.95 µs | 6.1 µs | 8.15 µs |
| D76 | 3.7 µs | 5.1 µs | 5.71 µs | 8.4 µs | 10.2 µs |
| D115 | 6.08 µs | 8.65 µs | 13.3 µs | 18.1 µs | 19.5 µs |
| D153 | 6.61 µs | 9.82 µs | 14.6 µs | 23.5 µs | 33.4 µs |
| D230 | 8.32 µs | 15.9 µs | 26.8 µs | 46.3 µs | 74.6 µs |
| D307 | 12.6 µs | 26.2 µs | 47.1 µs | 88.6 µs | 137 µs |
| D462 | 12.5 µs | 36.6 µs | 49 µs | 168 µs | 273 µs |
| D616 | 22.2 µs | 71 µs | 179 µs | 314 µs | 518 µs |
| D924 | 33.5 µs | 130 µs | 400 µs | 802 µs | 1.34 ms |
| D1232 | 42.2 µs | 219 µs | 710 µs | 1.43 ms | 3.06 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,138.1 88.2,139.5 124.4,181.6 160.5,181.6 196.7,170.8 232.9,169.0 269.1,164.0 305.3,154.9 341.5,155.2 377.6,142.7 413.8,133.8 450.0,128.8 450.0,35.7 413.8,53.7 377.6,74.3 341.5,88.2 305.3,103.2 269.1,116.4 232.9,133.8 196.7,145.4 160.5,159.6 124.4,164.4 88.2,119.0 52.0,146.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,138.1 88.2,139.5 124.4,181.6 160.5,181.6 196.7,170.8 232.9,169.0 269.1,164.0 305.3,154.9 341.5,155.2 377.6,142.7 413.8,133.8 450.0,128.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,135.4 88.2,132.5 124.4,178.3 160.5,174.6 196.7,163.1 232.9,160.4 269.1,149.9 305.3,139.1 341.5,131.8 377.6,117.4 413.8,104.2 450.0,93.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.5 88.2,129.2 124.4,175.3 160.5,172.2 196.7,153.9 232.9,151.8 269.1,138.6 305.3,126.4 341.5,125.5 377.6,97.3 413.8,79.9 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,122.1 124.4,170.7 160.5,163.8 196.7,147.1 232.9,141.4 269.1,126.7 305.3,112.6 341.5,98.8 377.6,85.1 413.8,64.8 450.0,52.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,146.4 88.2,119.0 124.4,164.4 160.5,159.6 196.7,145.4 232.9,133.8 269.1,116.4 305.3,103.2 341.5,88.2 377.6,74.3 413.8,53.7 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `add`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.06 ns | 1.05 ns | 1.06 ns | 1.06 ns | 1.15 ns |
| D38 | 1.62 ns | 1.81 ns | 1.62 ns | 1.82 ns | 1.82 ns |
| D57 | 2.51 ns | 2.25 ns | 2.26 ns | 2.25 ns | 2.5 ns |
| D76 | 3.49 ns | 3.49 ns | 3.08 ns | 3.48 ns | 3.49 ns |
| D115 | 4.4 ns | 4.41 ns | 4.99 ns | 5 ns | 3.33 ns |
| D153 | 6.63 ns | 5.95 ns | 5.94 ns | 6.63 ns | 6.64 ns |
| D230 | 13.9 ns | 13.8 ns | 13.9 ns | 15.5 ns | 15.4 ns |
| D307 | 18.6 ns | 18.5 ns | 18.5 ns | 19.6 ns | 19.6 ns |
| D462 | 40 ns | 29.7 ns | 21.6 ns | 32.6 ns | 32.7 ns |
| D616 | 51.3 ns | 45 ns | 60.2 ns | 45.5 ns | 45.4 ns |
| D924 | 84.7 ns | 72.1 ns | 92.6 ns | 85.2 ns | 74.7 ns |
| D1232 | 96.1 ns | 90.7 ns | 107 ns | 95.2 ns | 107 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,208.4 88.2,196.0 124.4,183.3 160.5,173.8 196.7,167.1 232.9,155.2 269.1,133.8 305.3,125.4 341.5,103.2 377.6,96.0 413.8,81.5 450.0,77.8 450.0,74.8 413.8,85.1 377.6,99.6 341.5,109.1 305.3,123.8 269.1,130.8 232.9,155.2 196.7,175.2 160.5,173.8 124.4,183.5 88.2,192.6 52.0,206.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.4 88.2,196.0 124.4,183.3 160.5,173.8 196.7,167.1 232.9,155.2 269.1,133.8 305.3,125.4 341.5,103.2 377.6,96.0 413.8,81.5 450.0,77.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,208.4 88.2,192.7 124.4,186.5 160.5,173.8 196.7,167.1 232.9,158.3 269.1,133.9 305.3,125.5 341.5,111.9 377.6,99.8 413.8,86.1 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,208.4 88.2,196.0 124.4,186.4 160.5,177.4 196.7,163.5 232.9,158.4 269.1,133.8 305.3,125.4 341.5,121.0 377.6,91.3 413.8,78.9 450.0,74.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,208.3 88.2,192.7 124.4,186.5 160.5,173.9 196.7,163.4 232.9,155.2 269.1,130.7 305.3,123.9 341.5,109.2 377.6,99.5 413.8,81.3 450.0,78.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,206.0 88.2,192.6 124.4,183.5 160.5,173.8 196.7,175.2 232.9,155.2 269.1,130.8 305.3,123.8 341.5,109.1 377.6,99.6 413.8,85.1 450.0,74.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 23.3 µs | 29.7 µs | 32.4 µs | 17.8 µs |
| D38 | 4.29 µs | 7.62 µs | 7.39 µs | 12.4 µs | 16.1 µs |
| D57 | 5.01 µs | 8.35 µs | 10.9 µs | 14.8 µs | 19.2 µs |
| D76 | 5.15 µs | 9.36 µs | 14.2 µs | 19.4 µs | 22 µs |
| D115 | 4.71 µs | 10.9 µs | 25.3 µs | 33.2 µs | 38.9 µs |
| D153 | 5.12 µs | 14.3 µs | 26.9 µs | 45.3 µs | 63.8 µs |
| D230 | 4.7 µs | 24 µs | 41.2 µs | 74.4 µs | 131 µs |
| D307 | 4.71 µs | 27 µs | 59.2 µs | 129 µs | 188 µs |
| D462 | 4.8 µs | 41.2 µs | 75.7 µs | 249 µs | 396 µs |
| D616 | 5.34 µs | 59.4 µs | 215 µs | 422 µs | 654 µs |
| D924 | 5.39 µs | 107 µs | 449 µs | 937 µs | 1.53 ms |
| D1232 | 5.26 µs | 168 µs | 700 µs | 1.5 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,106.2 124.4,104.3 160.5,104.0 196.7,105.1 232.9,104.0 269.1,105.1 305.3,105.1 341.5,104.8 377.6,103.5 413.8,103.4 450.0,103.7 450.0,23.1 413.8,33.3 377.6,43.8 341.5,50.1 305.3,59.3 269.1,63.8 232.9,72.7 196.7,78.9 160.5,85.9 124.4,87.6 88.2,89.8 52.0,88.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,106.2 124.4,104.3 160.5,104.0 196.7,105.1 232.9,104.0 269.1,105.1 305.3,105.1 341.5,104.8 377.6,103.5 413.8,103.4 450.0,103.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,99.1 124.4,98.0 160.5,96.5 196.7,94.6 232.9,91.3 269.1,84.9 305.3,83.4 341.5,78.1 377.6,73.6 413.8,66.3 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,99.5 124.4,94.6 160.5,91.4 196.7,84.2 232.9,83.5 269.1,78.1 305.3,73.6 341.5,70.6 377.6,57.6 413.8,48.5 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.1 124.4,90.9 160.5,87.5 196.7,80.8 232.9,77.0 269.1,70.8 305.3,64.0 341.5,55.8 377.6,49.3 413.8,39.4 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,88.5 88.2,89.8 124.4,87.6 160.5,85.9 196.7,78.9 232.9,72.7 269.1,63.8 305.3,59.3 341.5,50.1 377.6,43.8 413.8,33.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 23 µs | 29 µs | 31.8 µs | 17.2 µs |
| D38 | 3.73 ns | 29 µs | 33.7 µs | 42.9 µs | 55.2 µs |
| D57 | 2.11 ns | 5.4 µs | 7.27 µs | 8.92 µs | 12.8 µs |
| D76 | 2.22 ns | 6.93 µs | 8.74 µs | 13 µs | 15.2 µs |
| D115 | 13.4 ns | 12.6 µs | 21 µs | 26.7 µs | 31.2 µs |
| D153 | 15.9 ns | 14.9 µs | 22.1 µs | 37.2 µs | 47.8 µs |
| D230 | 27.9 ns | 24.8 µs | 41.8 µs | 67.3 µs | 102 µs |
| D307 | 44.8 ns | 37.3 µs | 66.3 µs | 120 µs | 174 µs |
| D462 | 69.4 ns | 54.9 µs | 69.3 µs | 207 µs | 336 µs |
| D616 | 88.8 ns | 106 µs | 247 µs | 405 µs | 622 µs |
| D924 | 114 ns | 199 µs | 532 µs | 941 µs | 1.47 ms |
| D1232 | 141 ns | 307 µs | 921 µs | 1.62 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,200.7 160.5,200.1 196.7,177.8 232.9,175.7 269.1,168.7 305.3,162.8 341.5,157.4 377.6,154.3 413.8,151.2 450.0,148.6 450.0,24.5 413.8,33.8 377.6,44.5 341.5,52.1 305.3,60.3 269.1,66.9 232.9,76.3 196.7,81.6 160.5,90.5 124.4,92.7 88.2,74.5 52.0,89.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,200.7 160.5,200.1 196.7,177.8 232.9,175.7 269.1,168.7 305.3,162.8 341.5,157.4 377.6,154.3 413.8,151.2 450.0,148.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.4 88.2,82.5 124.4,103.4 160.5,100.3 196.7,92.8 232.9,90.8 269.1,84.4 305.3,79.4 341.5,74.6 377.6,66.4 413.8,58.6 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.5 88.2,80.6 124.4,99.7 160.5,97.4 196.7,86.5 232.9,85.9 269.1,78.0 305.3,72.2 341.5,71.7 377.6,55.9 413.8,46.4 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.4 88.2,77.6 124.4,97.1 160.5,92.5 196.7,83.5 232.9,79.4 269.1,72.1 305.3,64.8 341.5,58.1 377.6,49.8 413.8,39.3 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,89.0 88.2,74.5 124.4,92.7 160.5,90.5 196.7,81.6 232.9,76.3 269.1,66.9 305.3,60.3 341.5,52.1 377.6,44.5 413.8,33.8 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.63 µs | 2.69 µs | 2.93 µs | 2.85 µs |
| D38 | 5.39 µs | 8.96 µs | 6.16 µs | 10.7 µs | 14 µs |
| D57 | 4.15 µs | 7.09 µs | 9.5 µs | 13.2 µs | 5.4 µs |
| D76 | 4.28 µs | 8.23 µs | 12.3 µs | 17.1 µs | 19.5 µs |
| D115 | 3.91 µs | 9.37 µs | 22.4 µs | 29.9 µs | 34.7 µs |
| D153 | 4.27 µs | 12.5 µs | 20.9 µs | 39.6 µs | 58.6 µs |
| D230 | 3.9 µs | 21.1 µs | 37 µs | 68.4 µs | 121 µs |
| D307 | 3.91 µs | 24.1 µs | 48.9 µs | 120 µs | 176 µs |
| D462 | 2.98 µs | 32.7 µs | 65.4 µs | 218 µs | 337 µs |
| D616 | 4.44 µs | 54.2 µs | 201 µs | 394 µs | 619 µs |
| D924 | 4.56 µs | 97.8 µs | 417 µs | 886 µs | 1.43 ms |
| D1232 | 4.38 µs | 159 µs | 662 µs | 1.45 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,103.4 124.4,106.6 160.5,106.3 196.7,107.4 232.9,106.3 269.1,107.4 305.3,107.4 341.5,110.7 377.6,105.8 413.8,105.5 450.0,105.9 450.0,23.6 413.8,34.1 377.6,44.5 341.5,52.1 305.3,60.1 269.1,64.8 232.9,73.8 196.7,80.3 160.5,87.4 124.4,103.4 88.2,91.5 52.0,111.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,103.4 124.4,106.6 160.5,106.3 196.7,107.4 232.9,106.3 269.1,107.4 305.3,107.4 341.5,110.7 377.6,105.8 413.8,105.5 450.0,105.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,97.1 124.4,100.0 160.5,98.1 196.7,96.5 232.9,92.9 269.1,86.5 305.3,84.8 341.5,81.0 377.6,74.7 413.8,67.4 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,101.7 124.4,96.3 160.5,93.1 196.7,85.7 232.9,86.6 269.1,79.5 305.3,76.0 341.5,72.4 377.6,58.5 413.8,49.4 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,94.8 124.4,92.3 160.5,89.0 196.7,82.1 232.9,78.6 269.1,71.9 305.3,64.9 341.5,57.5 377.6,50.1 413.8,40.1 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,91.5 124.4,103.4 160.5,87.4 196.7,80.3 232.9,73.8 269.1,64.8 305.3,60.1 341.5,52.1 377.6,44.5 413.8,34.1 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.81 µs | 9.4 µs | 10.3 µs | 6.22 µs |
| D38 | 3.74 ns | 9.4 µs | 11 µs | 13.9 µs | 16 µs |
| D57 | 625 ns | 5.47 µs | 6.9 µs | 8.53 µs | 11.8 µs |
| D76 | 611 ns | 6.53 µs | 7.97 µs | 11.8 µs | 15 µs |
| D115 | 1.05 µs | 11.9 µs | 19.1 µs | 27.4 µs | 30.1 µs |
| D153 | 1.24 µs | 13.5 µs | 21.9 µs | 36.3 µs | 53.8 µs |
| D230 | 1.36 µs | 23.4 µs | 41.2 µs | 77.5 µs | 128 µs |
| D307 | 2.13 µs | 39.9 µs | 78 µs | 152 µs | 242 µs |
| D462 | 2.21 µs | 56.8 µs | 82.5 µs | 296 µs | 496 µs |
| D616 | 3.96 µs | 114 µs | 311 µs | 563 µs | 951 µs |
| D924 | 5.71 µs | 214 µs | 710 µs | 1.47 ms | 2.5 ms |
| D1232 | 7.7 µs | 369 µs | 1.28 ms | 2.66 ms | 5.76 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,130.1 160.5,130.4 196.7,123.6 232.9,121.6 269.1,120.5 305.3,114.9 341.5,114.5 377.6,107.2 413.8,102.7 450.0,98.9 450.0,16.8 413.8,27.2 377.6,39.2 341.5,47.3 305.3,56.2 269.1,64.1 232.9,74.8 196.7,82.1 160.5,90.7 124.4,93.7 88.2,89.8 52.0,101.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,130.1 160.5,130.4 196.7,123.6 232.9,121.6 269.1,120.5 305.3,114.9 341.5,114.5 377.6,107.2 413.8,102.7 450.0,98.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.8 88.2,96.5 124.4,103.2 160.5,101.0 196.7,93.6 232.9,92.0 269.1,85.1 305.3,78.6 341.5,74.2 377.6,65.6 413.8,57.7 450.0,50.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,94.5 124.4,100.3 160.5,98.5 196.7,87.7 232.9,86.0 269.1,78.1 305.3,70.2 341.5,69.5 377.6,53.1 413.8,42.8 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,91.6 124.4,97.7 160.5,93.7 196.7,83.2 232.9,79.7 269.1,70.3 305.3,62.0 341.5,53.7 377.6,45.7 413.8,33.8 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.6 88.2,89.8 124.4,93.7 160.5,90.7 196.7,82.1 232.9,74.8 269.1,64.1 305.3,56.2 341.5,47.3 377.6,39.2 413.8,27.2 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.73 µs | 5.79 µs | 5.81 µs | 8.69 µs | 3.67 µs |
| D38 | 5.07 µs | 5.79 µs | 7.71 µs | 10.9 µs | 10.9 µs |
| D57 | 346 ns | 594 ns | 716 ns | 1.14 µs | 1.06 µs |
| D76 | 533 ns | 762 ns | 1.19 µs | 1.35 µs | 1.91 µs |
| D115 | 506 ns | 2.31 µs | 2.46 µs | 3.59 µs | 4.41 µs |
| D153 | 362 ns | 2.63 µs | 3.56 µs | 5.83 µs | 7.07 µs |
| D230 | 554 ns | 4.3 µs | 8.16 µs | 10.7 µs | 14.7 µs |
| D307 | 577 ns | 6.07 µs | 11.2 µs | 17.6 µs | 22.7 µs |
| D462 | 622 ns | 10.8 µs | 15.4 µs | 35.7 µs | 51.8 µs |
| D616 | 525 ns | 14.1 µs | 40.8 µs | 58.5 µs | 77.2 µs |
| D924 | 575 ns | 24.2 µs | 89.7 µs | 133 µs | 180 µs |
| D1232 | 1.22 µs | 39.6 µs | 150 µs | 227 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,122.1 88.2,124.7 124.4,183.0 160.5,173.7 196.7,174.8 232.9,182.1 269.1,172.8 305.3,171.9 341.5,170.3 377.6,174.0 413.8,172.0 450.0,155.8 450.0,33.1 413.8,47.2 377.6,65.6 341.5,74.3 305.3,92.2 269.1,101.6 232.9,117.5 196.7,127.8 160.5,146.0 124.4,158.8 88.2,108.2 52.0,131.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,122.1 88.2,124.7 124.4,183.0 160.5,173.7 196.7,174.8 232.9,182.1 269.1,172.8 305.3,171.9 341.5,170.3 377.6,174.0 413.8,172.0 450.0,155.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,121.9 124.4,171.3 160.5,165.9 196.7,141.8 232.9,139.0 269.1,128.3 305.3,120.8 341.5,108.3 377.6,102.5 413.8,90.8 450.0,80.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.8 88.2,115.7 124.4,167.2 160.5,156.2 196.7,140.5 232.9,132.4 269.1,114.4 305.3,107.4 341.5,100.7 377.6,79.5 413.8,62.4 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,108.1 124.4,157.2 160.5,153.4 196.7,132.2 232.9,121.7 269.1,108.5 305.3,97.7 341.5,82.4 377.6,71.6 413.8,53.8 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.8 88.2,108.2 124.4,158.8 160.5,146.0 196.7,127.8 232.9,117.5 269.1,101.6 305.3,92.2 341.5,74.3 377.6,65.6 413.8,47.2 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 4.26 µs | 5.96 µs | 6.41 µs | 3.85 µs |
| D38 | 4.98 ns | 5.96 µs | 6.75 µs | 8.6 µs | 9.7 µs |
| D57 | 2.81 ns | 3.32 µs | 4.34 µs | 5.39 µs | 9.47 µs |
| D76 | 3.14 ns | 3.93 µs | 5.15 µs | 7.44 µs | 9.37 µs |
| D115 | 17.4 ns | 4.4 µs | 9.96 µs | 14.8 µs | 16.4 µs |
| D153 | 22.4 ns | 5.32 µs | 9.48 µs | 19.8 µs | 31.7 µs |
| D230 | 48.8 ns | 10.4 µs | 18 µs | 39.5 µs | 72.3 µs |
| D307 | 81.7 ns | 11.8 µs | 25.1 µs | 73.6 µs | 115 µs |
| D462 | 129 ns | 16.1 µs | 36.2 µs | 139 µs | 230 µs |
| D616 | 168 ns | 30.3 µs | 129 µs | 255 µs | 429 µs |
| D924 | 229 ns | 59.7 µs | 270 µs | 611 µs | 1.03 ms |
| D1232 | 409 ns | 102 µs | 451 µs | 1.04 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,190.1 124.4,197.2 160.5,195.8 196.7,174.5 232.9,171.4 269.1,161.7 305.3,155.4 341.5,149.7 377.6,146.4 413.8,142.6 450.0,135.4 450.0,27.5 413.8,38.1 377.6,49.1 341.5,56.8 305.3,65.4 269.1,71.2 232.9,81.4 196.7,89.6 160.5,96.5 124.4,96.4 88.2,96.1 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,190.1 124.4,197.2 160.5,195.8 196.7,174.5 232.9,171.4 269.1,161.7 305.3,155.4 341.5,149.7 377.6,146.4 413.8,142.6 450.0,135.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.3 88.2,102.1 124.4,109.4 160.5,107.3 196.7,105.9 232.9,103.5 269.1,95.3 305.3,93.6 341.5,89.8 377.6,81.9 413.8,73.5 450.0,66.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.1 88.2,100.6 124.4,106.1 160.5,103.9 196.7,95.8 232.9,96.4 269.1,88.4 305.3,84.3 341.5,79.8 377.6,64.0 413.8,54.8 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,97.6 124.4,103.4 160.5,99.4 196.7,90.8 232.9,87.2 269.1,78.7 305.3,71.0 341.5,63.0 377.6,55.5 413.8,44.7 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,96.1 124.4,96.4 160.5,96.5 196.7,89.6 232.9,81.4 269.1,71.2 305.3,65.4 341.5,56.8 377.6,49.1 413.8,38.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.92 µs | 10.2 µs | 11 µs | 6.34 µs |
| D38 | 4.05 ns | 10.2 µs | 11.5 µs | 14.5 µs | 16.5 µs |
| D57 | 2.48 ns | 5.36 µs | 6.98 µs | 7.83 µs | 10.8 µs |
| D76 | 3.17 ns | 6.15 µs | 7.37 µs | 10.9 µs | 12.8 µs |
| D115 | 9.97 ns | 11.9 µs | 12.2 µs | 23 µs | 23.7 µs |
| D153 | 21.6 ns | 7.47 µs | 14.9 µs | 24.4 µs | 38.5 µs |
| D230 | 48.8 ns | 13.4 µs | 22.3 µs | 47.2 µs | 83.9 µs |
| D307 | 82.5 ns | 15.9 µs | 48.5 µs | 84.7 µs | 123 µs |
| D462 | 131 ns | 22.9 µs | 45.5 µs | 165 µs | 246 µs |
| D616 | 169 ns | 36.2 µs | 141 µs | 269 µs | 412 µs |
| D924 | 201 ns | 68.7 µs | 288 µs | 608 µs | 909 µs |
| D1232 | 397 ns | 113 µs | 446 µs | 911 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.7 124.4,198.7 160.5,195.7 196.7,181.5 232.9,171.9 269.1,161.8 305.3,155.2 341.5,149.5 377.6,146.3 413.8,144.2 450.0,135.8 450.0,25.6 413.8,39.8 377.6,49.6 341.5,56.0 305.3,64.6 269.1,69.3 232.9,79.0 196.7,85.0 160.5,92.6 124.4,94.7 88.2,89.5 52.0,101.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.7 124.4,198.7 160.5,195.7 196.7,181.5 232.9,171.9 269.1,161.8 305.3,155.2 341.5,149.5 377.6,146.3 413.8,144.2 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.5 124.4,103.5 160.5,101.7 196.7,93.6 232.9,99.3 269.1,92.1 305.3,90.0 341.5,85.4 377.6,79.8 413.8,71.8 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,93.9 124.4,100.2 160.5,99.5 196.7,93.2 232.9,90.8 269.1,85.7 305.3,76.1 341.5,76.9 377.6,62.9 413.8,54.0 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,91.1 124.4,98.7 160.5,94.7 196.7,85.4 232.9,84.7 269.1,76.4 305.3,69.2 341.5,61.0 377.6,54.9 413.8,44.7 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,89.5 124.4,94.7 160.5,92.6 196.7,85.0 232.9,79.0 269.1,69.3 305.3,64.6 341.5,56.0 377.6,49.6 413.8,39.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.5 ns | 6.32 ns | 6.34 ns | 9.2 ns | 9.21 ns |
| D38 | 9.55 ns | 12.4 ns | 15.2 ns | 809 ns | 1.07 µs |
| D57 | 35.3 ns | 49.2 ns | 68.5 ns | 100 ns | 107 ns |
| D76 | 41.1 ns | 65.9 ns | 76.1 ns | 106 ns | 132 ns |
| D115 | 54.9 ns | 82.3 ns | 110 ns | 190 ns | 201 ns |
| D153 | 68 ns | 108 ns | 143 ns | 239 ns | 319 ns |
| D230 | 96 ns | 154 ns | 227 ns | 403 ns | 586 ns |
| D307 | 130 ns | 220 ns | 357 ns | 601 ns | 950 ns |
| D462 | 229 ns | 409 ns | 433 ns | 1.1 µs | 1.44 µs |
| D616 | 259 ns | 601 ns | 1.06 µs | 1.76 µs | 2.23 µs |
| D924 | 429 ns | 942 ns | 2.24 µs | 2.8 µs | 4.44 µs |
| D1232 | 482 ns | 1.56 µs | 3.76 µs | 4.16 µs | 7.89 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.4 88.2,161.0 124.4,132.6 160.5,129.3 196.7,123.0 232.9,118.4 269.1,110.9 305.3,104.2 341.5,92.0 377.6,89.4 413.8,78.4 450.0,75.8 450.0,15.1 413.8,27.6 377.6,42.6 341.5,52.1 305.3,61.1 269.1,71.6 232.9,84.8 196.7,94.8 160.5,103.9 124.4,108.6 88.2,58.6 52.0,161.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.4 88.2,161.0 124.4,132.6 160.5,129.3 196.7,123.0 232.9,118.4 269.1,110.9 305.3,104.2 341.5,92.0 377.6,89.4 413.8,78.4 450.0,75.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,155.4 124.4,125.4 160.5,119.1 196.7,114.2 232.9,108.2 269.1,100.6 305.3,92.9 341.5,79.4 377.6,71.0 413.8,61.3 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.9 88.2,150.9 124.4,118.2 160.5,115.9 196.7,108.0 232.9,102.2 269.1,92.2 305.3,82.4 341.5,78.2 377.6,58.8 413.8,42.4 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,64.6 124.4,110.0 160.5,108.8 196.7,96.0 232.9,91.1 269.1,79.7 305.3,71.1 341.5,57.8 377.6,47.7 413.8,37.7 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,58.6 124.4,108.6 160.5,103.9 196.7,94.8 232.9,84.8 269.1,71.6 305.3,61.1 341.5,52.1 377.6,42.6 413.8,27.6 450.0,15.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 4.79 µs | 6.78 µs | 7.32 µs | 4.63 µs |
| D38 | 1.87 ns | 6.77 µs | 7.57 µs | 9.5 µs | 10.7 µs |
| D57 | 2.81 ns | 3.55 µs | 4.04 µs | 5.32 µs | 9.63 µs |
| D76 | 3.2 ns | 6.05 µs | 6.75 µs | 9.74 µs | 11.8 µs |
| D115 | 17.4 ns | 6.37 µs | 13.5 µs | 18.6 µs | 19.9 µs |
| D153 | 21.7 ns | 6.9 µs | 14.8 µs | 23.1 µs | 37.1 µs |
| D230 | 51.4 ns | 12.6 µs | 21.6 µs | 45.9 µs | 82.2 µs |
| D307 | 84.3 ns | 14.7 µs | 34.5 µs | 83.9 µs | 122 µs |
| D462 | 128 ns | 21.8 µs | 45.5 µs | 163 µs | 243 µs |
| D616 | 175 ns | 34.3 µs | 139 µs | 265 µs | 408 µs |
| D924 | 205 ns | 68.5 µs | 285 µs | 601 µs | 898 µs |
| D1232 | 422 ns | 109 µs | 440 µs | 902 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,202.2 124.4,197.2 160.5,195.6 196.7,174.5 232.9,171.8 269.1,161.1 305.3,155.0 341.5,149.8 377.6,145.9 413.8,144.0 450.0,135.0 450.0,25.6 413.8,39.9 377.6,49.7 341.5,56.1 305.3,64.7 269.1,69.6 232.9,79.4 196.7,87.2 160.5,93.7 124.4,96.2 88.2,94.8 52.0,105.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,202.2 124.4,197.2 160.5,195.6 196.7,174.5 232.9,171.8 269.1,161.1 305.3,155.0 341.5,149.8 377.6,145.9 413.8,144.0 450.0,135.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.8 88.2,100.5 124.4,108.6 160.5,101.9 196.7,101.3 232.9,100.3 269.1,92.8 305.3,90.9 341.5,86.1 377.6,80.4 413.8,71.8 450.0,66.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.5 88.2,99.2 124.4,107.0 160.5,100.6 196.7,92.0 232.9,90.8 269.1,86.2 305.3,80.4 341.5,76.9 377.6,63.0 413.8,54.2 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,96.4 124.4,103.5 160.5,96.0 196.7,88.0 232.9,85.3 269.1,76.8 305.3,69.3 341.5,61.1 377.6,55.1 413.8,44.9 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,94.8 124.4,96.2 160.5,93.7 196.7,87.2 232.9,79.4 269.1,69.6 305.3,64.7 341.5,56.1 377.6,49.7 413.8,39.9 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.2 ns | 39.7 ns | 42.9 ns | 40.3 ns | 38.1 ns |
| D38 | 16 ns | 42.9 ns | 67 ns | 75.4 ns | 108 ns |
| D57 | 16.5 ns | 39.9 ns | 67.3 ns | 692 ns | 631 ns |
| D76 | 17.3 ns | 73 ns | 692 ns | 607 ns | 899 ns |
| D115 | 22.1 ns | 73 ns | 621 ns | 1.08 µs | 1.09 µs |
| D153 | 23.1 ns | 696 ns | 1.05 µs | 1.34 µs | 2.1 µs |
| D230 | 29.3 ns | 736 ns | 1.52 µs | 2.29 µs | 3.19 µs |
| D307 | 42.2 ns | 1.09 µs | 2.13 µs | 3.24 µs | 5.6 µs |
| D462 | 77.3 ns | 1.51 µs | 2.24 µs | 6.39 µs | 9.04 µs |
| D616 | 73.9 ns | 2.42 µs | 6.16 µs | 10.8 µs | 15.9 µs |
| D924 | 117 ns | 2.91 µs | 11.3 µs | 24.8 µs | 28.5 µs |
| D1232 | 107 ns | 5.03 µs | 20.7 µs | 27.5 µs | 50.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,199.8 124.4,199.1 160.5,198.1 196.7,192.8 232.9,191.8 269.1,186.7 305.3,178.7 341.5,165.6 377.6,166.6 413.8,156.7 450.0,158.5 450.0,24.7 413.8,37.2 377.6,49.9 341.5,62.2 305.3,72.6 269.1,84.8 232.9,93.9 196.7,108.1 160.5,112.3 124.4,120.0 88.2,158.4 52.0,181.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,199.8 124.4,199.1 160.5,198.1 196.7,192.8 232.9,191.8 269.1,186.7 305.3,178.7 341.5,165.6 377.6,166.6 413.8,156.7 450.0,158.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,178.4 124.4,179.9 160.5,166.8 196.7,166.8 232.9,117.9 269.1,116.7 305.3,108.2 341.5,101.1 377.6,90.9 413.8,86.8 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,168.7 124.4,168.6 160.5,118.0 196.7,120.3 232.9,108.9 269.1,100.9 305.3,93.6 341.5,92.5 377.6,70.5 413.8,57.3 450.0,44.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,166.1 124.4,118.0 160.5,120.8 196.7,108.4 232.9,103.6 269.1,92.0 305.3,84.5 341.5,69.7 377.6,58.4 413.8,40.3 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.0 88.2,158.4 124.4,120.0 160.5,112.3 196.7,108.1 232.9,93.9 269.1,84.8 305.3,72.6 341.5,62.2 377.6,49.9 413.8,37.2 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 301 ns | 388 ns | 396 ns | 357 ns |
| D38 | 2.18 ns | 391 ns | 369 ns | 401 ns | 407 ns |
| D57 | 275 ns | 438 ns | 436 ns | 441 ns | 625 ns |
| D76 | 281 ns | 479 ns | 442 ns | 633 ns | 651 ns |
| D115 | 287 ns | 436 ns | 639 ns | 1.01 µs | 831 ns |
| D153 | 339 ns | 464 ns | 599 ns | 1.08 µs | 1.36 µs |
| D230 | 474 ns | 606 ns | 970 ns | 1.35 µs | 1.73 µs |
| D307 | 651 ns | 656 ns | 1.03 µs | 1.37 µs | 10.1 µs |
| D462 | 1.17 µs | 2.75 µs | 2.05 µs | 4.13 µs | 5.14 µs |
| D616 | 1.51 µs | 1.38 µs | 1.87 µs | 2.64 µs | 3.41 µs |
| D924 | 2 µs | 1.7 µs | 2.85 µs | 3.54 µs | 4.28 µs |
| D1232 | 2.97 µs | 2.43 µs | 4 µs | 4.58 µs | 6.48 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,196.5 124.4,112.4 160.5,112.1 196.7,111.7 232.9,108.8 269.1,103.0 305.3,97.5 341.5,87.3 377.6,82.8 413.8,78.0 450.0,71.1 450.0,57.5 413.8,64.7 377.6,68.7 341.5,61.6 305.3,49.9 269.1,80.5 232.9,84.7 196.7,93.2 160.5,97.5 124.4,98.2 88.2,105.6 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,196.5 124.4,112.4 160.5,112.1 196.7,111.7 232.9,108.8 269.1,103.0 305.3,97.5 341.5,87.3 377.6,82.8 413.8,78.0 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,110.9 88.2,106.3 124.4,104.3 160.5,102.8 196.7,104.4 232.9,103.4 269.1,98.7 305.3,97.3 341.5,72.4 377.6,84.4 413.8,80.8 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,107.3 124.4,104.4 160.5,104.2 196.7,97.8 232.9,98.9 269.1,90.5 305.3,89.5 341.5,77.5 377.6,79.2 413.8,71.8 450.0,65.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,105.9 124.4,104.2 160.5,97.9 196.7,89.8 232.9,88.6 269.1,84.8 305.3,84.5 341.5,65.4 377.6,73.1 413.8,68.0 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,105.6 124.4,98.2 160.5,97.5 196.7,93.2 232.9,84.7 269.1,80.5 305.3,49.9 341.5,61.6 377.6,68.7 413.8,64.7 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.16 µs | 9.23 µs | 12.4 µs | 13.6 µs | 8.82 µs |
| D38 | 7.73 µs | 12.4 µs | 14.5 µs | 18.3 µs | 21.2 µs |
| D57 | 4.5 µs | 4.14 µs | 4.32 µs | 4.38 µs | 5.22 µs |
| D76 | 4.49 µs | 4.85 µs | 4.36 µs | 5.09 µs | 5.38 µs |
| D115 | 7.57 µs | 8.15 µs | 9.34 µs | 10.4 µs | 8.65 µs |
| D153 | 8.14 µs | 8.32 µs | 8.88 µs | 10.7 µs | 11.6 µs |
| D230 | 10.3 µs | 12 µs | 13.9 µs | 15.7 µs | 17.7 µs |
| D307 | 15.9 µs | 18.3 µs | 21.4 µs | 25.3 µs | 28.2 µs |
| D462 | 15.8 µs | 20.3 µs | 14 µs | 29.5 µs | 33.8 µs |
| D616 | 28.9 µs | 39.7 µs | 55.7 µs | 61 µs | 71.4 µs |
| D924 | 42.4 µs | 58 µs | 104 µs | 134 µs | 149 µs |
| D1232 | 54.3 µs | 89.7 µs | 166 µs | 200 µs | 270 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.2 88.2,150.8 124.4,166.5 160.5,166.5 196.7,151.4 232.9,149.3 269.1,142.5 305.3,129.9 341.5,130.1 377.6,112.6 413.8,101.5 450.0,94.4 450.0,48.0 413.8,65.1 377.6,86.4 341.5,108.1 305.3,113.3 269.1,126.9 232.9,139.0 196.7,147.5 160.5,161.3 124.4,162.2 88.2,121.6 52.0,147.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.2 88.2,150.8 124.4,166.5 160.5,166.5 196.7,151.4 232.9,149.3 269.1,142.5 305.3,129.9 341.5,130.1 377.6,112.6 413.8,101.5 450.0,94.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.7 88.2,137.1 124.4,168.9 160.5,164.3 196.7,149.3 232.9,148.7 269.1,138.0 305.3,125.8 341.5,122.8 377.6,103.4 413.8,92.5 450.0,79.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.1 88.2,132.5 124.4,167.6 160.5,167.3 196.7,145.3 232.9,146.8 269.1,133.8 305.3,121.4 341.5,133.7 377.6,93.6 413.8,75.4 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,125.8 124.4,167.3 160.5,162.9 196.7,142.2 232.9,141.5 269.1,130.3 305.3,116.4 341.5,112.0 377.6,91.0 413.8,68.2 450.0,56.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,147.0 88.2,121.6 124.4,162.2 160.5,161.3 196.7,147.5 232.9,139.0 269.1,126.9 305.3,113.3 341.5,108.1 377.6,86.4 413.8,65.1 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 3.08 ns | 3.22 ns | 5.01 ns | 5.25 ns |
| D38 | 3.5 ns | 13.7 ns | 26.2 ns | 33.6 ns | 29.4 ns |
| D57 | 4.22 ns | 20.9 ns | 32.9 ns | 71.5 ns | 77.4 ns |
| D76 | 5.64 ns | 38.1 ns | 42.2 ns | 85.4 ns | 107 ns |
| D115 | 13.2 ns | 47.5 ns | 89.8 ns | 214 ns | 214 ns |
| D153 | 16.8 ns | 52.7 ns | 112 ns | 258 ns | 396 ns |
| D230 | 27.2 ns | 122 ns | 337 ns | 573 ns | 1.05 µs |
| D307 | 44.4 ns | 169 ns | 459 ns | 1.1 µs | 1.45 µs |
| D462 | 83 ns | 416 ns | 706 ns | 1.85 µs | 2.63 µs |
| D616 | 104 ns | 643 ns | 1.86 µs | 2.71 µs | 3.87 µs |
| D924 | 159 ns | 1.26 µs | 3.18 µs | 5.44 µs | 7.51 µs |
| D1232 | 180 ns | 1.89 µs | 5.02 µs | 8.08 µs | 14.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,209.1 88.2,188.3 124.4,185.0 160.5,180.0 196.7,165.2 232.9,160.9 269.1,152.6 305.3,144.1 341.5,133.2 377.6,129.4 413.8,121.9 450.0,119.8 450.0,44.0 413.8,55.0 377.6,66.5 341.5,73.2 305.3,83.5 269.1,89.2 232.9,106.1 196.7,116.8 160.5,128.8 124.4,134.4 88.2,151.3 52.0,181.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,188.3 124.4,185.0 160.5,180.0 196.7,165.2 232.9,160.9 269.1,152.6 305.3,144.1 341.5,133.2 377.6,129.4 413.8,121.9 450.0,119.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.5 88.2,164.5 124.4,157.2 160.5,146.8 196.7,142.9 232.9,141.1 269.1,126.6 305.3,120.9 341.5,105.2 377.6,97.7 413.8,86.0 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.7 88.2,153.2 124.4,149.3 160.5,145.0 196.7,131.9 232.9,128.0 269.1,108.9 305.3,103.5 341.5,96.1 377.6,79.3 413.8,69.9 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,149.0 124.4,135.8 160.5,132.7 196.7,116.8 232.9,113.5 269.1,99.7 305.3,88.4 341.5,79.3 377.6,72.7 413.8,60.6 450.0,53.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.2 88.2,151.3 124.4,134.4 160.5,128.8 196.7,116.8 232.9,106.1 269.1,89.2 305.3,83.5 341.5,73.2 377.6,66.5 413.8,55.0 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.703 ns | 0.703 ns | 0.703 ns | 0.703 ns | 0.345 ns |
| D38 | 1.33 ns | 1.45 ns | 1.32 ns | 1.45 ns | 1.45 ns |
| D57 | 1.74 ns | 1.87 ns | 1.87 ns | 1.87 ns | 1.74 ns |
| D76 | 2.17 ns | 2.17 ns | 2.1 ns | 2.17 ns | 2.63 ns |
| D115 | 2.86 ns | 2.86 ns | 3.17 ns | 3.55 ns | 2.89 ns |
| D153 | 4.22 ns | 3.82 ns | 4.29 ns | 4.6 ns | 4.6 ns |
| D230 | 5.86 ns | 5.86 ns | 7.16 ns | 7.24 ns | 7.24 ns |
| D307 | 10.9 ns | 11.1 ns | 11.1 ns | 12.5 ns | 12.5 ns |
| D462 | 15.1 ns | 15.3 ns | 11.1 ns | 16.6 ns | 16.7 ns |
| D616 | 23 ns | 19.8 ns | 21.7 ns | 20.2 ns | 20.1 ns |
| D924 | 63.2 ns | 69.2 ns | 97 ns | 84.8 ns | 76.9 ns |
| D1232 | 47.1 ns | 60 ns | 69.8 ns | 61.5 ns | 69.8 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.5 88.2,135.1 124.4,127.3 160.5,121.0 196.7,112.9 232.9,101.6 269.1,92.1 305.3,74.1 341.5,64.8 377.6,52.5 413.8,23.3 450.0,31.8 450.0,20.4 413.8,17.6 377.6,56.4 341.5,61.9 305.3,70.2 269.1,86.0 232.9,99.2 196.7,112.6 160.5,115.3 124.4,127.3 88.2,132.6 52.0,174.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,135.1 124.4,127.3 160.5,121.0 196.7,112.9 232.9,101.6 269.1,92.1 305.3,74.1 341.5,64.8 377.6,52.5 413.8,23.3 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,132.7 124.4,125.2 160.5,120.9 196.7,112.9 232.9,104.5 269.1,92.1 305.3,73.7 341.5,64.3 377.6,56.8 413.8,20.7 450.0,24.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,125.2 160.5,121.9 196.7,109.9 232.9,101.2 269.1,86.3 305.3,73.7 341.5,73.7 377.6,54.2 413.8,10.9 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.7 124.4,125.2 160.5,120.9 196.7,106.6 232.9,99.1 269.1,86.0 305.3,70.2 341.5,61.9 377.6,56.3 413.8,14.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.1 88.2,132.6 124.4,127.3 160.5,115.3 196.7,112.6 232.9,99.2 269.1,86.0 305.3,70.2 341.5,61.9 377.6,56.4 413.8,17.6 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 5.31 µs | 7.48 µs | 8.02 µs | 5.24 µs |
| D38 | 6.24 ns | 7.45 µs | 8.22 µs | 10.2 µs | 11.5 µs |
| D57 | 64 ns | 4 µs | 4.3 µs | 4.52 µs | 5.47 µs |
| D76 | 78.3 ns | 4.39 µs | 4.54 µs | 5.45 µs | 5.88 µs |
| D115 | 136 ns | 8.05 µs | 9.53 µs | 11 µs | 9.7 µs |
| D153 | 191 ns | 8.46 µs | 9.56 µs | 11.8 µs | 13 µs |
| D230 | 284 ns | 12.3 µs | 14.9 µs | 19.2 µs | 22.1 µs |
| D307 | 367 ns | 18.9 µs | 20.9 µs | 29.7 µs | 34.3 µs |
| D462 | 618 ns | 69.2 µs | 78.4 µs | 220 µs | 281 µs |
| D616 | 859 ns | 177 µs | 353 µs | 353 µs | 525 µs |
| D924 | 967 ns | 352 µs | 486 µs | 846 µs | 1.62 ms |
| D1232 | 1.39 µs | 644 µs | 853 µs | 2.1 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,187.3 124.4,158.4 160.5,155.9 196.7,149.0 232.9,144.8 269.1,139.9 305.3,136.7 341.5,130.2 377.6,126.2 413.8,124.7 450.0,120.2 450.0,25.0 413.8,32.6 377.6,46.6 341.5,54.3 305.3,80.4 269.1,85.9 232.9,92.4 196.7,96.1 160.5,102.3 124.4,103.2 88.2,94.0 52.0,103.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,187.3 124.4,158.4 160.5,155.9 196.7,149.0 232.9,144.8 269.1,139.9 305.3,136.7 341.5,130.2 377.6,126.2 413.8,124.7 450.0,120.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,99.4 124.4,107.1 160.5,105.9 196.7,98.4 232.9,97.8 269.1,93.1 305.3,87.8 341.5,71.7 377.6,60.1 413.8,51.5 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.3 88.2,98.1 124.4,106.2 160.5,105.5 196.7,96.3 232.9,96.3 269.1,90.8 305.3,86.5 341.5,70.2 377.6,51.5 413.8,47.5 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,95.4 124.4,105.6 160.5,103.3 196.7,94.6 232.9,93.7 269.1,87.6 305.3,82.2 341.5,57.3 377.6,51.5 413.8,40.6 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.7 88.2,94.0 124.4,103.2 160.5,102.3 196.7,96.1 232.9,92.4 269.1,85.9 305.3,80.4 341.5,54.3 377.6,46.6 413.8,32.6 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.11 ns | 2.11 ns | 2.11 ns | 2.46 ns | 2.88 ns |
| D38 | 7.17 ns | 8.06 ns | 12.4 ns | 19.4 ns | 16.6 ns |
| D57 | 8.09 ns | 7.16 ns | 7.17 ns | 7.17 ns | 8.09 ns |
| D76 | 9.51 ns | 9.51 ns | 8.71 ns | 9.84 ns | 9.84 ns |
| D115 | 12.8 ns | 12.4 ns | 14.1 ns | 14.1 ns | 9.25 ns |
| D153 | 20.7 ns | 15.8 ns | 16 ns | 20.1 ns | 20 ns |
| D230 | 32.2 ns | 32.2 ns | 32.1 ns | 36.8 ns | 36.3 ns |
| D307 | 41.3 ns | 40.1 ns | 44 ns | 48.8 ns | 48.2 ns |
| D462 | 81 ns | 72.8 ns | 48.7 ns | 86.5 ns | 83.1 ns |
| D616 | 104 ns | 81.8 ns | 96 ns | 78 ns | 77.5 ns |
| D924 | 121 ns | 112 ns | 127 ns | 117 ns | 94.2 ns |
| D1232 | 131 ns | 116 ns | 130 ns | 132 ns | 123 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.4 88.2,152.9 124.4,149.5 160.5,144.8 196.7,136.3 232.9,122.2 269.1,109.5 305.3,102.3 341.5,82.8 377.6,75.5 413.8,71.0 450.0,68.8 450.0,70.7 413.8,78.4 377.6,84.1 341.5,82.0 305.3,97.8 269.1,106.0 232.9,123.2 196.7,145.6 160.5,143.8 124.4,149.5 88.2,128.7 52.0,179.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,152.9 124.4,149.5 160.5,144.8 196.7,136.3 232.9,122.2 269.1,109.5 305.3,102.3 341.5,82.8 377.6,75.5 413.8,71.0 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,149.6 124.4,153.0 160.5,144.8 196.7,137.0 232.9,130.0 269.1,109.4 305.3,103.1 341.5,85.8 377.6,82.5 413.8,73.4 450.0,72.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,137.1 124.4,153.0 160.5,147.3 196.7,133.5 232.9,129.8 269.1,109.5 305.3,100.4 341.5,97.5 377.6,77.9 413.8,69.7 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,124.2 124.4,153.0 160.5,143.8 196.7,133.5 232.9,123.2 269.1,105.6 305.3,97.4 341.5,80.9 377.6,83.9 413.8,72.2 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.4 88.2,128.7 124.4,149.5 160.5,143.8 196.7,145.6 232.9,123.2 269.1,106.0 305.3,97.8 341.5,82.0 377.6,84.1 413.8,78.4 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 4.12 µs | 5.76 µs | 6.33 µs | 3.8 µs |
| D38 | 4.36 ns | 5.76 µs | 6.57 µs | 8.39 µs | 9.63 µs |
| D57 | 2.79 ns | 3.09 µs | 4.02 µs | 5.3 µs | 9.47 µs |
| D76 | 3.87 ns | 3.68 µs | 4.85 µs | 7.32 µs | 8.95 µs |
| D115 | 17.4 ns | 4.12 µs | 9.86 µs | 15 µs | 15.8 µs |
| D153 | 22.5 ns | 5.02 µs | 9.83 µs | 18.5 µs | 31.2 µs |
| D230 | 48.8 ns | 9.59 µs | 18.3 µs | 39.2 µs | 71 µs |
| D307 | 76.9 ns | 11.4 µs | 24.3 µs | 71.1 µs | 113 µs |
| D462 | 128 ns | 15 µs | 36.2 µs | 138 µs | 228 µs |
| D616 | 162 ns | 29.4 µs | 129 µs | 253 µs | 425 µs |
| D924 | 192 ns | 59.9 µs | 267 µs | 608 µs | 1.03 ms |
| D1232 | 408 ns | 101 µs | 452 µs | 1.04 ms | 2.42 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,191.7 124.4,197.3 160.5,193.2 196.7,174.5 232.9,171.4 269.1,161.8 305.3,156.1 341.5,149.8 377.6,146.9 413.8,144.8 450.0,135.4 450.0,27.6 413.8,38.2 377.6,49.2 341.5,56.9 305.3,65.6 269.1,71.4 232.9,81.6 196.7,90.1 160.5,97.1 124.4,96.4 88.2,96.2 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,191.7 124.4,197.3 160.5,193.2 196.7,174.5 232.9,171.4 269.1,161.8 305.3,156.1 341.5,149.8 377.6,146.9 413.8,144.8 450.0,135.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.7 88.2,102.6 124.4,110.3 160.5,108.1 196.7,106.7 232.9,104.3 269.1,96.2 305.3,94.1 341.5,90.6 377.6,82.3 413.8,73.5 450.0,67.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.6 88.2,100.9 124.4,107.0 160.5,104.7 196.7,95.9 232.9,95.9 269.1,88.2 305.3,84.7 341.5,79.7 377.6,64.0 413.8,55.0 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,97.9 124.4,103.6 160.5,99.6 196.7,90.7 232.9,88.1 269.1,78.8 305.3,71.4 341.5,63.1 377.6,55.6 413.8,44.8 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,96.2 124.4,96.4 160.5,97.1 196.7,90.1 232.9,81.6 269.1,71.4 305.3,65.6 341.5,56.9 377.6,49.2 413.8,38.2 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.93 µs | 10.2 µs | 11 µs | 6.34 µs |
| D38 | 4.05 ns | 10.2 µs | 11.5 µs | 14.5 µs | 16.5 µs |
| D57 | 12.2 ns | 5.28 µs | 6.96 µs | 7.84 µs | 10.9 µs |
| D76 | 12.1 ns | 6.13 µs | 7.38 µs | 10.9 µs | 12.8 µs |
| D115 | 10.3 ns | 11.7 µs | 12.2 µs | 23.3 µs | 23.5 µs |
| D153 | 20.8 ns | 7.52 µs | 14.9 µs | 24.1 µs | 39.1 µs |
| D230 | 49.6 ns | 13.5 µs | 22.5 µs | 47 µs | 84.3 µs |
| D307 | 76.6 ns | 16 µs | 48.5 µs | 84.8 µs | 123 µs |
| D462 | 126 ns | 23.4 µs | 44.2 µs | 165 µs | 247 µs |
| D616 | 161 ns | 37.1 µs | 142 µs | 269 µs | 411 µs |
| D924 | 203 ns | 68.9 µs | 289 µs | 608 µs | 901 µs |
| D1232 | 414 ns | 114 µs | 446 µs | 909 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.7 124.4,179.0 160.5,179.0 196.7,181.1 232.9,172.4 269.1,161.6 305.3,156.2 341.5,150.0 377.6,146.9 413.8,144.0 450.0,135.2 450.0,25.6 413.8,39.9 377.6,49.6 341.5,55.9 305.3,64.6 269.1,69.3 232.9,78.8 196.7,85.1 160.5,92.6 124.4,94.7 88.2,89.5 52.0,101.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.7 124.4,179.0 160.5,179.0 196.7,181.1 232.9,172.4 269.1,161.6 305.3,156.2 341.5,150.0 377.6,146.9 413.8,144.0 450.0,135.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.5 124.4,103.6 160.5,101.8 196.7,93.8 232.9,99.3 269.1,92.0 305.3,89.9 341.5,85.2 377.6,79.4 413.8,71.8 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,93.9 124.4,100.2 160.5,99.5 196.7,93.2 232.9,90.8 269.1,85.6 305.3,76.1 341.5,77.3 377.6,62.8 413.8,54.0 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,91.1 124.4,98.7 160.5,94.7 196.7,85.2 232.9,84.8 269.1,76.5 305.3,69.2 341.5,60.9 377.6,54.9 413.8,44.7 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,89.5 124.4,94.7 160.5,92.6 196.7,85.1 232.9,78.8 269.1,69.3 305.3,64.6 341.5,55.9 377.6,49.6 413.8,39.9 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.96 ns | 16.5 ns | 19.9 ns | 30.4 ns | 34.3 ns |
| D38 | 7.5 ns | 36.5 ns | 31.1 ns | 1.64 µs | 3.19 µs |
| D57 | 176 ns | 206 ns | 486 ns | 765 ns | 670 ns |
| D76 | 209 ns | 277 ns | 771 ns | 720 ns | 1.05 µs |
| D115 | 115 ns | 661 ns | 932 ns | 1.52 µs | 1.41 µs |
| D153 | 128 ns | 1.11 µs | 1.59 µs | 1.9 µs | 2.65 µs |
| D230 | 150 ns | 1.54 µs | 2.44 µs | 3.43 µs | 4.18 µs |
| D307 | 147 ns | 2.33 µs | 3.62 µs | 4.88 µs | 7.31 µs |
| D462 | 189 ns | 3.66 µs | 3.65 µs | 9.82 µs | 11.6 µs |
| D616 | 247 ns | 5.72 µs | 11.1 µs | 14.2 µs | 20.6 µs |
| D924 | 247 ns | 8.91 µs | 17.1 µs | 26.8 µs | 34.8 µs |
| D1232 | 268 ns | 13 µs | 28.4 µs | 39.3 µs | 62 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.3 88.2,175.0 124.4,120.2 160.5,117.2 196.7,127.6 232.9,125.8 269.1,122.9 305.3,123.3 341.5,118.9 377.6,114.3 413.8,114.3 450.0,112.9 450.0,18.3 413.8,28.3 377.6,37.4 341.5,47.5 305.3,55.4 269.1,65.1 232.9,73.0 196.7,84.0 160.5,89.2 124.4,97.0 88.2,69.8 52.0,148.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,176.3 88.2,175.0 124.4,120.2 160.5,117.2 196.7,127.6 232.9,125.8 269.1,122.9 305.3,123.3 341.5,118.9 377.6,114.3 413.8,114.3 450.0,112.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.3 88.2,147.5 124.4,117.5 160.5,112.3 196.7,97.2 232.9,88.2 269.1,82.5 305.3,75.3 341.5,67.5 377.6,59.7 413.8,52.0 450.0,45.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.0 88.2,150.3 124.4,102.5 160.5,94.5 196.7,91.2 232.9,81.9 269.1,74.5 305.3,67.6 341.5,67.5 377.6,48.1 413.8,40.6 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.7 88.2,81.4 124.4,94.7 160.5,95.7 196.7,82.8 232.9,78.9 269.1,68.6 305.3,62.5 341.5,50.3 377.6,43.9 413.8,32.9 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,148.6 88.2,69.8 124.4,97.0 160.5,89.2 196.7,84.0 232.9,73.0 269.1,65.1 305.3,55.4 341.5,47.5 377.6,37.4 413.8,28.3 450.0,18.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.05 ns | 1.05 ns | 1.06 ns | 1.06 ns | 1.08 ns |
| D38 | 1.62 ns | 1.81 ns | 1.6 ns | 1.81 ns | 1.82 ns |
| D57 | 2.5 ns | 2.26 ns | 2.25 ns | 2.25 ns | 2.5 ns |
| D76 | 3.46 ns | 3.45 ns | 3.09 ns | 3.46 ns | 3.45 ns |
| D115 | 4.83 ns | 4.84 ns | 5.58 ns | 5.55 ns | 4.03 ns |
| D153 | 8.43 ns | 7.65 ns | 7.63 ns | 8.48 ns | 8.43 ns |
| D230 | 16.1 ns | 16.1 ns | 16.1 ns | 17.6 ns | 17.7 ns |
| D307 | 23.5 ns | 23.5 ns | 23.5 ns | 25.2 ns | 25.2 ns |
| D462 | 46.4 ns | 37.4 ns | 26.5 ns | 40.4 ns | 43.4 ns |
| D616 | 48.8 ns | 45.8 ns | 59.8 ns | 45.9 ns | 46.1 ns |
| D924 | 84.8 ns | 72.5 ns | 89.6 ns | 85 ns | 75.6 ns |
| D1232 | 98.2 ns | 90.8 ns | 106 ns | 95.8 ns | 106 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,208.4 88.2,196.1 124.4,183.5 160.5,174.0 196.7,164.4 232.9,148.3 269.1,129.5 305.3,118.6 341.5,98.9 377.6,97.4 413.8,81.4 450.0,77.2 450.0,75.0 413.8,84.8 377.6,99.1 341.5,100.9 305.3,116.6 269.1,126.9 232.9,148.3 196.7,169.7 160.5,174.1 124.4,183.5 88.2,192.7 52.0,207.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.4 88.2,196.1 124.4,183.5 160.5,174.0 196.7,164.4 232.9,148.3 269.1,129.5 305.3,118.6 341.5,98.9 377.6,97.4 413.8,81.4 450.0,77.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,208.4 88.2,192.8 124.4,186.4 160.5,174.1 196.7,164.3 232.9,151.1 269.1,129.6 305.3,118.6 341.5,105.2 377.6,99.3 413.8,86.0 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,208.4 88.2,196.4 124.4,186.5 160.5,177.3 196.7,160.2 232.9,151.1 269.1,129.5 305.3,118.6 341.5,115.1 377.6,91.5 413.8,79.9 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,208.4 88.2,192.8 124.4,186.5 160.5,174.1 196.7,160.4 232.9,148.1 269.1,126.9 305.3,116.6 341.5,102.9 377.6,99.2 413.8,81.4 450.0,77.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,207.6 88.2,192.7 124.4,183.5 160.5,174.1 196.7,169.7 232.9,148.3 269.1,126.9 305.3,116.6 341.5,100.9 377.6,99.1 413.8,84.8 450.0,75.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 8.25 µs | 10.7 µs | 11.6 µs | 7.09 µs |
| D38 | 4.36 ns | 10.7 µs | 12.2 µs | 15.6 µs | 17.7 µs |
| D57 | 3.17 ns | 4.14 µs | 5.44 µs | 7.01 µs | 9.42 µs |
| D76 | 4.22 ns | 4.9 µs | 6.51 µs | 9.42 µs | 11.3 µs |
| D115 | 17.5 ns | 5.6 µs | 12.7 µs | 17.1 µs | 19.4 µs |
| D153 | 22.3 ns | 6.75 µs | 11.8 µs | 23.6 µs | 36.3 µs |
| D230 | 47.3 ns | 12.1 µs | 22 µs | 44.8 µs | 81.1 µs |
| D307 | 74.6 ns | 14.2 µs | 28.7 µs | 79.2 µs | 125 µs |
| D462 | 188 ns | 17.8 µs | 40.7 µs | 154 µs | 251 µs |
| D616 | 156 ns | 33.8 µs | 140 µs | 280 µs | 460 µs |
| D924 | 157 ns | 66.6 µs | 290 µs | 653 µs | 1.1 ms |
| D1232 | 398 ns | 111 µs | 487 µs | 1.1 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,191.7 124.4,195.7 160.5,192.1 196.7,174.5 232.9,171.5 269.1,162.2 305.3,156.5 341.5,145.0 377.6,147.3 413.8,147.2 450.0,135.7 450.0,26.9 413.8,37.4 377.6,48.2 341.5,55.7 305.3,64.3 269.1,69.7 232.9,79.7 196.7,87.5 160.5,94.2 124.4,96.5 88.2,88.6 52.0,100.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,191.7 124.4,195.7 160.5,192.1 196.7,174.5 232.9,171.5 269.1,162.2 305.3,156.5 341.5,145.0 377.6,147.3 413.8,147.2 450.0,135.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.1 88.2,94.8 124.4,106.7 160.5,104.6 196.7,102.9 232.9,100.6 269.1,93.4 305.3,91.3 341.5,88.5 377.6,80.6 413.8,72.2 450.0,65.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,93.2 124.4,103.3 160.5,101.0 196.7,92.7 232.9,93.6 269.1,85.9 305.3,82.6 341.5,78.3 377.6,63.0 413.8,53.9 450.0,47.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,90.2 124.4,100.1 160.5,96.5 196.7,89.0 232.9,85.1 269.1,77.1 305.3,70.0 341.5,61.8 377.6,54.4 413.8,43.9 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.0 88.2,88.6 124.4,96.5 160.5,94.2 196.7,87.5 232.9,79.7 269.1,69.7 305.3,64.3 341.5,55.7 377.6,48.2 413.8,37.4 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.93 µs | 10.3 µs | 11.2 µs | 6.49 µs |
| D38 | 3.74 ns | 10.3 µs | 11.5 µs | 14.6 µs | 16.6 µs |
| D57 | 2.83 µs | 5.32 µs | 7.29 µs | 8.17 µs | 11.2 µs |
| D76 | 2.85 µs | 6.2 µs | 7.72 µs | 11.2 µs | 13.1 µs |
| D115 | 5.46 µs | 12.3 µs | 12.8 µs | 23.4 µs | 24.6 µs |
| D153 | 3.09 µs | 7.86 µs | 15.5 µs | 24.6 µs | 39.2 µs |
| D230 | 2.89 µs | 14.1 µs | 22.8 µs | 48.2 µs | 85.3 µs |
| D307 | 3.15 µs | 16.5 µs | 50.6 µs | 86 µs | 124 µs |
| D462 | 3.33 µs | 23.8 µs | 46.1 µs | 167 µs | 249 µs |
| D616 | 3.78 µs | 36.7 µs | 144 µs | 272 µs | 415 µs |
| D924 | 4.1 µs | 69.5 µs | 291 µs | 612 µs | 917 µs |
| D1232 | 4.41 µs | 115 µs | 449 µs | 914 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,111.4 160.5,111.3 196.7,103.2 232.9,110.3 269.1,111.1 305.3,110.1 341.5,109.4 377.6,107.8 413.8,106.8 450.0,105.9 450.0,25.5 413.8,39.6 377.6,49.5 341.5,55.8 305.3,64.5 269.1,69.1 232.9,78.8 196.7,84.6 160.5,92.3 124.4,94.3 88.2,89.5 52.0,101.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,111.4 160.5,111.3 196.7,103.2 232.9,110.3 269.1,111.1 305.3,110.1 341.5,109.4 377.6,107.8 413.8,106.8 450.0,105.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.3 124.4,103.5 160.5,101.6 196.7,93.2 232.9,98.7 269.1,91.4 305.3,89.5 341.5,84.9 377.6,79.6 413.8,71.7 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,93.9 124.4,99.6 160.5,98.9 196.7,92.7 232.9,90.3 269.1,85.5 305.3,75.6 341.5,76.8 377.6,62.6 413.8,53.9 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,91.0 124.4,98.2 160.5,94.3 196.7,85.2 232.9,84.5 269.1,76.2 305.3,69.0 341.5,60.8 377.6,54.7 413.8,44.7 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.1 88.2,89.5 124.4,94.3 160.5,92.3 196.7,84.6 232.9,78.8 269.1,69.1 305.3,64.5 341.5,55.8 377.6,49.5 413.8,39.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4 ns | 3.21 µs | 3.47 µs | 3.8 µs | 1.74 µs |
| D38 | 4.05 ns | 3.46 µs | 4.01 µs | 5.09 µs | 5.85 µs |
| D57 | 197 ns | 291 ns | 300 ns | 307 ns | 434 ns |
| D76 | 200 ns | 318 ns | 309 ns | 430 ns | 464 ns |
| D115 | 346 ns | 476 ns | 654 ns | 760 ns | 713 ns |
| D153 | 392 ns | 497 ns | 614 ns | 851 ns | 1.01 µs |
| D230 | 503 ns | 786 ns | 945 ns | 1.37 µs | 1.85 µs |
| D307 | 838 ns | 1.14 µs | 1.45 µs | 2.17 µs | 2.71 µs |
| D462 | 881 ns | 1.3 µs | 1.16 µs | 3.16 µs | 4.17 µs |
| D616 | 1.16 µs | 1.74 µs | 3.03 µs | 4.06 µs | 5.56 µs |
| D924 | 1.57 µs | 2.28 µs | 4.74 µs | 7.5 µs | 10 µs |
| D1232 | 2.24 µs | 3.25 µs | 7.2 µs | 10.8 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.9 88.2,185.7 124.4,118.2 160.5,118.0 196.7,108.5 232.9,106.3 269.1,101.9 305.3,93.1 341.5,92.2 377.6,87.5 413.8,82.1 450.0,76.0 450.0,30.7 413.8,49.9 377.6,60.2 341.5,65.2 305.3,72.7 269.1,79.3 232.9,89.8 196.7,95.9 160.5,103.3 124.4,104.5 88.2,59.3 52.0,80.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.9 88.2,185.7 124.4,118.2 160.5,118.0 196.7,108.5 232.9,106.3 269.1,101.9 305.3,93.1 341.5,92.2 377.6,87.5 413.8,82.1 450.0,76.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,69.7 88.2,68.4 124.4,111.4 160.5,109.9 196.7,102.9 232.9,102.2 269.1,94.2 305.3,87.7 341.5,85.4 377.6,80.4 413.8,75.7 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.4 88.2,65.9 124.4,110.9 160.5,110.4 196.7,97.4 232.9,98.5 269.1,91.0 305.3,83.5 341.5,87.4 377.6,70.8 413.8,63.0 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.8 88.2,61.7 124.4,110.5 160.5,104.7 196.7,94.8 232.9,92.8 269.1,84.5 305.3,76.5 341.5,70.0 377.6,65.7 413.8,55.0 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.4 88.2,59.3 124.4,104.5 160.5,103.3 196.7,95.9 232.9,89.8 269.1,79.3 305.3,72.7 341.5,65.2 377.6,60.2 413.8,49.9 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 158 ns | 201 ns | 205 ns | 174 ns |
| D38 | 4.36 ns | 201 ns | 187 ns | 203 ns | 206 ns |
| D57 | 309 ns | 394 ns | 407 ns | 425 ns | 529 ns |
| D76 | 308 ns | 413 ns | 418 ns | 549 ns | 597 ns |
| D115 | 580 ns | 679 ns | 881 ns | 1.01 µs | 897 ns |
| D153 | 645 ns | 716 ns | 856 ns | 1.03 µs | 1.26 µs |
| D230 | 877 ns | 1.17 µs | 1.28 µs | 1.77 µs | 2.24 µs |
| D307 | 1.42 µs | 1.64 µs | 1.96 µs | 2.77 µs | 3.33 µs |
| D462 | 1.44 µs | 1.81 µs | 1.53 µs | 3.81 µs | 4.81 µs |
| D616 | 1.91 µs | 2.32 µs | 3.84 µs | 4.81 µs | 6.34 µs |
| D924 | 2.58 µs | 3.02 µs | 5.75 µs | 8.54 µs | 11.3 µs |
| D1232 | 3.38 µs | 4.24 µs | 8.47 µs | 12.3 µs | 32 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,184.4 124.4,110.4 160.5,110.5 196.7,99.5 232.9,97.6 269.1,92.3 305.3,83.9 341.5,83.6 377.6,78.8 413.8,73.5 450.0,68.8 450.0,29.8 413.8,47.9 377.6,57.9 341.5,62.7 305.3,69.1 269.1,76.0 232.9,86.0 196.7,91.9 160.5,99.0 124.4,101.1 88.2,117.5 52.0,120.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,184.4 124.4,110.4 160.5,110.5 196.7,99.5 232.9,97.6 269.1,92.3 305.3,83.9 341.5,83.6 377.6,78.8 413.8,73.5 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,117.9 124.4,106.2 160.5,105.4 196.7,96.7 232.9,95.8 269.1,87.2 305.3,81.4 341.5,79.7 377.6,75.4 413.8,70.8 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.8 88.2,119.2 124.4,105.6 160.5,105.1 196.7,92.2 232.9,92.7 269.1,85.7 305.3,78.3 341.5,82.6 377.6,66.6 413.8,59.6 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.5 88.2,117.7 124.4,104.9 160.5,100.4 196.7,89.8 232.9,89.4 269.1,80.1 305.3,72.3 341.5,66.8 377.6,62.7 413.8,52.7 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.3 88.2,117.5 124.4,101.1 160.5,99.0 196.7,91.9 232.9,86.0 269.1,76.0 305.3,69.1 341.5,62.7 377.6,57.9 413.8,47.9 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body -->
