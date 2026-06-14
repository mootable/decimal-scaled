# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.07 µs | 5.79 µs | 5.15 µs | 8.69 µs | 8.7 µs |
| D38 | 5.07 µs | 5.8 µs | 7.71 µs | 10.9 µs | 9.65 µs |
| D57 | 347 ns | 533 ns | 718 ns | 1.13 µs | 1.14 µs |
| D76 | 299 ns | 790 ns | 1.12 µs | 1.36 µs | 2.08 µs |
| D115 | 342 ns | 2.08 µs | 2.51 µs | 3.53 µs | 5.21 µs |
| D153 | 529 ns | 2.64 µs | 3.62 µs | 5.72 µs | 6.79 µs |
| D230 | 549 ns | 3.49 µs | 8.15 µs | 10.6 µs | 14.1 µs |
| D307 | 430 ns | 5.59 µs | 11.3 µs | 16.8 µs | 22.6 µs |
| D462 | 476 ns | 10.4 µs | 26.5 µs | 33.3 µs | 47.5 µs |
| D616 | 675 ns | 18.8 µs | 40.9 µs | 63.5 µs | 85 µs |
| D924 | 719 ns | 32.1 µs | 89.6 µs | 133 µs | 199 µs |
| D1232 | 859 ns | 48.6 µs | 124 µs | 250 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,124.7 88.2,124.7 124.4,183.0 160.5,186.3 196.7,183.3 232.9,173.8 269.1,173.0 305.3,178.3 341.5,176.1 377.6,168.5 413.8,167.2 450.0,163.3 450.0,33.1 413.8,45.0 377.6,63.5 341.5,76.2 305.3,92.3 269.1,102.5 232.9,118.4 196.7,124.2 160.5,144.1 124.4,157.1 88.2,110.8 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,124.7 88.2,124.7 124.4,183.0 160.5,186.3 196.7,183.3 232.9,173.8 269.1,173.0 305.3,178.3 341.5,176.1 377.6,168.5 413.8,167.2 450.0,163.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,121.8 124.4,173.7 160.5,165.1 196.7,144.1 232.9,138.9 269.1,132.8 305.3,122.6 341.5,109.1 377.6,96.3 413.8,84.6 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,115.6 124.4,167.2 160.5,157.6 196.7,140.0 232.9,132.1 269.1,114.4 305.3,107.4 341.5,88.9 377.6,79.4 413.8,62.4 450.0,55.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,108.1 124.4,157.3 160.5,153.3 196.7,132.6 232.9,122.1 269.1,108.8 305.3,98.8 341.5,83.9 377.6,69.9 413.8,53.8 450.0,40.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,110.8 124.4,157.1 160.5,144.1 196.7,124.2 232.9,118.4 269.1,102.5 305.3,92.3 341.5,76.2 377.6,63.5 413.8,45.0 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 4.79 µs | 6.24 µs | 7.31 µs | 7.88 µs |
| D38 | 1.87 ns | 6.77 µs | 7.57 µs | 9.5 µs | 9.85 µs |
| D57 | 2.81 ns | 3.98 µs | 4.09 µs | 5.4 µs | 8.63 µs |
| D76 | 1.93 ns | 5.55 µs | 7.32 µs | 8.98 µs | 11.9 µs |
| D115 | 17 ns | 6.84 µs | 13 µs | 17.1 µs | 21.2 µs |
| D153 | 22.4 ns | 6.86 µs | 15 µs | 21.4 µs | 33.9 µs |
| D230 | 51.4 ns | 11.9 µs | 21.6 µs | 45.9 µs | 76.9 µs |
| D307 | 115 ns | 16 µs | 33.9 µs | 78 µs | 121 µs |
| D462 | 167 ns | 23.4 µs | 84.9 µs | 153 µs | 228 µs |
| D616 | 141 ns | 38.3 µs | 138 µs | 284 µs | 444 µs |
| D924 | 171 ns | 79.8 µs | 284 µs | 598 µs | 984 µs |
| D1232 | 399 ns | 140 µs | 393 µs | 983 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,202.2 124.4,197.2 160.5,201.9 196.7,174.9 232.9,171.4 269.1,161.1 305.3,151.2 341.5,146.5 377.6,148.6 413.8,146.2 450.0,135.7 450.0,25.6 413.8,38.8 377.6,48.6 341.5,56.9 305.3,64.8 269.1,70.4 232.9,80.6 196.7,86.4 160.5,93.6 124.4,97.5 88.2,95.9 52.0,98.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,202.2 124.4,197.2 160.5,201.9 196.7,174.9 232.9,171.4 269.1,161.1 305.3,151.2 341.5,146.5 377.6,148.6 413.8,146.2 450.0,135.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.8 88.2,100.5 124.4,107.1 160.5,103.0 196.7,100.4 232.9,100.4 269.1,93.5 305.3,89.9 341.5,85.2 377.6,79.0 413.8,69.9 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.6 88.2,99.2 124.4,106.8 160.5,99.6 196.7,92.4 232.9,90.7 269.1,86.2 305.3,80.6 341.5,69.2 377.6,63.2 413.8,54.2 450.0,50.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,96.3 124.4,103.4 160.5,97.1 196.7,89.0 232.9,86.3 269.1,76.8 305.3,70.2 341.5,61.9 377.6,54.2 413.8,44.9 450.0,38.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.7 88.2,95.9 124.4,97.5 160.5,93.6 196.7,86.4 232.9,80.6 269.1,70.4 305.3,64.8 341.5,56.9 377.6,48.6 413.8,38.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 39.7 ns | 42.4 ns | 40.3 ns | 40.1 ns |
| D38 | 15.9 ns | 42.8 ns | 66.9 ns | 75.4 ns | 94.8 ns |
| D57 | 16.6 ns | 42.2 ns | 67.4 ns | 717 ns | 708 ns |
| D76 | 11.1 ns | 68.6 ns | 599 ns | 708 ns | 906 ns |
| D115 | 20.5 ns | 79.2 ns | 700 ns | 1.21 µs | 1.39 µs |
| D153 | 25.2 ns | 696 ns | 1.06 µs | 1.4 µs | 2.11 µs |
| D230 | 29.3 ns | 509 ns | 1.47 µs | 2.38 µs | 3.29 µs |
| D307 | 42.8 ns | 968 ns | 2.14 µs | 3.32 µs | 5.6 µs |
| D462 | 63.4 ns | 1.44 µs | 3.67 µs | 6.27 µs | 9.59 µs |
| D616 | 68.7 ns | 2.32 µs | 6.2 µs | 11.2 µs | 15.4 µs |
| D924 | 100 ns | 3.74 µs | 11.4 µs | 24.8 µs | 28.7 µs |
| D1232 | 110 ns | 6.26 µs | 18.1 µs | 27.2 µs | 50.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,199.9 124.4,199.0 160.5,207.7 196.7,194.4 232.9,190.0 269.1,186.6 305.3,178.4 341.5,169.9 377.6,168.2 413.8,160.0 450.0,158.0 450.0,24.7 413.8,37.1 377.6,50.7 341.5,60.9 305.3,72.6 269.1,84.1 232.9,93.8 196.7,102.9 160.5,112.1 124.4,117.5 88.2,161.2 52.0,179.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,199.9 124.4,199.0 160.5,207.7 196.7,194.4 232.9,190.0 269.1,186.6 305.3,178.4 341.5,169.9 377.6,168.2 413.8,160.0 450.0,158.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,178.4 124.4,178.7 160.5,168.2 196.7,165.1 232.9,117.9 269.1,124.7 305.3,110.7 341.5,102.0 377.6,91.7 413.8,81.3 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.6 88.2,168.7 124.4,168.6 160.5,121.1 196.7,117.8 232.9,108.8 269.1,101.7 305.3,93.5 341.5,81.8 377.6,70.4 413.8,57.2 450.0,47.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,166.1 124.4,117.2 160.5,117.5 196.7,105.8 232.9,102.7 269.1,91.1 305.3,83.9 341.5,70.1 377.6,57.5 413.8,40.2 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,161.2 124.4,117.5 160.5,112.1 196.7,102.9 232.9,93.8 269.1,84.1 305.3,72.6 341.5,60.9 377.6,50.7 413.8,37.1 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.35 ns | 301 ns | 374 ns | 396 ns | 400 ns |
| D38 | 2.18 ns | 391 ns | 369 ns | 401 ns | 372 ns |
| D57 | 267 ns | 467 ns | 438 ns | 439 ns | 555 ns |
| D76 | 165 ns | 446 ns | 472 ns | 555 ns | 654 ns |
| D115 | 289 ns | 487 ns | 565 ns | 904 ns | 953 ns |
| D153 | 293 ns | 468 ns | 589 ns | 982 ns | 1.22 µs |
| D230 | 476 ns | 543 ns | 965 ns | 1.35 µs | 1.61 µs |
| D307 | 733 ns | 771 ns | 1.02 µs | 1.24 µs | 10.2 µs |
| D462 | 1.29 µs | 3.07 µs | 3.63 µs | 3.79 µs | 4.77 µs |
| D616 | 1.35 µs | 1.49 µs | 1.83 µs | 2.85 µs | 3.66 µs |
| D924 | 1.82 µs | 1.8 µs | 2.87 µs | 3.56 µs | 4.66 µs |
| D1232 | 3.03 µs | 3 µs | 3.07 µs | 4.82 µs | 6.46 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,196.5 124.4,112.9 160.5,121.3 196.7,111.6 232.9,111.3 269.1,102.9 305.3,95.4 341.5,85.5 377.6,84.8 413.8,79.6 450.0,70.7 450.0,57.6 413.8,63.3 377.6,67.5 341.5,62.9 305.3,49.6 269.1,81.8 232.9,86.6 196.7,90.8 160.5,97.4 124.4,100.2 88.2,107.2 52.0,105.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,196.5 124.4,112.9 160.5,121.3 196.7,111.6 232.9,111.3 269.1,102.9 305.3,95.4 341.5,85.5 377.6,84.8 413.8,79.6 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,110.9 88.2,106.3 124.4,103.2 160.5,104.0 196.7,102.5 232.9,103.2 269.1,100.6 305.3,94.5 341.5,70.5 377.6,83.1 413.8,79.8 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,107.3 124.4,104.4 160.5,103.0 196.7,99.9 232.9,99.2 269.1,90.6 305.3,89.7 341.5,67.6 377.6,79.5 413.8,71.7 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,105.9 124.4,104.3 160.5,100.2 196.7,91.8 232.9,90.3 269.1,84.8 305.3,86.3 341.5,66.9 377.6,71.8 413.8,67.9 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,107.2 124.4,100.2 160.5,97.4 196.7,90.8 232.9,86.6 269.1,81.8 305.3,49.6 341.5,62.9 377.6,67.5 413.8,63.3 450.0,57.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.71 µs | 9.24 µs | 11.6 µs | 13.6 µs | 14.9 µs |
| D38 | 7.71 µs | 12.5 µs | 14.5 µs | 18.3 µs | 19.7 µs |
| D57 | 4.44 µs | 4.59 µs | 4.36 µs | 4.39 µs | 4.68 µs |
| D76 | 2.19 µs | 4.33 µs | 4.95 µs | 4.59 µs | 5.34 µs |
| D115 | 8.12 µs | 9.09 µs | 8.46 µs | 9.34 µs | 9.62 µs |
| D153 | 7.59 µs | 8.44 µs | 8.86 µs | 9.65 µs | 10.4 µs |
| D230 | 10.4 µs | 10.3 µs | 13.8 µs | 15.8 µs | 16.6 µs |
| D307 | 16.2 µs | 19.1 µs | 21.3 µs | 23.7 µs | 28 µs |
| D462 | 16.7 µs | 21.9 µs | 25.6 µs | 27.2 µs | 31 µs |
| D616 | 27.5 µs | 42.5 µs | 56 µs | 67.4 µs | 78.8 µs |
| D924 | 40.8 µs | 68.1 µs | 104 µs | 133 µs | 163 µs |
| D1232 | 55.3 µs | 113 µs | 140 µs | 221 µs | 270 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.9 88.2,150.9 124.4,166.8 160.5,187.3 196.7,149.4 232.9,151.3 269.1,142.2 305.3,129.3 341.5,128.4 377.6,114.0 413.8,102.7 450.0,93.8 450.0,47.9 413.8,62.6 377.6,83.6 341.5,110.6 305.3,113.5 269.1,128.6 232.9,142.2 196.7,144.4 160.5,161.5 124.4,165.3 88.2,123.7 52.0,131.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,150.9 88.2,150.9 124.4,166.8 160.5,187.3 196.7,149.4 232.9,151.3 269.1,142.2 305.3,129.3 341.5,128.4 377.6,114.0 413.8,102.7 450.0,93.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.6 88.2,137.0 124.4,165.8 160.5,167.5 196.7,146.1 232.9,148.2 269.1,142.4 305.3,124.5 341.5,120.7 377.6,101.4 413.8,87.8 450.0,73.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,139.0 88.2,132.5 124.4,167.4 160.5,163.7 196.7,148.2 232.9,146.8 269.1,134.1 305.3,121.5 341.5,116.1 377.6,93.4 413.8,75.6 450.0,66.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,125.8 124.4,167.2 160.5,165.9 196.7,145.3 232.9,144.4 269.1,130.1 305.3,118.3 341.5,114.4 377.6,88.1 413.8,68.3 450.0,53.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,123.7 124.4,165.3 160.5,161.5 196.7,144.4 232.9,142.2 269.1,128.6 305.3,113.5 341.5,110.6 377.6,83.6 413.8,62.6 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 5.28 µs | 6.88 µs | 8 µs | 8.6 µs |
| D38 | 6.24 ns | 7.45 µs | 8.22 µs | 10.2 µs | 10.6 µs |
| D57 | 64 ns | 4.43 µs | 4.33 µs | 4.49 µs | 4.91 µs |
| D76 | 53.5 ns | 3.9 µs | 4.86 µs | 4.98 µs | 5.89 µs |
| D115 | 149 ns | 8.67 µs | 8.72 µs | 9.98 µs | 10.5 µs |
| D153 | 179 ns | 8.52 µs | 9.58 µs | 10.8 µs | 11.8 µs |
| D230 | 284 ns | 11.1 µs | 14.8 µs | 19.4 µs | 20.6 µs |
| D307 | 434 ns | 20.3 µs | 21.1 µs | 27.9 µs | 34.2 µs |
| D462 | 683 ns | 77.4 µs | 147 µs | 204 µs | 261 µs |
| D616 | 758 ns | 192 µs | 350 µs | 379 µs | 570 µs |
| D924 | 926 ns | 423 µs | 486 µs | 850 µs | 1.74 ms |
| D1232 | 1.46 µs | 827 µs | 773 µs | 2.31 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,187.3 124.4,158.4 160.5,160.6 196.7,147.9 232.9,145.7 269.1,139.9 305.3,134.7 341.5,129.0 377.6,127.7 413.8,125.2 450.0,119.6 450.0,25.0 413.8,31.7 377.6,45.5 341.5,55.2 305.3,80.4 269.1,86.7 232.9,93.6 196.7,95.2 160.5,102.3 124.4,104.5 88.2,95.0 52.0,97.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,187.3 124.4,158.4 160.5,160.6 196.7,147.9 232.9,145.7 269.1,139.9 305.3,134.7 341.5,129.0 377.6,127.7 413.8,125.2 450.0,119.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,99.4 124.4,105.8 160.5,107.4 196.7,97.5 232.9,97.7 269.1,94.5 305.3,86.9 341.5,70.3 377.6,59.0 413.8,49.3 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.4 88.2,98.1 124.4,106.1 160.5,104.7 196.7,97.4 232.9,96.2 269.1,90.8 305.3,86.5 341.5,62.4 377.6,51.6 413.8,47.5 450.0,41.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,95.4 124.4,105.6 160.5,104.4 196.7,95.7 232.9,94.8 269.1,87.5 305.3,83.0 341.5,58.3 377.6,50.6 413.8,40.6 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,95.0 124.4,104.5 160.5,102.3 196.7,95.2 232.9,93.6 269.1,86.7 305.3,80.4 341.5,55.2 377.6,45.5 413.8,31.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.38 ns | 14.4 ns | 19.3 ns | 30.4 ns | 30.7 ns |
| D38 | 7.51 ns | 36.5 ns | 31.1 ns | 1.64 µs | 2.85 µs |
| D57 | 176 ns | 220 ns | 486 ns | 765 ns | 766 ns |
| D76 | 121 ns | 270 ns | 645 ns | 840 ns | 1.16 µs |
| D115 | 124 ns | 600 ns | 1.01 µs | 1.53 µs | 1.7 µs |
| D153 | 117 ns | 1.11 µs | 1.6 µs | 1.89 µs | 2.67 µs |
| D230 | 145 ns | 1.38 µs | 2.46 µs | 3.44 µs | 4.28 µs |
| D307 | 165 ns | 2.29 µs | 3.62 µs | 4.69 µs | 7.14 µs |
| D462 | 195 ns | 3.8 µs | 6.31 µs | 9.5 µs | 12 µs |
| D616 | 224 ns | 6.39 µs | 11.1 µs | 14.3 µs | 20.8 µs |
| D924 | 216 ns | 10.5 µs | 17.3 µs | 27 µs | 35.7 µs |
| D1232 | 289 ns | 16.4 µs | 24.3 µs | 40.7 µs | 62 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.8 88.2,175.0 124.4,120.2 160.5,126.6 196.7,126.3 232.9,127.2 269.1,123.5 305.3,121.3 341.5,118.4 377.6,116.0 413.8,116.7 450.0,111.6 450.0,18.3 413.8,27.9 377.6,37.3 341.5,46.8 305.3,55.9 269.1,64.7 232.9,72.9 196.7,80.8 160.5,87.4 124.4,94.6 88.2,71.8 52.0,150.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.8 88.2,175.0 124.4,120.2 160.5,126.6 196.7,126.3 232.9,127.2 269.1,123.5 305.3,121.3 341.5,118.4 377.6,116.0 413.8,116.7 450.0,111.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.7 88.2,147.5 124.4,116.3 160.5,112.7 196.7,98.9 232.9,88.1 269.1,84.4 305.3,75.6 341.5,66.8 377.6,57.8 413.8,49.2 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.6 88.2,150.3 124.4,102.5 160.5,97.6 196.7,89.8 232.9,81.8 269.1,74.4 305.3,67.6 341.5,58.0 377.6,48.1 413.8,40.5 450.0,34.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.7 88.2,81.4 124.4,94.7 160.5,93.0 196.7,82.6 232.9,78.9 269.1,68.5 305.3,63.2 341.5,50.9 377.6,43.7 413.8,32.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.5 88.2,71.8 124.4,94.6 160.5,87.4 196.7,80.8 232.9,72.9 269.1,64.7 305.3,55.9 341.5,46.8 377.6,37.3 413.8,27.9 450.0,18.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
