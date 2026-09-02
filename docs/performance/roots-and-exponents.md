# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 210 ns | 350 ns | 153 ns | 258 ns | 386 ns |
| D38 | 474 ns | 238 ns | 539 ns | 600 ns | 841 ns |
| D57 | 321 ns | 384 ns | 707 ns | 1.14 µs | 1.19 µs |
| D76 | 467 ns | 773 ns | 1.19 µs | 1.34 µs | 1.89 µs |
| D115 | 241 ns | 2.01 µs | 2.38 µs | 3.5 µs | 5.26 µs |
| D153 | 300 ns | 2.01 µs | 3.57 µs | 5.65 µs | 7.04 µs |
| D230 | 382 ns | 4.31 µs | 7.75 µs | 10.6 µs | 13.8 µs |
| D307 | 302 ns | 5.48 µs | 11.1 µs | 16.4 µs | 22.5 µs |
| D462 | 465 ns | 10.8 µs | 20.9 µs | 33 µs | 47.6 µs |
| D616 | 435 ns | 14.2 µs | 37 µs | 57.3 µs | 76.8 µs |
| D924 | 578 ns | 30 µs | 89.4 µs | 103 µs | 199 µs |
| D1232 | 883 ns | 49 µs | 150 µs | 172 µs | 344 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.9 88.2,176.2 124.4,184.7 160.5,176.5 196.7,190.9 232.9,186.2 269.1,180.9 305.3,186.0 341.5,176.6 377.6,178.1 413.8,171.9 450.0,162.7 450.0,33.2 413.8,45.1 377.6,65.7 341.5,76.1 305.3,92.4 269.1,103.1 232.9,117.6 196.7,124.0 160.5,146.2 124.4,156.2 88.2,163.8 52.0,180.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.9 88.2,176.2 124.4,184.7 160.5,176.5 196.7,190.9 232.9,186.2 269.1,180.9 305.3,186.0 341.5,176.6 377.6,178.1 413.8,171.9 450.0,162.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.8 88.2,191.2 124.4,180.8 160.5,165.6 196.7,144.9 232.9,144.8 269.1,128.3 305.3,123.1 341.5,108.4 377.6,102.4 413.8,86.2 450.0,75.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.7 88.2,173.4 124.4,167.5 160.5,156.3 196.7,141.1 232.9,132.4 269.1,115.5 305.3,107.8 341.5,94.0 377.6,81.6 413.8,62.4 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.5 88.2,171.1 124.4,157.1 160.5,153.6 196.7,132.8 232.9,122.4 269.1,108.8 305.3,99.3 341.5,84.1 377.6,72.1 413.8,59.4 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.7 88.2,163.8 124.4,156.2 160.5,146.2 196.7,124.0 232.9,117.6 269.1,103.1 305.3,92.4 341.5,76.1 377.6,65.7 413.8,45.1 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.52 ns | 1.43 µs | 3.24 µs | 2.94 µs | 3.69 µs |
| D38 | 1.56 ns | 2.35 µs | 3.82 µs | 4.28 µs | 4.73 µs |
| D57 | 2.18 ns | 3.01 µs | 4.43 µs | 5.8 µs | 9.26 µs |
| D76 | 3.12 ns | 5.84 µs | 6.99 µs | 10.2 µs | 11.2 µs |
| D115 | 10.8 ns | 5.29 µs | 13.8 µs | 18 µs | 22.2 µs |
| D153 | 13.6 ns | 4.29 µs | 15.5 µs | 22.4 µs | 38.2 µs |
| D230 | 45.3 ns | 13 µs | 22.1 µs | 45.9 µs | 78.5 µs |
| D307 | 52.3 ns | 15.8 µs | 35.6 µs | 78.8 µs | 123 µs |
| D462 | 236 ns | 24.1 µs | 68.9 µs | 154 µs | 230 µs |
| D616 | 141 ns | 34.6 µs | 132 µs | 267 µs | 409 µs |
| D924 | 205 ns | 86.2 µs | 287 µs | 490 µs | 988 µs |
| D1232 | 382 ns | 139 µs | 443 µs | 724 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,204.5 124.4,200.3 160.5,195.9 196.7,180.4 232.9,177.6 269.1,162.7 305.3,160.9 341.5,142.2 377.6,148.6 413.8,143.9 450.0,136.2 450.0,25.6 413.8,38.7 377.6,49.7 341.5,56.8 305.3,64.6 269.1,70.1 232.9,79.1 196.7,85.8 160.5,94.3 124.4,96.7 88.2,105.0 52.0,108.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,204.5 124.4,200.3 160.5,195.9 196.7,180.4 232.9,177.6 269.1,162.7 305.3,160.9 341.5,142.2 377.6,148.6 413.8,143.9 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.8 88.2,113.7 124.4,110.6 160.5,102.4 196.7,103.6 232.9,106.2 269.1,92.4 305.3,90.0 341.5,84.8 377.6,80.3 413.8,69.0 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,107.7 124.4,105.8 160.5,100.2 196.7,91.7 232.9,90.3 269.1,85.9 305.3,80.0 341.5,71.8 377.6,63.7 413.8,54.0 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,106.2 124.4,102.5 160.5,95.5 196.7,88.4 232.9,85.7 269.1,76.8 305.3,70.1 341.5,61.8 377.6,55.0 413.8,47.4 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,105.0 124.4,96.7 160.5,94.3 196.7,85.8 232.9,79.1 269.1,70.1 305.3,64.6 341.5,56.8 377.6,49.7 413.8,38.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 12.8 ns | 31.5 ns | 41.4 ns | 32.5 ns | 41.5 ns |
| D38 | 16.1 ns | 32.9 ns | 66.4 ns | 66.4 ns | 94.3 ns |
| D57 | 17 ns | 33.8 ns | 67.9 ns | 696 ns | 718 ns |
| D76 | 17.9 ns | 70.2 ns | 687 ns | 631 ns | 1.07 µs |
| D115 | 13.7 ns | 64.6 ns | 622 ns | 1.22 µs | 1.37 µs |
| D153 | 17.2 ns | 331 ns | 1.07 µs | 1.44 µs | 2.03 µs |
| D230 | 28.6 ns | 723 ns | 1.47 µs | 2.31 µs | 3.31 µs |
| D307 | 22.6 ns | 1.08 µs | 2.12 µs | 3.27 µs | 5.58 µs |
| D462 | 63.4 ns | 1.46 µs | 2.82 µs | 6.25 µs | 9.59 µs |
| D616 | 63.6 ns | 2.47 µs | 6.08 µs | 10.9 µs | 16.1 µs |
| D924 | 102 ns | 3.67 µs | 11.4 µs | 20 µs | 28.4 µs |
| D1232 | 121 ns | 6.25 µs | 20.8 µs | 20.9 µs | 50.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.7 88.2,199.7 124.4,198.4 160.5,197.4 196.7,203.1 232.9,198.2 269.1,187.2 305.3,192.3 341.5,169.9 377.6,169.8 413.8,159.5 450.0,155.8 450.0,24.7 413.8,37.3 377.6,49.6 341.5,60.9 305.3,72.7 269.1,84.0 232.9,94.7 196.7,103.1 160.5,108.5 124.4,117.2 88.2,161.3 52.0,179.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.7 88.2,199.7 124.4,198.4 160.5,197.4 196.7,203.1 232.9,198.2 269.1,187.2 305.3,192.3 341.5,169.9 377.6,169.8 413.8,159.5 450.0,155.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,185.1 88.2,184.1 124.4,183.5 160.5,167.7 196.7,169.5 232.9,134.0 269.1,117.1 305.3,108.4 341.5,101.7 377.6,90.4 413.8,81.8 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,168.9 124.4,168.4 160.5,118.1 196.7,120.3 232.9,108.4 269.1,101.7 305.3,93.7 341.5,87.5 377.6,70.8 413.8,57.2 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.4 88.2,168.9 124.4,117.9 160.5,120.0 196.7,105.7 232.9,102.1 269.1,91.8 305.3,84.3 341.5,70.2 377.6,58.0 413.8,45.0 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,161.3 124.4,117.2 160.5,108.5 196.7,103.1 232.9,94.7 269.1,84.0 305.3,72.7 341.5,60.9 377.6,49.6 413.8,37.3 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.34 ns | 234 ns | 377 ns | 322 ns | 393 ns |
| D38 | 2.49 ns | 307 ns | 391 ns | 385 ns | 399 ns |
| D57 | 274 ns | 390 ns | 482 ns | 484 ns | 635 ns |
| D76 | 278 ns | 475 ns | 476 ns | 688 ns | 625 ns |
| D115 | 222 ns | 396 ns | 712 ns | 944 ns | 978 ns |
| D153 | 237 ns | 322 ns | 634 ns | 1 µs | 1.41 µs |
| D230 | 569 ns | 643 ns | 1.02 µs | 1.49 µs | 1.82 µs |
| D307 | 458 ns | 699 ns | 1.14 µs | 1.36 µs | 11.2 µs |
| D462 | 1.36 µs | 3.11 µs | 2.93 µs | 4.01 µs | 4.94 µs |
| D616 | 1.38 µs | 1.47 µs | 1.86 µs | 2.88 µs | 3.66 µs |
| D924 | 2.15 µs | 2.25 µs | 3.2 µs | 3.06 µs | 5.32 µs |
| D1232 | 3.25 µs | 3.2 µs | 4.42 µs | 3.82 µs | 7.22 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.1 88.2,194.1 124.4,112.5 160.5,112.2 196.7,116.1 232.9,115.0 269.1,99.8 305.3,103.6 341.5,84.6 377.6,84.4 413.8,76.7 450.0,69.5 450.0,55.7 413.8,61.0 377.6,67.5 341.5,62.3 305.3,48.1 269.1,79.6 232.9,84.0 196.7,90.4 160.5,98.2 124.4,97.9 88.2,106.0 52.0,106.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.1 88.2,194.1 124.4,112.5 160.5,112.2 196.7,116.1 232.9,115.0 269.1,99.8 305.3,103.6 341.5,84.6 377.6,84.4 413.8,76.7 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.2 88.2,110.5 124.4,106.4 160.5,102.9 196.7,106.1 232.9,109.7 269.1,97.7 305.3,96.2 341.5,70.3 377.6,83.3 413.8,75.9 450.0,69.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,106.3 124.4,102.7 160.5,102.9 196.7,95.9 232.9,97.9 269.1,89.6 305.3,87.8 341.5,71.3 377.6,79.2 413.8,69.8 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,106.6 124.4,102.6 160.5,96.5 196.7,91.0 232.9,90.0 269.1,83.0 305.3,84.7 341.5,65.9 377.6,71.6 413.8,70.6 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,106.0 124.4,97.9 160.5,98.2 196.7,90.4 232.9,84.0 269.1,79.6 305.3,48.1 341.5,62.3 377.6,67.5 413.8,61.0 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.21 µs | 2.53 µs | 5.8 µs | 4.98 µs | 6.92 µs |
| D38 | 2.82 µs | 4.28 µs | 7.21 µs | 8.44 µs | 9.74 µs |
| D57 | 4.07 µs | 3.15 µs | 4.5 µs | 4.52 µs | 4.8 µs |
| D76 | 4.01 µs | 4.29 µs | 4.51 µs | 5.17 µs | 4.85 µs |
| D115 | 5.76 µs | 7.3 µs | 9.43 µs | 9.66 µs | 10.1 µs |
| D153 | 6.53 µs | 5.63 µs | 9.35 µs | 10.2 µs | 11.8 µs |
| D230 | 11.3 µs | 12.6 µs | 14.2 µs | 16.4 µs | 17.4 µs |
| D307 | 11.7 µs | 18.3 µs | 21.7 µs | 24 µs | 28.6 µs |
| D462 | 16.6 µs | 21.7 µs | 20.2 µs | 27.9 µs | 31.3 µs |
| D616 | 23.7 µs | 40.2 µs | 51.9 µs | 61.8 µs | 72.4 µs |
| D924 | 43.4 µs | 74 µs | 104 µs | 111 µs | 164 µs |
| D1232 | 57.3 µs | 115 µs | 170 µs | 160 µs | 273 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.0 88.2,180.0 124.4,169.4 160.5,169.8 196.7,159.3 232.9,155.7 269.1,139.8 305.3,138.9 341.5,128.7 377.6,118.4 413.8,100.8 450.0,92.8 450.0,47.6 413.8,62.3 377.6,86.0 341.5,110.3 305.3,112.9 269.1,127.3 232.9,138.6 196.7,143.1 160.5,164.3 124.4,164.6 88.2,144.1 52.0,154.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.0 88.2,180.0 124.4,169.4 160.5,169.8 196.7,159.3 232.9,155.7 269.1,139.8 305.3,138.9 341.5,128.7 377.6,118.4 413.8,100.8 450.0,92.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,183.2 88.2,167.9 124.4,176.8 160.5,167.8 196.7,152.5 232.9,159.9 269.1,136.7 305.3,125.8 341.5,120.9 377.6,103.1 413.8,85.4 450.0,72.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.1 88.2,152.8 124.4,166.5 160.5,166.4 196.7,145.0 232.9,145.3 269.1,133.1 305.3,120.8 341.5,123.0 377.6,95.7 413.8,75.4 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.5 88.2,148.3 124.4,166.3 160.5,162.5 196.7,144.3 232.9,142.7 269.1,129.0 305.3,117.9 341.5,113.6 377.6,90.6 413.8,73.7 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.0 88.2,144.1 124.4,164.6 160.5,164.3 196.7,143.1 232.9,138.6 269.1,127.3 305.3,112.9 341.5,110.3 377.6,86.0 413.8,62.3 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7 ns | 1.82 µs | 3.89 µs | 3.47 µs | 4.33 µs |
| D38 | 6.54 ns | 2.9 µs | 4.46 µs | 4.92 µs | 5.39 µs |
| D57 | 56.1 ns | 3.41 µs | 4.6 µs | 4.75 µs | 5.14 µs |
| D76 | 75.3 ns | 4.02 µs | 4.76 µs | 5.7 µs | 5.6 µs |
| D115 | 114 ns | 6.93 µs | 9.62 µs | 10.3 µs | 10.8 µs |
| D153 | 144 ns | 5.44 µs | 9.84 µs | 11.1 µs | 13.2 µs |
| D230 | 347 ns | 12.7 µs | 15.2 µs | 20.2 µs | 21.4 µs |
| D307 | 263 ns | 19.5 µs | 21.6 µs | 28.8 µs | 34.7 µs |
| D462 | 702 ns | 77.6 µs | 117 µs | 206 µs | 265 µs |
| D616 | 700 ns | 177 µs | 328 µs | 354 µs | 529 µs |
| D924 | 1.01 µs | 452 µs | 488 µs | 706 µs | 1.75 ms |
| D1232 | 1.49 µs | 830 µs | 860 µs | 1.72 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.9 88.2,186.7 124.4,160.0 160.5,156.4 196.7,151.2 232.9,148.4 269.1,137.4 305.3,140.8 341.5,128.7 377.6,128.7 413.8,124.1 450.0,119.3 450.0,25.0 413.8,31.7 377.6,46.5 341.5,55.1 305.3,80.3 269.1,86.3 232.9,92.3 196.7,94.7 160.5,102.9 124.4,104.0 88.2,103.4 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.9 88.2,186.7 124.4,160.0 160.5,156.4 196.7,151.2 232.9,148.4 269.1,137.4 305.3,140.8 341.5,128.7 377.6,128.7 413.8,124.1 450.0,119.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,111.1 124.4,109.1 160.5,107.0 196.7,100.3 232.9,103.3 269.1,92.7 305.3,87.4 341.5,70.3 377.6,60.1 413.8,48.4 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.4 88.2,105.7 124.4,105.4 160.5,104.9 196.7,96.2 232.9,95.9 269.1,90.5 305.3,86.1 341.5,65.2 377.6,52.4 413.8,47.5 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,104.5 124.4,104.9 160.5,102.7 196.7,95.3 232.9,94.4 269.1,87.0 305.3,82.6 341.5,58.1 377.6,51.4 413.8,42.9 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,103.4 124.4,104.0 160.5,102.9 196.7,94.7 232.9,92.3 269.1,86.3 305.3,80.3 341.5,55.1 377.6,46.5 413.8,31.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.49 ns | 11.8 ns | 18.2 ns | 23.1 ns | 28.8 ns |
| D38 | 7.92 ns | 16.9 ns | 31.3 ns | 416 ns | 604 ns |
| D57 | 158 ns | 206 ns | 488 ns | 775 ns | 767 ns |
| D76 | 192 ns | 270 ns | 771 ns | 721 ns | 1.21 µs |
| D115 | 102 ns | 437 ns | 925 ns | 1.53 µs | 1.68 µs |
| D153 | 113 ns | 561 ns | 1.58 µs | 1.87 µs | 2.6 µs |
| D230 | 158 ns | 1.54 µs | 2.4 µs | 3.66 µs | 4.29 µs |
| D307 | 122 ns | 2.42 µs | 3.71 µs | 4.73 µs | 7.14 µs |
| D462 | 199 ns | 3.69 µs | 4.93 µs | 9.17 µs | 11.9 µs |
| D616 | 234 ns | 5.94 µs | 10.3 µs | 14 µs | 20.2 µs |
| D924 | 235 ns | 11.3 µs | 17.2 µs | 22 µs | 35.6 µs |
| D1232 | 310 ns | 16.7 µs | 28.4 µs | 30 µs | 62.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.9 88.2,174.1 124.4,122.0 160.5,118.6 196.7,129.6 232.9,127.9 269.1,122.0 305.3,126.5 341.5,118.0 377.6,115.3 413.8,115.2 450.0,110.4 450.0,18.2 413.8,28.0 377.6,37.8 341.5,47.0 305.3,55.9 269.1,64.7 232.9,73.4 196.7,81.0 160.5,86.7 124.4,94.6 88.2,98.7 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.9 88.2,174.1 124.4,122.0 160.5,118.6 196.7,129.6 232.9,127.9 269.1,122.0 305.3,126.5 341.5,118.0 377.6,115.3 413.8,115.2 450.0,110.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,167.2 88.2,160.9 124.4,117.5 160.5,112.8 196.7,104.4 232.9,100.0 269.1,82.5 305.3,74.7 341.5,67.3 377.6,59.1 413.8,47.9 450.0,41.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.6 88.2,150.2 124.4,102.4 160.5,94.5 196.7,91.3 232.9,82.0 269.1,74.8 305.3,67.2 341.5,62.3 377.6,49.5 413.8,40.6 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.5 88.2,105.2 124.4,94.4 160.5,95.7 196.7,82.6 232.9,79.1 269.1,67.5 305.3,63.0 341.5,51.5 377.6,44.1 413.8,36.3 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,98.7 124.4,94.6 160.5,86.7 196.7,81.0 232.9,73.4 269.1,64.7 305.3,55.9 341.5,47.0 377.6,37.8 413.8,28.0 450.0,18.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
