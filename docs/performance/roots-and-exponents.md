# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 300 ns | 592 ns | 154 ns | 389 ns | 386 ns |
| D38 | 305 ns | 264 ns | 468 ns | 391 ns | 723 ns |
| D57 | 321 ns | 298 ns | 707 ns | 1.14 µs | 729 ns |
| D76 | 386 ns | 768 ns | 631 ns | 1.35 µs | 1.88 µs |
| D115 | 516 ns | 2.3 µs | 2.06 µs | 3.48 µs | 5.15 µs |
| D153 | 357 ns | 2.07 µs | 3.42 µs | 5.6 µs | 5.93 µs |
| D230 | 307 ns | 4.11 µs | 8.13 µs | 10.3 µs | 13.6 µs |
| D307 | 566 ns | 5.64 µs | 11 µs | 16.3 µs | 22.4 µs |
| D462 | 481 ns | 9.92 µs | 23.9 µs | 35.8 µs | 51.7 µs |
| D616 | 316 ns | 14.7 µs | 37.2 µs | 57.6 µs | 48.3 µs |
| D924 | 713 ns | 24.2 µs | 89.4 µs | 102 µs | 198 µs |
| D1232 | 813 ns | 49.2 µs | 138 µs | 227 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.1 88.2,185.8 124.4,184.7 160.5,180.7 196.7,174.4 232.9,182.3 269.1,185.7 305.3,172.3 341.5,175.9 377.6,185.0 413.8,167.3 450.0,164.5 450.0,33.1 413.8,45.1 377.6,75.8 341.5,74.3 305.3,92.5 269.1,103.3 232.9,121.3 196.7,124.4 160.5,146.3 124.4,166.9 88.2,167.0 52.0,180.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.1 88.2,185.8 124.4,184.7 160.5,180.7 196.7,174.4 232.9,182.3 269.1,185.7 305.3,172.3 341.5,175.9 377.6,185.0 413.8,167.3 450.0,164.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.4 88.2,188.9 124.4,186.3 160.5,165.7 196.7,141.9 232.9,144.2 269.1,129.3 305.3,122.4 341.5,110.2 377.6,101.6 413.8,90.8 450.0,75.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.6 88.2,176.5 124.4,167.5 160.5,170.0 196.7,144.3 232.9,133.3 269.1,114.5 305.3,107.9 341.5,91.0 377.6,81.5 413.8,62.4 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.5 88.2,180.4 124.4,157.1 160.5,153.5 196.7,132.9 232.9,122.6 269.1,109.3 305.3,99.4 341.5,82.3 377.6,72.0 413.8,59.6 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.7 88.2,167.0 124.4,166.9 160.5,146.3 196.7,124.4 232.9,121.3 269.1,103.3 305.3,92.5 341.5,74.3 377.6,75.8 413.8,45.1 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 1.65 µs | 3.25 µs | 3.47 µs | 3.69 µs |
| D38 | 1.6 ns | 3.24 µs | 4.21 µs | 2.96 µs | 5.22 µs |
| D57 | 2.19 ns | 2.15 µs | 4.47 µs | 5.78 µs | 7.21 µs |
| D76 | 2.65 ns | 5.8 µs | 4.2 µs | 10.1 µs | 12.5 µs |
| D115 | 11.8 ns | 6.59 µs | 12.3 µs | 18 µs | 22.2 µs |
| D153 | 17.9 ns | 4.32 µs | 16.2 µs | 22.3 µs | 32.7 µs |
| D230 | 35 ns | 13.8 µs | 23.5 µs | 46.1 µs | 78.6 µs |
| D307 | 68.1 ns | 15.6 µs | 35.2 µs | 78.7 µs | 123 µs |
| D462 | 131 ns | 22.5 µs | 80.2 µs | 166 µs | 247 µs |
| D616 | 94.1 ns | 34.7 µs | 131 µs | 268 µs | 223 µs |
| D924 | 171 ns | 51.3 µs | 288 µs | 491 µs | 988 µs |
| D1232 | 358 ns | 140 µs | 410 µs | 915 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,204.2 124.4,200.3 160.5,197.9 196.7,179.3 232.9,174.2 269.1,165.9 305.3,157.6 341.5,149.5 377.6,153.6 413.8,146.2 450.0,137.0 450.0,25.5 413.8,38.7 377.6,57.2 341.5,55.9 305.3,64.5 269.1,70.1 232.9,81.0 196.7,85.8 160.5,93.0 124.4,99.8 88.2,103.8 52.0,108.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,204.2 124.4,200.3 160.5,197.9 196.7,179.3 232.9,174.2 269.1,165.9 305.3,157.6 341.5,149.5 377.6,153.6 413.8,146.2 450.0,137.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.0 88.2,109.7 124.4,114.8 160.5,102.5 196.7,100.9 232.9,106.1 269.1,91.7 305.3,90.2 341.5,85.6 377.6,80.3 413.8,75.4 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,106.4 124.4,105.7 160.5,106.5 196.7,93.2 232.9,89.7 269.1,85.1 305.3,80.1 341.5,69.9 377.6,63.7 413.8,54.0 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,110.8 124.4,102.5 160.5,95.6 196.7,88.4 232.9,85.8 269.1,76.8 305.3,70.1 341.5,60.8 377.6,54.9 413.8,47.4 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,103.8 124.4,99.8 160.5,93.0 196.7,85.8 232.9,81.0 269.1,70.1 305.3,64.5 341.5,55.9 377.6,57.2 413.8,38.7 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.3 ns | 40.8 ns | 41.4 ns | 41.1 ns | 41.5 ns |
| D38 | 15.5 ns | 41.4 ns | 74.4 ns | 55.1 ns | 106 ns |
| D57 | 17.1 ns | 25.6 ns | 67.9 ns | 693 ns | 396 ns |
| D76 | 14.3 ns | 70.2 ns | 323 ns | 606 ns | 923 ns |
| D115 | 21 ns | 72.7 ns | 491 ns | 1.24 µs | 1.38 µs |
| D153 | 23.4 ns | 336 ns | 917 ns | 1.44 µs | 1.79 µs |
| D230 | 22.8 ns | 645 ns | 1.35 µs | 2.25 µs | 3.34 µs |
| D307 | 43.3 ns | 1.08 µs | 2.12 µs | 3.26 µs | 5.65 µs |
| D462 | 63 ns | 1.55 µs | 3.69 µs | 6.44 µs | 9.05 µs |
| D616 | 51.1 ns | 2.44 µs | 6.1 µs | 10.9 µs | 8.06 µs |
| D924 | 91.6 ns | 2.35 µs | 11.3 µs | 19.6 µs | 28.4 µs |
| D1232 | 110 ns | 6.25 µs | 19.4 µs | 27.3 µs | 50.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.1 88.2,200.5 124.4,198.3 160.5,202.2 196.7,193.9 232.9,191.5 269.1,192.1 305.3,178.2 341.5,170.0 377.6,174.6 413.8,161.9 450.0,158.0 450.0,24.7 413.8,37.3 377.6,64.7 341.5,62.2 305.3,72.4 269.1,83.8 232.9,97.4 196.7,102.9 160.5,111.7 124.4,130.1 88.2,158.6 52.0,179.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.1 88.2,200.5 124.4,198.3 160.5,202.2 196.7,193.9 232.9,191.5 269.1,192.1 305.3,178.2 341.5,170.0 377.6,174.6 413.8,161.9 450.0,158.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.5 88.2,179.1 124.4,189.6 160.5,167.7 196.7,166.9 232.9,133.7 269.1,119.5 305.3,108.4 341.5,100.5 377.6,90.6 413.8,91.4 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,166.4 124.4,168.4 160.5,134.5 196.7,125.5 232.9,111.9 269.1,103.4 305.3,93.7 341.5,81.7 377.6,70.7 413.8,57.3 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.3 88.2,172.9 124.4,118.0 160.5,120.9 196.7,105.4 232.9,102.1 269.1,92.4 305.3,84.3 341.5,69.6 377.6,58.1 413.8,45.3 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,158.6 124.4,130.1 160.5,111.7 196.7,102.9 232.9,97.4 269.1,83.8 305.3,72.4 341.5,62.2 377.6,64.7 413.8,37.3 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.95 ns | 278 ns | 377 ns | 383 ns | 386 ns |
| D38 | 2.81 ns | 380 ns | 418 ns | 307 ns | 427 ns |
| D57 | 274 ns | 292 ns | 482 ns | 491 ns | 417 ns |
| D76 | 241 ns | 475 ns | 304 ns | 695 ns | 708 ns |
| D115 | 282 ns | 491 ns | 576 ns | 949 ns | 1 µs |
| D153 | 352 ns | 329 ns | 762 ns | 1.04 µs | 1.19 µs |
| D230 | 466 ns | 713 ns | 1.16 µs | 1.47 µs | 1.85 µs |
| D307 | 643 ns | 684 ns | 1.11 µs | 1.35 µs | 11.2 µs |
| D462 | 1.29 µs | 2.81 µs | 3.33 µs | 4.32 µs | 5.71 µs |
| D616 | 981 ns | 1.47 µs | 1.89 µs | 2.88 µs | 2.03 µs |
| D924 | 1.99 µs | 1.33 µs | 3.16 µs | 3.02 µs | 5.25 µs |
| D1232 | 3.16 µs | 3.25 µs | 4.24 µs | 5.05 µs | 7.16 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.1 88.2,192.0 124.4,112.5 160.5,114.7 196.7,112.0 232.9,108.2 269.1,103.3 305.3,97.7 341.5,85.5 377.6,90.3 413.8,78.0 450.0,70.0 450.0,55.8 413.8,61.2 377.6,77.7 341.5,59.7 305.3,48.0 269.1,79.3 232.9,87.0 196.7,89.9 160.5,96.0 124.4,105.2 88.2,104.8 52.0,106.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.1 88.2,192.0 124.4,112.5 160.5,114.7 196.7,112.0 232.9,108.2 269.1,103.3 305.3,97.7 341.5,85.5 377.6,90.3 413.8,78.0 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.2 88.2,106.8 124.4,111.4 160.5,102.9 196.7,102.4 232.9,109.3 269.1,95.9 305.3,96.6 341.5,72.0 377.6,83.3 413.8,85.1 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,105.2 124.4,102.7 160.5,110.7 196.7,99.6 232.9,94.7 269.1,87.5 305.3,88.2 341.5,69.1 377.6,79.0 413.8,70.0 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,110.5 124.4,102.4 160.5,96.3 196.7,90.9 232.9,89.3 269.1,83.3 305.3,84.8 341.5,64.6 377.6,71.6 413.8,70.8 450.0,61.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,104.8 124.4,105.2 160.5,96.0 196.7,89.9 232.9,87.0 269.1,79.3 305.3,48.0 341.5,59.7 377.6,77.7 413.8,61.2 450.0,55.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.89 µs | 3.16 µs | 5.8 µs | 6.29 µs | 6.92 µs |
| D38 | 2.9 µs | 5.77 µs | 7.37 µs | 6.02 µs | 9.97 µs |
| D57 | 4.06 µs | 2.28 µs | 4.43 µs | 4.53 µs | 3.22 µs |
| D76 | 3.09 µs | 4.26 µs | 2.44 µs | 5.18 µs | 5.36 µs |
| D115 | 7.91 µs | 8.73 µs | 8.18 µs | 9.66 µs | 10 µs |
| D153 | 8.43 µs | 5.82 µs | 9.99 µs | 10.2 µs | 9.96 µs |
| D230 | 9.32 µs | 13.2 µs | 15.2 µs | 16.4 µs | 17.3 µs |
| D307 | 15.7 µs | 18.4 µs | 21.5 µs | 24.1 µs | 28.6 µs |
| D462 | 16.3 µs | 20.5 µs | 23.7 µs | 29.9 µs | 34 µs |
| D616 | 18 µs | 39.8 µs | 52.2 µs | 62 µs | 43.7 µs |
| D924 | 41.2 µs | 44.8 µs | 105 µs | 109 µs | 164 µs |
| D1232 | 57.4 µs | 114 µs | 155 µs | 202 µs | 273 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.3 88.2,179.2 124.4,169.4 160.5,177.3 196.7,150.1 232.9,148.3 269.1,145.4 305.3,130.2 341.5,129.3 377.6,126.3 413.8,102.3 450.0,92.8 450.0,47.6 413.8,62.4 377.6,100.7 341.5,107.9 305.3,112.9 269.1,127.4 232.9,143.5 196.7,143.2 160.5,161.4 124.4,176.1 88.2,143.4 52.0,154.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.3 88.2,179.2 124.4,169.4 160.5,177.3 196.7,150.1 232.9,148.3 269.1,145.4 305.3,130.2 341.5,129.3 377.6,126.3 413.8,102.3 450.0,92.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.7 88.2,159.3 124.4,186.2 160.5,168.0 196.7,147.3 232.9,159.0 269.1,135.4 305.3,125.6 341.5,122.6 377.6,103.3 413.8,99.9 450.0,72.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.1 88.2,152.2 124.4,166.9 160.5,184.2 196.7,149.2 232.9,143.4 269.1,131.2 305.3,121.2 341.5,118.3 377.6,95.5 413.8,75.3 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.7 88.2,158.0 124.4,166.3 160.5,162.4 196.7,144.3 232.9,142.8 269.1,128.9 305.3,117.8 341.5,111.6 377.6,90.5 413.8,74.2 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.0 88.2,143.4 124.4,176.1 160.5,161.4 196.7,143.2 232.9,143.5 269.1,127.4 305.3,112.9 341.5,107.9 377.6,100.7 413.8,62.4 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 2.09 µs | 3.89 µs | 4.09 µs | 4.33 µs |
| D38 | 6.59 ns | 3.9 µs | 4.92 µs | 3.48 µs | 5.95 µs |
| D57 | 56.1 ns | 2.37 µs | 4.55 µs | 4.8 µs | 3.88 µs |
| D76 | 60.5 ns | 4.06 µs | 2.65 µs | 5.63 µs | 6.19 µs |
| D115 | 137 ns | 8.33 µs | 8.15 µs | 10.4 µs | 10.9 µs |
| D153 | 199 ns | 5.5 µs | 10.3 µs | 11.1 µs | 11.2 µs |
| D230 | 241 ns | 13.7 µs | 16.7 µs | 20.1 µs | 21.5 µs |
| D307 | 368 ns | 19.3 µs | 21.7 µs | 28.6 µs | 35 µs |
| D462 | 720 ns | 70.1 µs | 137 µs | 222 µs | 285 µs |
| D616 | 536 ns | 177 µs | 329 µs | 356 µs | 290 µs |
| D924 | 938 ns | 292 µs | 491 µs | 695 µs | 1.75 ms |
| D1232 | 1.47 µs | 831 µs | 795 µs | 2.11 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,186.6 124.4,160.0 160.5,159.1 196.7,148.9 232.9,144.3 269.1,141.9 305.3,136.7 341.5,128.4 377.6,132.0 413.8,125.1 450.0,119.5 450.0,25.0 413.8,31.7 377.6,53.9 341.5,54.2 305.3,80.2 269.1,86.2 232.9,94.3 196.7,94.6 160.5,101.7 124.4,107.5 88.2,102.2 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,186.6 124.4,160.0 160.5,159.1 196.7,148.9 232.9,144.3 269.1,141.9 305.3,136.7 341.5,128.4 377.6,132.0 413.8,125.1 450.0,119.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.2 88.2,107.4 124.4,113.6 160.5,106.9 196.7,98.0 232.9,103.1 269.1,91.8 305.3,87.6 341.5,71.6 377.6,60.1 413.8,53.8 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.4 88.2,104.5 124.4,105.5 160.5,112.2 196.7,98.3 232.9,95.3 269.1,89.3 305.3,86.1 341.5,63.3 377.6,52.4 413.8,47.4 450.0,41.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,108.8 124.4,104.8 160.5,102.9 196.7,95.2 232.9,94.5 269.1,87.0 305.3,82.7 341.5,57.2 377.6,51.4 413.8,43.1 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,102.2 124.4,107.5 160.5,101.7 196.7,94.6 232.9,94.3 269.1,86.2 305.3,80.2 341.5,54.2 377.6,53.9 413.8,31.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.29 ns | 13.5 ns | 18.3 ns | 28.9 ns | 28.7 ns |
| D38 | 8.23 ns | 19.5 ns | 32.7 ns | 244 ns | 489 ns |
| D57 | 158 ns | 160 ns | 490 ns | 771 ns | 491 ns |
| D76 | 158 ns | 271 ns | 375 ns | 721 ns | 1.05 µs |
| D115 | 114 ns | 650 ns | 713 ns | 1.54 µs | 1.68 µs |
| D153 | 126 ns | 574 ns | 1.52 µs | 1.87 µs | 2.27 µs |
| D230 | 122 ns | 1.71 µs | 2.37 µs | 3.42 µs | 4.23 µs |
| D307 | 148 ns | 2.39 µs | 3.65 µs | 4.74 µs | 7.2 µs |
| D462 | 197 ns | 3.6 µs | 5.83 µs | 9.64 µs | 11.9 µs |
| D616 | 169 ns | 6 µs | 10.4 µs | 14.2 µs | 10.6 µs |
| D924 | 217 ns | 9.56 µs | 17.4 µs | 22.1 µs | 35.4 µs |
| D1232 | 290 ns | 16.3 µs | 27.7 µs | 39.3 µs | 62.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.1 88.2,173.4 124.4,122.0 160.5,122.1 196.7,127.7 232.9,126.0 269.1,126.5 305.3,123.2 341.5,118.2 377.6,120.9 413.8,116.6 450.0,111.5 450.0,18.1 413.8,28.0 377.6,49.0 341.5,47.0 305.3,55.7 269.1,65.0 232.9,75.8 196.7,81.0 160.5,89.1 124.4,102.3 88.2,102.4 52.0,151.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.1 88.2,173.4 124.4,122.0 160.5,122.1 196.7,127.7 232.9,126.0 269.1,126.5 305.3,123.2 341.5,118.2 377.6,120.9 413.8,116.6 450.0,111.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.8 88.2,158.4 124.4,121.9 160.5,112.7 196.7,97.5 232.9,99.7 269.1,80.7 305.3,74.9 341.5,67.7 377.6,58.9 413.8,50.8 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.5 88.2,149.4 124.4,102.4 160.5,107.0 196.7,95.9 232.9,82.7 269.1,75.0 305.3,67.5 341.5,59.4 377.6,49.4 413.8,40.4 450.0,32.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,114.5 124.4,94.5 160.5,95.7 196.7,82.5 232.9,79.1 269.1,68.7 305.3,63.0 341.5,50.6 377.6,43.9 413.8,36.2 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,102.4 124.4,102.3 160.5,89.1 196.7,81.0 232.9,75.8 269.1,65.0 305.3,55.7 341.5,47.0 377.6,49.0 413.8,28.0 450.0,18.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
