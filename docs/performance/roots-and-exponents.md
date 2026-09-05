# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 131 ns | 263 ns | 225 ns | · | · |
| D38 | 79.5 ns | 156 ns | 375 ns | 399 ns | · |
| D57 | 153 ns | 723 ns | 695 ns | 569 ns | · |
| D76 | 199 ns | 486 ns | 639 ns | 1.26 µs | 1.67 µs |
| D115 | 755 ns | 1.31 µs | 2.08 µs | 2.99 µs | · |
| D153 | 253 ns | 687 ns | 1.77 µs | 4.32 µs | · |
| D230 | 1.44 µs | 2.6 µs | 5.47 µs | 9.06 µs | · |
| D307 | 1.79 µs | 3.65 µs | 8 µs | 13.5 µs | · |
| D462 | 466 ns | 3.13 µs | 30.1 µs | · | · |
| D616 | 4.81 µs | 30.6 µs | · | · | · |
| D924 | 539 ns | 9.13 µs | · | · | · |
| D1232 | 688 ns | 75.5 µs | 202 µs | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,165.3 88.2,174.0 124.4,162.7 160.5,158.0 196.7,134.9 232.9,153.9 269.1,123.7 305.3,119.8 341.5,143.3 377.6,102.7 413.8,140.7 450.0,136.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.2 88.2,162.3 124.4,135.6 160.5,142.5 196.7,125.2 232.9,136.5 269.1,113.4 305.3,107.5 341.5,110.2 377.6,70.5 413.8,91.6 450.0,54.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.9 88.2,147.0 124.4,136.3 160.5,137.8 196.7,117.3 232.9,120.0 269.1,100.5 305.3,93.9 341.5,70.8 450.0,37.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,145.9 124.4,139.8 160.5,125.9 196.7,111.0 232.9,104.6 269.1,91.7 305.3,84.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.79 µs | 3.74 µs | 2.99 µs | · | · |
| D38 | 1.47 µs | 3.19 µs | 4.26 µs | 4.72 µs | · |
| D57 | 2.38 µs | 4.57 µs | 4 µs | 5.23 µs | · |
| D76 | 5.05 µs | 6.33 µs | 5.1 µs | 9.22 µs | 12.5 µs |
| D115 | 6.67 µs | 13.1 µs | 16.7 µs | 22.3 µs | · |
| D153 | 5.24 µs | 5.04 µs | 16 µs | 35.2 µs | · |
| D230 | 13.7 µs | 19.8 µs | 46.2 µs | 78.3 µs | · |
| D307 | 15.4 µs | 28.2 µs | 67.4 µs | 116 µs | · |
| D462 | 6.79 µs | 23 µs | 232 µs | · | · |
| D616 | 39 µs | 268 µs | · | · | · |
| D924 | 80.9 µs | · | · | · | · |
| D1232 | 5 µs | 658 µs | 2.68 ms | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,197.3 88.2,201.6 124.4,191.2 160.5,174.8 196.7,168.8 232.9,174.0 269.1,153.1 305.3,150.6 341.5,168.4 377.6,130.5 413.8,114.6 450.0,175.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.4 88.2,184.8 124.4,177.0 160.5,169.9 196.7,154.1 232.9,174.9 269.1,145.1 305.3,137.5 341.5,141.9 377.6,88.6 450.0,69.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.2 88.2,178.5 124.4,179.9 160.5,174.6 196.7,148.8 232.9,149.8 269.1,126.8 305.3,118.6 341.5,91.7 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,176.3 124.4,174.1 160.5,161.8 196.7,142.6 232.9,132.7 269.1,115.3 305.3,106.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 455 ns | 466 ns | · | · | · |
| D38 | 435 ns | 436 ns | 449 ns | 454 ns | · |
| D57 | 182 ns | 512 ns | 351 ns | 320 ns | · |
| D76 | 278 ns | 514 ns | 359 ns | 622 ns | 734 ns |
| D115 | 494 ns | 631 ns | 843 ns | 1.03 µs | · |
| D153 | 303 ns | 399 ns | 1.28 µs | · | · |
| D230 | 716 ns | 985 ns | 1.44 µs | · | · |
| D307 | 701 ns | 931 ns | 1.31 µs | · | · |
| D462 | 1.53 µs | 1.28 µs | 2.78 µs | · | · |
| D616 | 1.56 µs | 2.85 µs | · | · | · |
| D924 | 2.02 µs | · | · | · | · |
| D1232 | 2.26 µs | 3.28 µs | 6.73 µs | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,144.2 88.2,146.2 124.4,183.9 160.5,165.6 196.7,140.7 232.9,161.9 269.1,124.5 305.3,125.4 341.5,91.7 377.6,90.7 413.8,79.4 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,143.1 88.2,146.0 124.4,139.1 160.5,138.9 196.7,130.0 232.9,150.0 269.1,110.6 305.3,113.1 341.5,99.2 377.6,64.6 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,144.8 124.4,155.5 160.5,154.5 196.7,117.4 232.9,99.2 269.1,94.1 305.3,98.2 341.5,65.6 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,144.3 124.4,159.5 160.5,130.6 196.7,108.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 403 ns | 1.47 µs | · | · | · |
| D38 | 380 ns | 1.25 µs | 1.66 µs | 1.86 µs | · |
| D57 | 685 ns | 1.4 µs | 913 ns | · | · |
| D76 | 1.24 µs | 1.45 µs | 883 ns | 1.46 µs | 1.68 µs |
| D115 | 2.41 µs | 2.55 µs | 2.52 µs | 2.87 µs | · |
| D153 | 2.26 µs | 1.96 µs | 3.07 µs | · | · |
| D230 | 3.57 µs | 3.5 µs | 4.48 µs | · | · |
| D307 | 5.35 µs | 5.45 µs | · | · | · |
| D462 | 5.79 µs | 5.93 µs | 8.71 µs | · | · |
| D616 | 10.7 µs | · | · | · | · |
| D1232 | 14 µs | 27.2 µs | · | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="91.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="131.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="171.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="211.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="251.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="290.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="330.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="370.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="410.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,169.7 91.8,171.4 131.6,154.3 171.4,137.1 211.2,117.9 251.0,119.7 290.8,106.5 330.6,94.8 370.4,92.5 410.2,74.6 450.0,67.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,132.1 91.8,136.8 131.6,133.7 171.4,132.6 211.2,116.3 251.0,123.9 290.8,107.0 330.6,94.2 370.4,91.8 450.0,47.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="91.8,128.7 131.6,146.0 171.4,146.9 211.2,116.6 251.0,110.8 290.8,99.9 370.4,80.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="91.8,125.4 171.4,132.5 211.2,112.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 363 ns | 1.44 µs | · | · | · |
| D38 | 340 ns | 1.24 µs | 1.63 µs | 1.83 µs | · |
| D57 | 530 ns | 1.35 µs | 825 ns | 699 ns | · |
| D76 | 942 ns | 1.16 µs | 785 ns | 1.41 µs | 1.5 µs |
| D115 | 2.23 µs | 2.28 µs | 2.16 µs | 2.54 µs | · |
| D153 | 1.78 µs | 1.55 µs | 2.65 µs | · | · |
| D230 | 3 µs | 2.88 µs | 3.72 µs | · | · |
| D307 | 4.46 µs | 3.8 µs | 4.6 µs | · | · |
| D462 | 4.83 µs | 5.04 µs | 7.77 µs | · | · |
| D616 | 8.75 µs | 12.1 µs | · | · | · |
| D924 | 13.7 µs | · | · | · | · |
| D1232 | 10.7 µs | 24.5 µs | · | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,172.7 88.2,174.5 124.4,161.7 160.5,145.1 196.7,120.1 232.9,126.6 269.1,111.5 305.3,100.0 341.5,97.8 377.6,80.5 413.8,67.6 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,132.8 88.2,137.1 124.4,134.7 160.5,139.0 196.7,119.4 232.9,130.7 269.1,112.7 305.3,104.7 341.5,96.5 377.6,71.1 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,129.2 124.4,148.9 160.5,150.3 196.7,121.0 232.9,115.1 269.1,105.3 305.3,99.2 341.5,84.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,125.8 124.4,153.7 160.5,133.5 196.7,116.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 13.7 ns | 29.6 ns | 28.7 ns | · | · |
| D38 | 6.96 ns | 21.6 ns | 135 ns | 194 ns | · |
| D57 | 69.8 ns | 213 ns | 290 ns | 239 ns | · |
| D76 | 82.7 ns | 144 ns | 227 ns | 372 ns | 634 ns |
| D115 | 236 ns | 392 ns | 750 ns | 956 ns | · |
| D153 | 114 ns | 244 ns | 629 ns | 1.54 µs | · |
| D230 | 456 ns | 797 ns | 1.95 µs | 2.58 µs | · |
| D307 | 671 ns | 1.34 µs | 2.52 µs | 4.98 µs | · |
| D462 | 246 ns | 1.03 µs | 6.97 µs | · | · |
| D616 | 1.95 µs | 6.73 µs | · | · | · |
| D924 | 251 ns | 2.72 µs | 6.56 µs | · | · |
| D1232 | 309 ns | 18.7 µs | 44.8 µs | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,164.6 88.2,176.3 124.4,136.2 160.5,133.3 196.7,115.1 232.9,127.8 269.1,103.6 305.3,96.9 341.5,114.3 377.6,78.4 413.8,114.0 450.0,110.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,151.2 88.2,156.6 124.4,116.9 160.5,123.7 196.7,106.3 232.9,114.5 269.1,94.0 305.3,84.9 341.5,89.4 377.6,56.9 413.8,72.6 450.0,39.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,124.8 124.4,111.5 160.5,115.8 196.7,95.0 232.9,98.1 269.1,78.4 305.3,74.0 341.5,56.3 413.8,57.3 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="88.2,118.4 124.4,114.9 160.5,107.2 196.7,90.8 232.9,82.5 269.1,73.5 305.3,62.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
